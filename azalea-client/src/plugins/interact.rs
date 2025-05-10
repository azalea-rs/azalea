use azalea_block::BlockState;
use azalea_core::{
    direction::Direction,
    game_type::GameMode,
    hit_result::{BlockHitResult, HitResult},
    position::{BlockPos, Vec3},
    tick::GameTick,
};
use azalea_entity::{
    Attributes, EyeHeight, LocalEntity, LookDirection, Position, clamp_look_direction, view_vector,
};
use azalea_inventory::{ItemStack, ItemStackData, components};
use azalea_physics::{
    PhysicsSet,
    clip::{BlockShapeType, ClipContext, FluidPickType},
};
use azalea_protocol::packets::game::{
    ServerboundUseItem, s_interact::InteractionHand, s_swing::ServerboundSwing,
    s_use_item_on::ServerboundUseItemOn,
};
use azalea_world::{Instance, InstanceContainer, InstanceName};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};
use tracing::warn;

use super::mining::{Mining, MiningSet};
use crate::{
    Client,
    attack::handle_attack_event,
    inventory::{Inventory, InventorySet},
    local_player::{LocalGameMode, PermissionLevel, PlayerAbilities},
    movement::MoveEventsSet,
    packet::game::SendPacketEvent,
    respawn::perform_respawn,
};

/// A plugin that allows clients to interact with blocks in the world.
pub struct InteractPlugin;
impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartUseItemEvent>()
            .add_event::<SwingArmEvent>()
            .add_systems(
                Update,
                (
                    (
                        handle_start_use_item_event,
                        update_hit_result_component.after(clamp_look_direction),
                        handle_swing_arm_event,
                    )
                        .after(InventorySet)
                        .after(perform_respawn)
                        .after(handle_attack_event)
                        .chain(),
                    update_modifiers_for_held_item
                        .after(InventorySet)
                        .after(MoveEventsSet),
                ),
            )
            .add_systems(
                GameTick,
                handle_start_use_item_queued
                    .after(MiningSet)
                    .before(PhysicsSet),
            )
            .add_observer(handle_swing_arm_trigger);
    }
}

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
        self.ecs.lock().send_event(StartUseItemEvent {
            entity: self.entity,
            hand: InteractionHand::MainHand,
            force_block: Some(position),
        });
    }

    /// Right-click the currently held item.
    ///
    /// If the item is consumable, then it'll act as if right-click was held
    /// until the item finishes being consumed. You can use this to eat food.
    ///
    /// If we're looking at a block or entity, then it will be clicked. Also see
    /// [`Client::block_interact`].
    pub fn start_use_item(&self) {
        self.ecs.lock().send_event(StartUseItemEvent {
            entity: self.entity,
            hand: InteractionHand::MainHand,
            force_block: None,
        });
    }
}

/// A component that contains the number of changes this client has made to
/// blocks.
#[derive(Component, Copy, Clone, Debug, Default, Deref)]
pub struct CurrentSequenceNumber(u32);

impl CurrentSequenceNumber {
    /// Get the next sequence number that we're going to use and increment the
    /// value.
    pub fn get_and_increment(&mut self) -> u32 {
        let cur = self.0;
        self.0 += 1;
        cur
    }
}

/// A component that contains the block or entity that the player is currently
/// looking at.
#[doc(alias("looking at", "looking at block", "crosshair"))]
#[derive(Component, Clone, Debug, Deref, DerefMut)]
pub struct HitResultComponent(HitResult);

/// An event that makes one of our clients simulate a right-click.
///
/// This event just inserts the [`StartUseItemQueued`] component on the given
/// entity.
#[doc(alias("right click"))]
#[derive(Event)]
pub struct StartUseItemEvent {
    pub entity: Entity,
    pub hand: InteractionHand,
    /// See [`QueuedStartUseItem::force_block`].
    pub force_block: Option<BlockPos>,
}
pub fn handle_start_use_item_event(
    mut commands: Commands,
    mut events: EventReader<StartUseItemEvent>,
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
#[derive(Component)]
pub struct StartUseItemQueued {
    pub hand: InteractionHand,
    /// Optionally force us to send a [`ServerboundUseItemOn`] on the given
    /// block.
    ///
    /// This is useful if you want to interact with a block without looking at
    /// it, but should be avoided to stay compatible with anticheats.
    pub force_block: Option<BlockPos>,
}
#[allow(clippy::type_complexity)]
pub fn handle_start_use_item_queued(
    mut commands: Commands,
    query: Query<(
        Entity,
        &StartUseItemQueued,
        &mut CurrentSequenceNumber,
        &HitResultComponent,
        &LookDirection,
        Option<&Mining>,
    )>,
) {
    for (entity, start_use_item, mut sequence_number, hit_result, look_direction, mining) in query {
        commands.entity(entity).remove::<StartUseItemQueued>();

        if mining.is_some() {
            warn!("Got a StartUseItemEvent for a client that was mining");
        }

        // TODO: this also skips if LocalPlayer.handsBusy is true, which is used when
        // rowing a boat

        let mut hit_result = hit_result.0.clone();

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
            HitResult::Block(block_hit_result) => {
                if block_hit_result.miss {
                    commands.trigger(SendPacketEvent::new(
                        entity,
                        ServerboundUseItem {
                            hand: start_use_item.hand,
                            sequence: sequence_number.get_and_increment(),
                            x_rot: look_direction.x_rot,
                            y_rot: look_direction.y_rot,
                        },
                    ));
                } else {
                    commands.trigger(SendPacketEvent::new(
                        entity,
                        ServerboundUseItemOn {
                            hand: start_use_item.hand,
                            block_hit: block_hit_result.into(),
                            sequence: sequence_number.get_and_increment(),
                        },
                    ));
                    // TODO: depending on the result of useItemOn, this might
                    // also need to send a SwingArmEvent.
                    // basically, this TODO is for
                    // simulating block interactions/placements on the
                    // client-side.
                }
            }
            HitResult::Entity => {
                // TODO: implement HitResult::Entity

                // TODO: worldborder check

                // commands.trigger(SendPacketEvent::new(
                //     entity,
                //     ServerboundInteract {
                //         entity_id: todo!(),
                //         action: todo!(),
                //         using_secondary_action: todo!(),
                //     },
                // ));
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn update_hit_result_component(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        Option<&mut HitResultComponent>,
        &LocalGameMode,
        &Position,
        &EyeHeight,
        &LookDirection,
        &InstanceName,
    )>,
    instance_container: Res<InstanceContainer>,
) {
    for (entity, hit_result_ref, game_mode, position, eye_height, look_direction, world_name) in
        &mut query
    {
        let pick_range = if game_mode.current == GameMode::Creative {
            6.
        } else {
            4.5
        };
        let eye_position = Vec3 {
            x: position.x,
            y: position.y + **eye_height as f64,
            z: position.z,
        };

        let Some(instance_lock) = instance_container.get(world_name) else {
            continue;
        };
        let instance = instance_lock.read();

        let hit_result = pick(look_direction, &eye_position, &instance.chunks, pick_range);
        if let Some(mut hit_result_ref) = hit_result_ref {
            **hit_result_ref = hit_result;
        } else {
            commands
                .entity(entity)
                .insert(HitResultComponent(hit_result));
        }
    }
}

/// Get the block or entity that a player would be looking at if their eyes were
/// at the given direction and position.
///
/// If you need to get the block/entity the player is looking at right now, use
/// [`HitResultComponent`].
///
/// Also see [`pick_block`].
///
/// TODO: does not currently check for entities
pub fn pick(
    look_direction: &LookDirection,
    eye_position: &Vec3,
    chunks: &azalea_world::ChunkStorage,
    pick_range: f64,
) -> HitResult {
    // TODO
    // let entity_hit_result = ;

    HitResult::Block(pick_block(look_direction, eye_position, chunks, pick_range))
}

/// Get the block that a player would be looking at if their eyes were at the
/// given direction and position.
///
/// Also see [`pick`].
pub fn pick_block(
    look_direction: &LookDirection,
    eye_position: &Vec3,
    chunks: &azalea_world::ChunkStorage,
    pick_range: f64,
) -> BlockHitResult {
    let view_vector = view_vector(look_direction);
    let end_position = eye_position + &(view_vector * pick_range);

    azalea_physics::clip::clip(
        chunks,
        ClipContext {
            from: *eye_position,
            to: end_position,
            block_shape_type: BlockShapeType::Outline,
            fluid_pick_type: FluidPickType::None,
        },
    )
}

/// Whether we can't interact with the block, based on your gamemode. If
/// this is false, then we can interact with the block.
///
/// Passing the inventory, block position, and instance is necessary for the
/// adventure mode check.
pub fn check_is_interaction_restricted(
    instance: &Instance,
    block_pos: &BlockPos,
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
pub fn check_block_can_be_broken_by_item_in_adventure_mode(
    item: &ItemStackData,
    _block: &BlockState,
) -> bool {
    // minecraft caches the last checked block but that's kind of an unnecessary
    // optimization and makes the code too complicated

    if !item.components.has::<components::CanBreak>() {
        // no CanDestroy tag
        return false;
    };

    false

    // for block_predicate in can_destroy {
    //     // TODO
    //     // defined in BlockPredicateArgument.java
    // }

    // true
}

pub fn can_use_game_master_blocks(
    abilities: &PlayerAbilities,
    permission_level: &PermissionLevel,
) -> bool {
    abilities.instant_break && **permission_level >= 2
}

/// Swing your arm. This is purely a visual effect and won't interact with
/// anything in the world.
#[derive(Event, Clone, Debug)]
pub struct SwingArmEvent {
    pub entity: Entity,
}
pub fn handle_swing_arm_trigger(trigger: Trigger<SwingArmEvent>, mut commands: Commands) {
    commands.trigger(SendPacketEvent::new(
        trigger.event().entity,
        ServerboundSwing {
            hand: InteractionHand::MainHand,
        },
    ));
}
pub fn handle_swing_arm_event(mut events: EventReader<SwingArmEvent>, mut commands: Commands) {
    for event in events.read() {
        commands.trigger(event.clone());
    }
}

#[allow(clippy::type_complexity)]
fn update_modifiers_for_held_item(
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
