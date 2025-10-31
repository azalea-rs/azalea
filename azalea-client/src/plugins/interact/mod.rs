pub mod pick;

use std::collections::HashMap;

use azalea_block::BlockState;
use azalea_core::{
    direction::Direction,
    game_type::GameMode,
    hit_result::{BlockHitResult, HitResult},
    position::{BlockPos, Vec3},
    tick::GameTick,
};
use azalea_entity::{
    Attributes, LocalEntity, LookDirection, PlayerAbilities, Position,
    attributes::{
        creative_block_interaction_range_modifier, creative_entity_interaction_range_modifier,
    },
    clamp_look_direction,
    indexing::EntityIdIndex,
};
use azalea_inventory::{ItemStack, ItemStackData, components};
use azalea_physics::{
    PhysicsSystems, collision::entity_collisions::update_last_bounding_box,
    local_player::PhysicsState,
};
use azalea_protocol::packets::game::{
    ServerboundInteract, ServerboundUseItem,
    s_interact::{self, InteractionHand},
    s_swing::ServerboundSwing,
    s_use_item_on::ServerboundUseItemOn,
};
use azalea_world::Instance;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use tracing::warn;

use super::mining::Mining;
use crate::{
    Client,
    attack::handle_attack_event,
    interact::pick::{HitResultComponent, update_hit_result_component},
    inventory::{Inventory, InventorySystems},
    local_player::{LocalGameMode, PermissionLevel},
    movement::MoveEventsSystems,
    packet::game::SendGamePacketEvent,
    respawn::perform_respawn,
};

/// A plugin that allows clients to interact with blocks in the world.
pub struct InteractPlugin;
impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<StartUseItemEvent>()
            .add_message::<EntityInteractEvent>()
            .add_systems(
                Update,
                (
                    (
                        update_attributes_for_held_item,
                        update_attributes_for_gamemode,
                    )
                        .in_set(UpdateAttributesSystems)
                        .chain(),
                    handle_start_use_item_event,
                    update_hit_result_component
                        .after(clamp_look_direction)
                        .after(update_last_bounding_box),
                )
                    .after(InventorySystems)
                    .after(MoveEventsSystems)
                    .after(perform_respawn)
                    .after(handle_attack_event)
                    .chain(),
            )
            .add_systems(
                GameTick,
                (handle_start_use_item_queued, handle_entity_interact)
                    .chain()
                    .before(PhysicsSystems),
            )
            .add_observer(handle_swing_arm_trigger);
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UpdateAttributesSystems;

impl Client {
    /// Right-click a block.
    ///
    /// The behavior of this depends on the target block,
    /// and it'll either place the block you're holding in your hand or use the
    /// block you clicked (like toggling a lever).
    ///
    /// Note that this may trigger anticheats as it doesn't take into account
    /// whether you're actually looking at the block.
    pub fn block_interact(&self, position: BlockPos) {
        self.ecs.lock().write_message(StartUseItemEvent {
            entity: self.entity,
            hand: InteractionHand::MainHand,
            force_block: Some(position),
        });
    }

    /// Right-click an entity.
    ///
    /// This can click through walls, which may trigger anticheats. If that
    /// behavior isn't desired, consider using [`Client::start_use_item`]
    /// instead.
    pub fn entity_interact(&self, entity: Entity) {
        self.ecs.lock().write_message(EntityInteractEvent {
            client: self.entity,
            target: entity,
            location: None,
        });
    }

    /// Right-click the currently held item.
    ///
    /// If the item is consumable, then it'll act as if right-click was held
    /// until the item finishes being consumed. You can use this to eat food.
    ///
    /// If we're looking at a block or entity, then it will be clicked. Also see
    /// [`Client::block_interact`] and [`Client::entity_interact`].
    pub fn start_use_item(&self) {
        self.ecs.lock().write_message(StartUseItemEvent {
            entity: self.entity,
            hand: InteractionHand::MainHand,
            force_block: None,
        });
    }
}

/// A component that contains information about our local block state
/// predictions.
#[derive(Component, Clone, Debug, Default)]
pub struct BlockStatePredictionHandler {
    /// The total number of changes that this client has made to blocks.
    seq: u32,
    server_state: HashMap<BlockPos, ServerVerifiedState>,
}
#[derive(Clone, Debug)]
struct ServerVerifiedState {
    seq: u32,
    block_state: BlockState,
    /// Used for teleporting the player back if we're colliding with the block
    /// that got placed back.
    #[expect(unused)]
    player_pos: Vec3,
}

impl BlockStatePredictionHandler {
    /// Get the next sequence number that we're going to use and increment the
    /// value.
    pub const fn start_predicting(&mut self) -> u32 {
        self.seq += 1;
        self.seq
    }

    /// Should be called right before the client updates a block with its
    /// prediction.
    ///
    /// This is used to make sure that we can rollback to this state if the
    /// server acknowledges the sequence number (with
    /// [`ClientboundBlockChangedAck`]) without having sent a block update.
    ///
    /// [`ClientboundBlockChangedAck`]: azalea_protocol::packets::game::ClientboundBlockChangedAck
    pub fn retain_known_server_state(
        &mut self,
        pos: BlockPos,
        old_state: BlockState,
        player_pos: Vec3,
    ) {
        self.server_state
            .entry(pos)
            .and_modify(|s| s.seq = self.seq)
            .or_insert(ServerVerifiedState {
                seq: self.seq,
                block_state: old_state,
                player_pos,
            });
    }

    /// Save this update as the correct server state so when the server sends a
    /// [`ClientboundBlockChangedAck`] we don't roll back this new update.
    ///
    /// This should be used when we receive a block update from the server.
    ///
    /// [`ClientboundBlockChangedAck`]: azalea_protocol::packets::game::ClientboundBlockChangedAck
    pub fn update_known_server_state(&mut self, pos: BlockPos, state: BlockState) -> bool {
        if let Some(s) = self.server_state.get_mut(&pos) {
            s.block_state = state;
            true
        } else {
            false
        }
    }

    pub fn end_prediction_up_to(&mut self, seq: u32, world: &Instance) {
        let mut to_remove = Vec::new();
        for (pos, state) in &self.server_state {
            if state.seq > seq {
                continue;
            }
            to_remove.push(*pos);

            // syncBlockState
            let client_block_state = world.get_block_state(*pos).unwrap_or_default();
            let server_block_state = state.block_state;
            if client_block_state == server_block_state {
                continue;
            }
            world.set_block_state(*pos, server_block_state);
            // TODO: implement these two functions
            // if is_colliding(player, *pos, server_block_state) {
            //     abs_snap_to(state.player_pos);
            // }
        }

        for pos in to_remove {
            self.server_state.remove(&pos);
        }
    }
}

/// An event that makes one of our clients simulate a right-click.
///
/// This event just inserts the [`StartUseItemQueued`] component on the given
/// entity.
#[doc(alias("right click"))]
#[derive(Message)]
pub struct StartUseItemEvent {
    pub entity: Entity,
    pub hand: InteractionHand,
    /// See [`StartUseItemQueued::force_block`].
    pub force_block: Option<BlockPos>,
}
pub fn handle_start_use_item_event(
    mut commands: Commands,
    mut events: MessageReader<StartUseItemEvent>,
) {
    for event in events.read() {
        commands.entity(event.entity).insert(StartUseItemQueued {
            hand: event.hand,
            force_block: event.force_block,
        });
    }
}

/// A component that makes our client simulate a right-click on the next
/// [`GameTick`]. It's removed after that tick.
///
/// You may find it more convenient to use [`StartUseItemEvent`] instead, which
/// just inserts this component for you.
///
/// [`GameTick`]: azalea_core::tick::GameTick
#[derive(Component, Debug)]
pub struct StartUseItemQueued {
    pub hand: InteractionHand,
    /// Optionally force us to send a [`ServerboundUseItemOn`] on the given
    /// block.
    ///
    /// This is useful if you want to interact with a block without looking at
    /// it, but should be avoided to stay compatible with anticheats.
    pub force_block: Option<BlockPos>,
}
#[expect(clippy::type_complexity)]
pub fn handle_start_use_item_queued(
    mut commands: Commands,
    query: Query<(
        Entity,
        &StartUseItemQueued,
        &mut BlockStatePredictionHandler,
        &HitResultComponent,
        &LookDirection,
        Option<&Mining>,
    )>,
    mut entity_interact: MessageWriter<EntityInteractEvent>,
) {
    for (entity, start_use_item, mut prediction_handler, hit_result, look_direction, mining) in
        query
    {
        commands.entity(entity).remove::<StartUseItemQueued>();

        if mining.is_some() {
            warn!("Got a StartUseItemEvent for a client that was mining");
        }

        // TODO: this also skips if LocalPlayer.handsBusy is true, which is used when
        // rowing a boat

        let mut hit_result = (**hit_result).clone();

        if let Some(force_block) = start_use_item.force_block {
            let hit_result_matches = if let HitResult::Block(block_hit_result) = &hit_result {
                block_hit_result.block_pos == force_block
            } else {
                false
            };

            if !hit_result_matches {
                // we're not looking at the block, so make up some numbers
                hit_result = HitResult::Block(BlockHitResult {
                    location: force_block.center(),
                    direction: Direction::Up,
                    block_pos: force_block,
                    inside: false,
                    world_border: false,
                    miss: false,
                });
            }
        }

        match &hit_result {
            HitResult::Block(r) => {
                let seq = prediction_handler.start_predicting();
                if r.miss {
                    commands.trigger(SendGamePacketEvent::new(
                        entity,
                        ServerboundUseItem {
                            hand: start_use_item.hand,
                            seq,
                            x_rot: look_direction.x_rot(),
                            y_rot: look_direction.y_rot(),
                        },
                    ));
                } else {
                    commands.trigger(SendGamePacketEvent::new(
                        entity,
                        ServerboundUseItemOn {
                            hand: start_use_item.hand,
                            block_hit: r.into(),
                            seq,
                        },
                    ));
                    // TODO: depending on the result of useItemOn, this might
                    // also need to send a SwingArmEvent.
                    // basically, this TODO is for simulating block
                    // interactions/placements on the client-side.
                }
            }
            HitResult::Entity(r) => {
                entity_interact.write(EntityInteractEvent {
                    client: entity,
                    target: r.entity,
                    location: Some(r.location),
                });
            }
        }
    }
}

/// An ECS `Message` that makes the client tell the server that we right-clicked
/// an entity.
#[derive(Message)]
pub struct EntityInteractEvent {
    pub client: Entity,
    pub target: Entity,
    /// The position on the entity that we'll tell the server that we clicked
    /// on.
    ///
    /// This doesn't matter for most entities. If it's set to `None` but we're
    /// looking at the target, it'll use the correct value. If it's `None` and
    /// we're not looking at the entity, then it'll arbitrary send the target's
    /// exact position.
    pub location: Option<Vec3>,
}

pub fn handle_entity_interact(
    mut events: MessageReader<EntityInteractEvent>,
    mut commands: Commands,
    client_query: Query<(&PhysicsState, &EntityIdIndex, &HitResultComponent)>,
    target_query: Query<&Position>,
) {
    for event in events.read() {
        let Some((physics_state, entity_id_index, hit_result)) =
            client_query.get(event.target).ok()
        else {
            warn!(
                "tried to interact with an entity but the client didn't have the required components"
            );
            continue;
        };

        // TODO: worldborder check

        let Some(entity_id) = entity_id_index.get_by_ecs_entity(event.target) else {
            warn!("tried to interact with an entity that isn't known by the client");
            continue;
        };

        let location = if let Some(l) = event.location {
            l
        } else {
            // if we're looking at the entity, use that
            if let Some(entity_hit_result) = hit_result.as_entity_hit_result()
                && entity_hit_result.entity == event.target
            {
                entity_hit_result.location
            } else {
                // if we're not looking at the entity, make up a value that's good enough by
                // using the entity's position
                let Ok(target_position) = target_query.get(event.target) else {
                    warn!("tried to look at an entity without the entity having a position");
                    continue;
                };
                **target_position
            }
        };

        let mut interact = ServerboundInteract {
            entity_id,
            action: s_interact::ActionType::InteractAt {
                location,
                hand: InteractionHand::MainHand,
            },
            using_secondary_action: physics_state.trying_to_crouch,
        };
        commands.trigger(SendGamePacketEvent::new(event.client, interact.clone()));
        // TODO: this is true if the interaction failed, which i think can only happen
        // in certain cases when interacting with armor stands
        let consumes_action = false;
        if !consumes_action {
            // but yes, most of the time vanilla really does send two interact packets like
            // this
            interact.action = s_interact::ActionType::Interact {
                hand: InteractionHand::MainHand,
            };
            commands.trigger(SendGamePacketEvent::new(event.client, interact));
        }
    }
}

/// Whether we can't interact with the block, based on your gamemode.
///
/// If this is false, then we can interact with the block.
///
/// Passing the inventory, block position, and instance is necessary for the
/// adventure mode check.
#[must_use]
pub fn check_is_interaction_restricted(
    instance: &Instance,
    block_pos: BlockPos,
    game_mode: &GameMode,
    inventory: &Inventory,
) -> bool {
    match game_mode {
        GameMode::Adventure => {
            // vanilla checks for abilities.mayBuild here but servers have no
            // way of modifying that

            let held_item = inventory.held_item();
            match &held_item {
                ItemStack::Present(item) => {
                    let block = instance.chunks.get_block_state(block_pos);
                    let Some(block) = block else {
                        // block isn't loaded so just say that it is restricted
                        return true;
                    };
                    check_block_can_be_broken_by_item_in_adventure_mode(item, &block)
                }
                _ => true,
            }
        }
        GameMode::Spectator => true,
        _ => false,
    }
}

/// Check if the item has the `CanDestroy` tag for the block.
#[must_use]
pub fn check_block_can_be_broken_by_item_in_adventure_mode(
    item: &ItemStackData,
    _block: &BlockState,
) -> bool {
    // minecraft caches the last checked block but that's kind of an unnecessary
    // optimization and makes the code too complicated

    if item.get_component::<components::CanBreak>().is_none() {
        // no CanDestroy tag
        return false;
    }

    false

    // for block_predicate in can_destroy {
    //     // TODO
    //     // defined in BlockPredicateArgument.java
    // }

    // true
}

#[must_use]
pub fn can_use_game_master_blocks(
    abilities: &PlayerAbilities,
    permission_level: &PermissionLevel,
) -> bool {
    abilities.instant_break && **permission_level >= 2
}

/// Swing your arm.
///
/// This is purely a visual effect and won't interact with anything in the
/// world.
#[derive(EntityEvent, Clone, Debug)]
pub struct SwingArmEvent {
    pub entity: Entity,
}
pub fn handle_swing_arm_trigger(swing_arm: On<SwingArmEvent>, mut commands: Commands) {
    commands.trigger(SendGamePacketEvent::new(
        swing_arm.entity,
        ServerboundSwing {
            hand: InteractionHand::MainHand,
        },
    ));
}

#[expect(clippy::type_complexity)]
fn update_attributes_for_held_item(
    mut query: Query<(&mut Attributes, &Inventory), (With<LocalEntity>, Changed<Inventory>)>,
) {
    for (mut attributes, inventory) in &mut query {
        let held_item = inventory.held_item();

        use azalea_registry::Item;
        let added_attack_speed = match held_item.kind() {
            Item::WoodenSword => -2.4,
            Item::WoodenShovel => -3.0,
            Item::WoodenPickaxe => -2.8,
            Item::WoodenAxe => -3.2,
            Item::WoodenHoe => -3.0,

            Item::StoneSword => -2.4,
            Item::StoneShovel => -3.0,
            Item::StonePickaxe => -2.8,
            Item::StoneAxe => -3.2,
            Item::StoneHoe => -2.0,

            Item::GoldenSword => -2.4,
            Item::GoldenShovel => -3.0,
            Item::GoldenPickaxe => -2.8,
            Item::GoldenAxe => -3.0,
            Item::GoldenHoe => -3.0,

            Item::IronSword => -2.4,
            Item::IronShovel => -3.0,
            Item::IronPickaxe => -2.8,
            Item::IronAxe => -3.1,
            Item::IronHoe => -1.0,

            Item::DiamondSword => -2.4,
            Item::DiamondShovel => -3.0,
            Item::DiamondPickaxe => -2.8,
            Item::DiamondAxe => -3.0,
            Item::DiamondHoe => 0.0,

            Item::NetheriteSword => -2.4,
            Item::NetheriteShovel => -3.0,
            Item::NetheritePickaxe => -2.8,
            Item::NetheriteAxe => -3.0,
            Item::NetheriteHoe => 0.0,

            Item::Trident => -2.9,
            _ => 0.,
        };
        attributes
            .attack_speed
            .insert(azalea_entity::attributes::base_attack_speed_modifier(
                added_attack_speed,
            ));
    }
}

#[expect(clippy::type_complexity)]
fn update_attributes_for_gamemode(
    query: Query<(&mut Attributes, &LocalGameMode), (With<LocalEntity>, Changed<LocalGameMode>)>,
) {
    for (mut attributes, game_mode) in query {
        if game_mode.current == GameMode::Creative {
            attributes
                .block_interaction_range
                .insert(creative_block_interaction_range_modifier());
            attributes
                .entity_interaction_range
                .insert(creative_entity_interaction_range_modifier());
        } else {
            attributes
                .block_interaction_range
                .remove(&creative_block_interaction_range_modifier().id);
            attributes
                .entity_interaction_range
                .remove(&creative_entity_interaction_range_modifier().id);
        }
    }
}
