use azalea_block::{BlockState, BlockTrait, fluid_state::FluidState};
use azalea_core::{direction::Direction, game_type::GameMode, position::BlockPos, tick::GameTick};
use azalea_entity::{FluidOnEyes, Physics, PlayerAbilities, Position, mining::get_mine_progress};
use azalea_inventory::ItemStack;
use azalea_physics::{PhysicsSet, collision::BlockWithShape};
use azalea_protocol::packets::game::s_player_action::{self, ServerboundPlayerAction};
use azalea_world::{InstanceContainer, InstanceName};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};
use tracing::{debug, trace};

use crate::{
    Client,
    interact::{
        BlockStatePredictionHandler, SwingArmEvent, can_use_game_master_blocks,
        check_is_interaction_restricted, pick::HitResultComponent,
    },
    inventory::{Inventory, InventorySet},
    local_player::{InstanceHolder, LocalGameMode, PermissionLevel},
    movement::MoveEventsSet,
    packet::game::SendGamePacketEvent,
};

/// A plugin that allows clients to break blocks in the world.
pub struct MiningPlugin;
impl Plugin for MiningPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<StartMiningBlockEvent>()
            .add_message::<StopMiningBlockEvent>()
            .add_message::<MineBlockProgressEvent>()
            .add_message::<AttackBlockEvent>()
            .add_systems(
                GameTick,
                (
                    update_mining_component,
                    handle_auto_mine,
                    handle_mining_queued,
                    continue_mining_block,
                )
                    .chain()
                    .before(PhysicsSet)
                    .before(super::movement::send_position)
                    .before(super::interact::handle_start_use_item_queued)
                    .in_set(MiningSet),
            )
            .add_systems(
                Update,
                (
                    handle_start_mining_block_event,
                    handle_stop_mining_block_event,
                )
                    .chain()
                    .in_set(MiningSet)
                    .after(InventorySet)
                    .after(MoveEventsSet)
                    .after(azalea_entity::update_fluid_on_eyes)
                    .after(crate::interact::pick::update_hit_result_component)
                    .after(crate::attack::handle_attack_event),
            )
            .add_observer(handle_finish_mining_block_observer);
    }
}

/// The Bevy system set for things related to mining.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MiningSet;

impl Client {
    pub fn start_mining(&self, position: BlockPos) {
        let mut ecs = self.ecs.lock();

        ecs.write_message(StartMiningBlockEvent {
            entity: self.entity,
            position,
        });
    }

    /// When enabled, the bot will mine any block that it is looking at if it is
    /// reachable.
    pub fn left_click_mine(&self, enabled: bool) {
        let mut ecs = self.ecs.lock();
        let mut entity_mut = ecs.entity_mut(self.entity);

        if enabled {
            entity_mut.insert(LeftClickMine);
        } else {
            entity_mut.remove::<LeftClickMine>();
        }
    }
}

/// A component that simulates the client holding down left click to mine the
/// block that it's facing, but this only interacts with blocks and not
/// entities.
#[derive(Component)]
pub struct LeftClickMine;

#[allow(clippy::type_complexity)]
fn handle_auto_mine(
    mut query: Query<
        (
            &HitResultComponent,
            Entity,
            Option<&Mining>,
            &Inventory,
            &MineBlockPos,
            &MineItem,
        ),
        With<LeftClickMine>,
    >,
    mut start_mining_block_event: MessageWriter<StartMiningBlockEvent>,
    mut stop_mining_block_event: MessageWriter<StopMiningBlockEvent>,
) {
    for (
        hit_result_component,
        entity,
        mining,
        inventory,
        current_mining_pos,
        current_mining_item,
    ) in &mut query.iter_mut()
    {
        let block_pos = hit_result_component
            .as_block_hit_result_if_not_miss()
            .map(|b| b.block_pos);

        // start mining if we're looking at a block and we're not already mining it
        if let Some(block_pos) = block_pos
            && (mining.is_none()
                || !is_same_mining_target(
                    block_pos,
                    inventory,
                    current_mining_pos,
                    current_mining_item,
                ))
        {
            start_mining_block_event.write(StartMiningBlockEvent {
                entity,
                position: block_pos,
            });
        } else if mining.is_some() && hit_result_component.miss() {
            stop_mining_block_event.write(StopMiningBlockEvent { entity });
        }
    }
}

/// Information about the block we're currently mining. This is only present if
/// we're currently mining a block.
#[derive(Component, Debug, Clone)]
pub struct Mining {
    pub pos: BlockPos,
    pub dir: Direction,
    /// See [`MiningQueued::force`].
    pub force: bool,
}

/// Start mining the block at the given position.
///
/// If we're looking at the block then the correct direction will be used,
/// otherwise it'll be [`Direction::Down`].
#[derive(Message, Debug)]
pub struct StartMiningBlockEvent {
    pub entity: Entity,
    pub position: BlockPos,
}
fn handle_start_mining_block_event(
    mut commands: Commands,
    mut events: MessageReader<StartMiningBlockEvent>,
    mut query: Query<&HitResultComponent>,
) {
    for event in events.read() {
        trace!("{event:?}");
        let hit_result = query.get_mut(event.entity).unwrap();
        let (direction, force) = if let Some(block_hit_result) =
            hit_result.as_block_hit_result_if_not_miss()
            && block_hit_result.block_pos == event.position
        {
            // we're looking at the block
            (block_hit_result.direction, false)
        } else {
            debug!(
                "Got StartMiningBlockEvent but we're not looking at the block ({:?}.block_pos != {:?}). Picking an arbitrary direction instead.",
                hit_result, event.position
            );
            // we're not looking at the block, arbitrary direction
            (Direction::Down, true)
        };
        commands.entity(event.entity).insert(MiningQueued {
            position: event.position,
            direction,
            force,
        });
    }
}

/// Present on entities when they're going to start mining a block next tick.
#[derive(Component, Debug, Clone)]
pub struct MiningQueued {
    pub position: BlockPos,
    pub direction: Direction,
    /// Whether we should mine the block regardless of whether it's reachable.
    pub force: bool,
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn handle_mining_queued(
    mut commands: Commands,
    mut attack_block_events: MessageWriter<AttackBlockEvent>,
    mut mine_block_progress_events: MessageWriter<MineBlockProgressEvent>,
    query: Query<(
        Entity,
        &MiningQueued,
        &InstanceHolder,
        &LocalGameMode,
        &Inventory,
        &FluidOnEyes,
        &Physics,
        Option<&Mining>,
        &mut BlockStatePredictionHandler,
        &mut MineDelay,
        &mut MineProgress,
        &mut MineTicks,
        &mut MineItem,
        &mut MineBlockPos,
    )>,
) {
    for (
        entity,
        mining_queued,
        instance_holder,
        game_mode,
        inventory,
        fluid_on_eyes,
        physics,
        mining,
        mut sequence_number,
        mut mine_delay,
        mut mine_progress,
        mut mine_ticks,
        mut current_mining_item,
        mut current_mining_pos,
    ) in query
    {
        commands.entity(entity).remove::<MiningQueued>();

        let instance = instance_holder.instance.read();
        if check_is_interaction_restricted(
            &instance,
            mining_queued.position,
            &game_mode.current,
            inventory,
        ) {
            continue;
        }
        // TODO (when world border is implemented): vanilla ignores if the block
        // is outside of the worldborder

        if game_mode.current == GameMode::Creative {
            // In creative mode, first send START_DESTROY_BLOCK packet then immediately
            // finish mining
            commands.trigger(SendGamePacketEvent::new(
                entity,
                ServerboundPlayerAction {
                    action: s_player_action::Action::StartDestroyBlock,
                    pos: mining_queued.position,
                    direction: mining_queued.direction,
                    seq: sequence_number.start_predicting(),
                },
            ));
            commands.trigger(FinishMiningBlockEvent {
                entity,
                position: mining_queued.position,
            });
            **mine_delay = 5;
            commands.trigger(SwingArmEvent { entity });
        } else if mining.is_none()
            || !is_same_mining_target(
                mining_queued.position,
                inventory,
                &current_mining_pos,
                &current_mining_item,
            )
        {
            if mining.is_some() {
                // send a packet to stop mining since we just changed target
                commands.trigger(SendGamePacketEvent::new(
                    entity,
                    ServerboundPlayerAction {
                        action: s_player_action::Action::AbortDestroyBlock,
                        pos: current_mining_pos
                            .expect("IsMining is true so MineBlockPos must be present"),
                        direction: mining_queued.direction,
                        seq: 0,
                    },
                ));
            }

            let target_block_state = instance
                .get_block_state(mining_queued.position)
                .unwrap_or_default();

            // we can't break blocks if they don't have a bounding box
            let block_is_solid = !target_block_state.outline_shape().is_empty();

            if block_is_solid && **mine_progress == 0. {
                // interact with the block (like note block left click) here
                attack_block_events.write(AttackBlockEvent {
                    entity,
                    position: mining_queued.position,
                });
            }

            let block = Box::<dyn BlockTrait>::from(target_block_state);

            let held_item = inventory.held_item();

            if block_is_solid
                && get_mine_progress(
                    block.as_ref(),
                    held_item.kind(),
                    &inventory.inventory_menu,
                    fluid_on_eyes,
                    physics,
                ) >= 1.
            {
                // block was broken instantly (instamined)
                commands.trigger(FinishMiningBlockEvent {
                    entity,
                    position: mining_queued.position,
                });
            } else {
                let mining = Mining {
                    pos: mining_queued.position,
                    dir: mining_queued.direction,
                    force: mining_queued.force,
                };
                trace!("inserting mining component {mining:?} for entity {entity:?}");
                commands.entity(entity).insert(mining);
                **current_mining_pos = Some(mining_queued.position);
                **current_mining_item = held_item;
                **mine_progress = 0.;
                **mine_ticks = 0.;
                mine_block_progress_events.write(MineBlockProgressEvent {
                    entity,
                    position: mining_queued.position,
                    destroy_stage: mine_progress.destroy_stage(),
                });
            }

            commands.trigger(SendGamePacketEvent::new(
                entity,
                ServerboundPlayerAction {
                    action: s_player_action::Action::StartDestroyBlock,
                    pos: mining_queued.position,
                    direction: mining_queued.direction,
                    seq: sequence_number.start_predicting(),
                },
            ));
            commands.trigger(SwingArmEvent { entity });
            // another swing packet gets sent in the same tick in
            // continue_mining_block, vanilla does this too
        }
    }
}

#[derive(Message)]
pub struct MineBlockProgressEvent {
    pub entity: Entity,
    pub position: BlockPos,
    pub destroy_stage: Option<u32>,
}

/// A player left clicked on a block, used for stuff like interacting with note
/// blocks.
#[derive(Message)]
pub struct AttackBlockEvent {
    pub entity: Entity,
    pub position: BlockPos,
}

/// Returns whether the block and item are still the same as when we started
/// mining.
fn is_same_mining_target(
    target_block: BlockPos,
    inventory: &Inventory,
    current_mining_pos: &MineBlockPos,
    current_mining_item: &MineItem,
) -> bool {
    let held_item = inventory.held_item();
    Some(target_block) == current_mining_pos.0 && held_item == current_mining_item.0
}

/// A component bundle for players that can mine blocks.
#[derive(Bundle, Default, Clone)]
pub struct MineBundle {
    pub delay: MineDelay,
    pub progress: MineProgress,
    pub ticks: MineTicks,
    pub mining_pos: MineBlockPos,
    pub mine_item: MineItem,
}

/// A component that counts down until we start mining the next block.
#[derive(Component, Debug, Default, Deref, DerefMut, Clone)]
pub struct MineDelay(pub u32);

/// A component that stores the progress of the current mining operation. This
/// is a value between 0 and 1.
#[derive(Component, Debug, Default, Deref, DerefMut, Clone)]
pub struct MineProgress(pub f32);

impl MineProgress {
    pub fn destroy_stage(&self) -> Option<u32> {
        if self.0 > 0. {
            Some((self.0 * 10.) as u32)
        } else {
            None
        }
    }
}

/// A component that stores the number of ticks that we've been mining the same
/// block for. This is a float even though it should only ever be a round
/// number.
#[derive(Component, Clone, Debug, Default, Deref, DerefMut)]
pub struct MineTicks(pub f32);

/// A component that stores the position of the block we're currently mining.
#[derive(Component, Clone, Debug, Default, Deref, DerefMut)]
pub struct MineBlockPos(pub Option<BlockPos>);

/// A component that contains the item we're currently using to mine. If we're
/// not mining anything, it'll be [`ItemStack::Empty`].
#[derive(Component, Clone, Debug, Default, Deref, DerefMut)]
pub struct MineItem(pub ItemStack);

/// A trigger that's sent when we completed mining a block.
#[derive(EntityEvent)]
pub struct FinishMiningBlockEvent {
    pub entity: Entity,
    pub position: BlockPos,
}

pub fn handle_finish_mining_block_observer(
    finish_mining_block: On<FinishMiningBlockEvent>,
    mut query: Query<(
        &InstanceName,
        &LocalGameMode,
        &Inventory,
        &PlayerAbilities,
        &PermissionLevel,
        &Position,
        &mut BlockStatePredictionHandler,
    )>,
    instances: Res<InstanceContainer>,
) {
    let event = finish_mining_block.event();

    let (
        instance_name,
        game_mode,
        inventory,
        abilities,
        permission_level,
        player_pos,
        mut prediction_handler,
    ) = query.get_mut(finish_mining_block.entity).unwrap();
    let instance_lock = instances.get(instance_name).unwrap();
    let instance = instance_lock.read();
    if check_is_interaction_restricted(&instance, event.position, &game_mode.current, inventory) {
        return;
    }

    if game_mode.current == GameMode::Creative {
        let held_item = inventory.held_item().kind();
        if matches!(
            held_item,
            azalea_registry::Item::Trident | azalea_registry::Item::DebugStick
        ) || azalea_registry::tags::items::SWORDS.contains(&held_item)
        {
            return;
        }
    }

    let Some(block_state) = instance.get_block_state(event.position) else {
        return;
    };

    let registry_block: azalea_registry::Block =
        Box::<dyn BlockTrait>::from(block_state).as_registry_block();
    if !can_use_game_master_blocks(abilities, permission_level)
        && matches!(
            registry_block,
            azalea_registry::Block::CommandBlock | azalea_registry::Block::StructureBlock
        )
    {
        return;
    }
    if block_state == BlockState::AIR {
        return;
    }

    // when we break a waterlogged block we want to keep the water there
    let fluid_state = FluidState::from(block_state);
    let block_state_for_fluid = BlockState::from(fluid_state);
    let old_state = instance
        .set_block_state(event.position, block_state_for_fluid)
        .unwrap_or_default();
    prediction_handler.retain_known_server_state(event.position, old_state, **player_pos);
}

/// Abort mining a block.
#[derive(Message)]
pub struct StopMiningBlockEvent {
    pub entity: Entity,
}
pub fn handle_stop_mining_block_event(
    mut events: MessageReader<StopMiningBlockEvent>,
    mut commands: Commands,
    mut mine_block_progress_events: MessageWriter<MineBlockProgressEvent>,
    mut query: Query<(&MineBlockPos, &mut MineProgress)>,
) {
    for event in events.read() {
        let (mine_block_pos, mut mine_progress) = query.get_mut(event.entity).unwrap();

        let mine_block_pos =
            mine_block_pos.expect("IsMining is true so MineBlockPos must be present");
        commands.trigger(SendGamePacketEvent::new(
            event.entity,
            ServerboundPlayerAction {
                action: s_player_action::Action::AbortDestroyBlock,
                pos: mine_block_pos,
                direction: Direction::Down,
                seq: 0,
            },
        ));
        commands.entity(event.entity).remove::<Mining>();
        **mine_progress = 0.;
        mine_block_progress_events.write(MineBlockProgressEvent {
            entity: event.entity,
            position: mine_block_pos,
            destroy_stage: None,
        });
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn continue_mining_block(
    mut query: Query<(
        Entity,
        &InstanceName,
        &LocalGameMode,
        &Inventory,
        &MineBlockPos,
        &MineItem,
        &FluidOnEyes,
        &Physics,
        &Mining,
        &mut MineDelay,
        &mut MineProgress,
        &mut MineTicks,
        &mut BlockStatePredictionHandler,
    )>,
    mut commands: Commands,
    mut mine_block_progress_events: MessageWriter<MineBlockProgressEvent>,
    instances: Res<InstanceContainer>,
) {
    for (
        entity,
        instance_name,
        game_mode,
        inventory,
        current_mining_pos,
        current_mining_item,
        fluid_on_eyes,
        physics,
        mining,
        mut mine_delay,
        mut mine_progress,
        mut mine_ticks,
        mut prediction_handler,
    ) in query.iter_mut()
    {
        if **mine_delay > 0 {
            **mine_delay -= 1;
            continue;
        }

        if game_mode.current == GameMode::Creative {
            // TODO: worldborder check
            **mine_delay = 5;
            commands.trigger(SendGamePacketEvent::new(
                entity,
                ServerboundPlayerAction {
                    action: s_player_action::Action::StartDestroyBlock,
                    pos: mining.pos,
                    direction: mining.dir,
                    seq: prediction_handler.start_predicting(),
                },
            ));
            commands.trigger(FinishMiningBlockEvent {
                entity,
                position: mining.pos,
            });
            commands.trigger(SwingArmEvent { entity });
        } else if mining.force
            || is_same_mining_target(
                mining.pos,
                inventory,
                current_mining_pos,
                current_mining_item,
            )
        {
            trace!("continue mining block at {:?}", mining.pos);
            let instance_lock = instances.get(instance_name).unwrap();
            let instance = instance_lock.read();
            let target_block_state = instance.get_block_state(mining.pos).unwrap_or_default();

            trace!("target_block_state: {target_block_state:?}");

            if target_block_state.is_air() {
                commands.entity(entity).remove::<Mining>();
                continue;
            }
            let block = Box::<dyn BlockTrait>::from(target_block_state);
            **mine_progress += get_mine_progress(
                block.as_ref(),
                current_mining_item.kind(),
                &inventory.inventory_menu,
                fluid_on_eyes,
                physics,
            );

            if **mine_ticks % 4. == 0. {
                // vanilla makes a mining sound here
            }
            **mine_ticks += 1.;

            if **mine_progress >= 1. {
                // MiningQueued is removed in case we were doing an infinite loop that
                // repeatedly inserts MiningQueued
                commands.entity(entity).remove::<(Mining, MiningQueued)>();
                trace!("finished mining block at {:?}", mining.pos);
                commands.trigger(FinishMiningBlockEvent {
                    entity,
                    position: mining.pos,
                });
                commands.trigger(SendGamePacketEvent::new(
                    entity,
                    ServerboundPlayerAction {
                        action: s_player_action::Action::StopDestroyBlock,
                        pos: mining.pos,
                        direction: mining.dir,
                        seq: prediction_handler.start_predicting(),
                    },
                ));
                **mine_progress = 0.;
                **mine_ticks = 0.;
                **mine_delay = 5;
            }

            mine_block_progress_events.write(MineBlockProgressEvent {
                entity,
                position: mining.pos,
                destroy_stage: mine_progress.destroy_stage(),
            });
            commands.trigger(SwingArmEvent { entity });
        } else {
            trace!("switching mining target to {:?}", mining.pos);
            commands.entity(entity).insert(MiningQueued {
                position: mining.pos,
                direction: mining.dir,
                force: false,
            });
        }
    }
}

pub fn update_mining_component(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Mining, &HitResultComponent)>,
) {
    for (entity, mut mining, hit_result_component) in &mut query.iter_mut() {
        if let Some(block_hit_result) = hit_result_component.as_block_hit_result_if_not_miss() {
            if mining.force && block_hit_result.block_pos != mining.pos {
                continue;
            }

            mining.pos = block_hit_result.block_pos;
            mining.dir = block_hit_result.direction;
        } else {
            if mining.force {
                continue;
            }

            debug!("Removing mining component because we're no longer looking at the block");
            commands.entity(entity).remove::<Mining>();
        }
    }
}
