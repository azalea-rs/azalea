use azalea_block::{Block, BlockState, FluidState};
use azalea_core::{BlockPos, Direction, GameMode};
use azalea_entity::{mining::get_mine_progress, FluidOnEyes, Physics};
use azalea_inventory::ItemSlot;
use azalea_protocol::packets::game::serverbound_player_action_packet::{
    self, ServerboundPlayerActionPacket,
};
use azalea_world::{InstanceContainer, InstanceName};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};

use crate::{
    client::{PermissionLevel, PlayerAbilities},
    interact::{
        can_use_game_master_blocks, check_is_interaction_restricted, CurrentSequenceNumber,
    },
    inventory::InventoryComponent,
    local_player::{LocalGameMode, SendPacketEvent},
    Client,
};

/// A plugin that allows clients to break blocks in the world.
pub struct MinePlugin;
impl Plugin for MinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartMiningBlockEvent>().add_systems(
            Update,
            (
                handle_start_mining_block_event,
                handle_continue_mining_block_event,
                handle_finish_mining_block_event,
                handle_stop_mining_block_event,
            )
                .chain(),
        );
    }
}

impl Client {
    /// Start mining a block.
    pub fn start_mining_block(&self, position: BlockPos, direction: Direction) {
        self.ecs.lock().send_event(StartMiningBlockEvent {
            entity: self.entity,
            position,
            direction,
        });
    }
}

#[derive(Event)]
pub struct StartMiningBlockEvent {
    pub entity: Entity,
    pub position: BlockPos,
    pub direction: Direction,
}

fn handle_start_mining_block_event(
    mut events: EventReader<StartMiningBlockEvent>,
    mut finish_mining_events: EventWriter<FinishMiningBlockEvent>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
    mut attack_block_events: EventWriter<AttackBlockEvent>,
    mut mine_block_progress_events: EventWriter<MineBlockProgressEvent>,
    mut query: Query<(
        &InstanceName,
        &LocalGameMode,
        &InventoryComponent,
        &mut CurrentSequenceNumber,
        &mut MineDelay,
        &mut IsMining,
        &mut MineProgress,
        &mut MineTicks,
        &mut MineBlockPos,
        &mut MineItem,
        &FluidOnEyes,
        &Physics,
    )>,
    instances: Res<InstanceContainer>,
) {
    for event in events.iter() {
        let (
            instance_name,
            game_mode,
            inventory,
            mut sequence_number,
            mut mine_delay,
            mut is_mining,
            mut mine_progress,
            mut mine_ticks,
            mut current_mining_pos,
            mut current_mining_item,
            fluid_on_eyes,
            physics,
        ) = query.get_mut(event.entity).unwrap();
        let instance_lock = instances.get(instance_name).unwrap();
        let instance = instance_lock.read();
        if check_is_interaction_restricted(
            &instance,
            &event.position,
            &game_mode.current,
            inventory,
        ) {
            continue;
        }
        // TODO (when world border is implemented): vanilla ignores if the block
        // is outside of the worldborder

        if game_mode.current == GameMode::Creative {
            *sequence_number += 1;
            finish_mining_events.send(FinishMiningBlockEvent {
                entity: event.entity,
                position: event.position,
            });
            **mine_delay = 5;
        } else if !**is_mining
            || !is_same_mining_target(
                event.position,
                inventory,
                &current_mining_pos,
                &current_mining_item,
            )
        {
            if **is_mining {
                // send a packet to stop mining since we just changed target
                send_packet_events.send(SendPacketEvent {
                    entity: event.entity,
                    packet: ServerboundPlayerActionPacket {
                        action: serverbound_player_action_packet::Action::AbortDestroyBlock,
                        pos: current_mining_pos
                            .expect("IsMining is true so MineBlockPos must be present"),
                        direction: event.direction,
                        sequence: 0,
                    }
                    .get(),
                });
            }

            let target_block_state = instance
                .get_block_state(&event.position)
                .unwrap_or_default();
            *sequence_number += 1;
            let block_is_solid = !target_block_state.is_air();
            if block_is_solid && **mine_progress == 0. {
                // interact with the block (like note block left click) here
                attack_block_events.send(AttackBlockEvent {
                    entity: event.entity,
                    position: event.position,
                });
            }

            let block = Box::<dyn Block>::from(target_block_state);

            let held_item = inventory.held_item();

            if block_is_solid
                && get_mine_progress(
                    &block,
                    held_item.kind(),
                    &inventory.inventory_menu,
                    fluid_on_eyes,
                    physics,
                ) >= 1.
            {
                // block was broken instantly
                finish_mining_events.send(FinishMiningBlockEvent {
                    entity: event.entity,
                    position: event.position,
                });
            } else {
                **is_mining = true;
                **current_mining_pos = Some(event.position);
                **current_mining_item = held_item;
                **mine_progress = 0.;
                **mine_ticks = 0.;
                mine_block_progress_events.send(MineBlockProgressEvent {
                    entity: event.entity,
                    position: event.position,
                    destroy_stage: mine_progress.destroy_stage(),
                })
            }

            send_packet_events.send(SendPacketEvent {
                entity: event.entity,
                packet: ServerboundPlayerActionPacket {
                    action: serverbound_player_action_packet::Action::StartDestroyBlock,
                    pos: current_mining_pos
                        .expect("IsMining is true so MineBlockPos must be present"),
                    direction: event.direction,
                    sequence: **sequence_number,
                }
                .get(),
            });
        }
    }
}

#[derive(Event)]
pub struct MineBlockProgressEvent {
    pub entity: Entity,
    pub position: BlockPos,
    pub destroy_stage: Option<u32>,
}

/// A player left clicked on a block, used for stuff like interacting with note
/// blocks.
#[derive(Event)]
pub struct AttackBlockEvent {
    pub entity: Entity,
    pub position: BlockPos,
}

/// Returns whether the block and item are still the same as when we started
/// mining.
fn is_same_mining_target(
    target_block: BlockPos,
    inventory: &InventoryComponent,
    current_mining_pos: &MineBlockPos,
    current_mining_item: &MineItem,
) -> bool {
    let held_item = inventory.held_item();
    Some(target_block) == current_mining_pos.0 && held_item == current_mining_item.0
}

/// A component bundle for players that can mine blocks.
#[derive(Bundle, Default)]
pub struct MineBundle {
    pub delay: MineDelay,
    pub is_mining: IsMining,
    pub progress: MineProgress,
    pub ticks: MineTicks,
    pub mining_pos: MineBlockPos,
    pub mine_item: MineItem,
}

/// A component that counts down until we start mining the next block.
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct MineDelay(pub u32);

/// A component that stores whether the player is currently in the process of
/// mining a block.
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct IsMining(pub bool);

/// A component that stores the progress of the current mining operation. This
/// is a value between 0 and 1.
#[derive(Component, Debug, Default, Deref, DerefMut)]
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
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct MineTicks(pub f32);

/// A component that stores the position of the block we're currently mining.
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct MineBlockPos(pub Option<BlockPos>);

/// A component that contains the item we're currently using to mine. If we're
/// not mining anything, it'll be [`ItemSlot::Empty`].
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct MineItem(pub ItemSlot);

/// Sent when we completed mining a block.
#[derive(Event)]
pub struct FinishMiningBlockEvent {
    pub entity: Entity,
    pub position: BlockPos,
}

fn handle_finish_mining_block_event(
    mut events: EventReader<FinishMiningBlockEvent>,
    mut query: Query<(
        &InstanceName,
        &LocalGameMode,
        &InventoryComponent,
        &PlayerAbilities,
        &PermissionLevel,
        &mut CurrentSequenceNumber,
    )>,
    instances: Res<InstanceContainer>,
) {
    for event in events.iter() {
        let (instance_name, game_mode, inventory, abilities, permission_level, _sequence_number) =
            query.get_mut(event.entity).unwrap();
        let instance_lock = instances.get(instance_name).unwrap();
        let instance = instance_lock.read();
        if check_is_interaction_restricted(
            &instance,
            &event.position,
            &game_mode.current,
            inventory,
        ) {
            continue;
        }

        if game_mode.current == GameMode::Creative {
            let held_item = inventory.held_item().kind();
            if matches!(
                held_item,
                azalea_registry::Item::Trident | azalea_registry::Item::DebugStick
            ) || azalea_registry::tags::items::SWORDS.contains(&held_item)
            {
                continue;
            }
        }

        let Some(block_state) = instance.get_block_state(&event.position) else {
            continue;
        };

        let registry_block = Box::<dyn Block>::from(block_state).as_registry_block();
        if !can_use_game_master_blocks(abilities, permission_level)
            && matches!(
                registry_block,
                azalea_registry::Block::CommandBlock | azalea_registry::Block::StructureBlock
            )
        {
            continue;
        }
        if block_state == BlockState::AIR {
            continue;
        }

        // when we break a waterlogged block we want to keep the water there
        let fluid_state = FluidState::from(block_state);
        let block_state_for_fluid = BlockState::from(fluid_state);
        instance.set_block_state(&event.position, block_state_for_fluid);
    }
}

/// Abort mining a block.
#[derive(Event)]
pub struct StopMiningBlockEvent {
    pub entity: Entity,
}
fn handle_stop_mining_block_event(
    mut events: EventReader<FinishMiningBlockEvent>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
    mut mine_block_progress_events: EventWriter<MineBlockProgressEvent>,
    mut query: Query<(&mut IsMining, &MineBlockPos, &mut MineProgress)>,
) {
    for event in events.iter() {
        let (mut is_mining, mine_block_pos, mut mine_progress) =
            query.get_mut(event.entity).unwrap();

        if !**is_mining {
            continue;
        }
        let mine_block_pos =
            mine_block_pos.expect("IsMining is true so MineBlockPos must be present");
        send_packet_events.send(SendPacketEvent {
            entity: event.entity,
            packet: ServerboundPlayerActionPacket {
                action: serverbound_player_action_packet::Action::AbortDestroyBlock,
                pos: mine_block_pos,
                direction: Direction::Down,
                sequence: 0,
            }
            .get(),
        });
        **is_mining = false;
        **mine_progress = 0.;
        mine_block_progress_events.send(MineBlockProgressEvent {
            entity: event.entity,
            position: mine_block_pos,
            destroy_stage: None,
        });
    }
}

/// This should be sent every tick that we're mining a block.
#[derive(Event)]
pub struct ContinueMiningBlockEvent {
    pub entity: Entity,
    pub position: BlockPos,
    pub direction: Direction,
}
fn handle_continue_mining_block_event(
    mut events: EventReader<ContinueMiningBlockEvent>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
    mut mine_block_progress_events: EventWriter<MineBlockProgressEvent>,
    mut finish_mining_events: EventWriter<FinishMiningBlockEvent>,
    mut start_mining_events: EventWriter<StartMiningBlockEvent>,
    mut query: Query<(
        &InstanceName,
        &LocalGameMode,
        &InventoryComponent,
        &MineBlockPos,
        &MineItem,
        &FluidOnEyes,
        &Physics,
        &mut MineDelay,
        &mut MineProgress,
        &mut MineTicks,
        &mut CurrentSequenceNumber,
        &mut IsMining,
    )>,
    instances: Res<InstanceContainer>,
) {
    for event in events.iter() {
        let (
            instance_name,
            game_mode,
            inventory,
            current_mining_pos,
            current_mining_item,
            fluid_on_eyes,
            physics,
            mut mine_delay,
            mut mine_progress,
            mut mine_ticks,
            mut sequence_number,
            mut is_mining,
        ) = query.get_mut(event.entity).unwrap();

        if **mine_delay > 0 {
            **mine_delay -= 1;
            continue;
        }

        if game_mode.current == GameMode::Creative {
            // TODO: worldborder check
            **mine_delay = 5;
            finish_mining_events.send(FinishMiningBlockEvent {
                entity: event.entity,
                position: event.position,
            });
            *sequence_number += 1;
            send_packet_events.send(SendPacketEvent {
                entity: event.entity,
                packet: ServerboundPlayerActionPacket {
                    action: serverbound_player_action_packet::Action::StartDestroyBlock,
                    pos: event.position,
                    direction: event.direction,
                    sequence: **sequence_number,
                }
                .get(),
            });
        } else if is_same_mining_target(
            event.position,
            inventory,
            current_mining_pos,
            current_mining_item,
        ) {
            let instance_lock = instances.get(instance_name).unwrap();
            let instance = instance_lock.read();
            let target_block_state = instance
                .get_block_state(&event.position)
                .unwrap_or_default();

            if target_block_state.is_air() {
                **is_mining = false;
                continue;
            }
            let block = Box::<dyn Block>::from(target_block_state);
            **mine_progress += get_mine_progress(
                &block,
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
                **is_mining = false;
                *sequence_number += 1;
                finish_mining_events.send(FinishMiningBlockEvent {
                    entity: event.entity,
                    position: event.position,
                });
                send_packet_events.send(SendPacketEvent {
                    entity: event.entity,
                    packet: ServerboundPlayerActionPacket {
                        action: serverbound_player_action_packet::Action::StartDestroyBlock,
                        pos: event.position,
                        direction: event.direction,
                        sequence: **sequence_number,
                    }
                    .get(),
                });
                **mine_progress = 0.;
                **mine_ticks = 0.;
                **mine_delay = 0;
            }

            mine_block_progress_events.send(MineBlockProgressEvent {
                entity: event.entity,
                position: event.position,
                destroy_stage: mine_progress.destroy_stage(),
            })
        } else {
            start_mining_events.send(StartMiningBlockEvent {
                entity: event.entity,
                position: event.position,
                direction: event.direction,
            })
        }
    }
}
