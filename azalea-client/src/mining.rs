use azalea_block::BlockState;
use azalea_core::{BlockPos, Direction, GameMode};
use azalea_inventory::ItemSlot;
use azalea_protocol::packets::game::serverbound_player_action_packet::{
    self, ServerboundPlayerActionPacket,
};
use azalea_world::{entity::InstanceName, InstanceContainer};
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};

use crate::{
    interact::{check_is_interaction_restricted, CurrentSequenceNumber},
    inventory::InventoryComponent,
    local_player::{LocalGameMode, SendPacketEvent},
    Client,
};

/// A plugin that allows clients to break blocks in the world.
pub struct MinePlugin;
impl Plugin for MinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartMiningBlockEvent>()
            .add_system(handle_start_mining_block_event);
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
    mut query: Query<(
        &InstanceName,
        &LocalGameMode,
        &InventoryComponent,
        &mut CurrentSequenceNumber,
        &mut MineDelay,
        &mut IsMining,
        &mut MineProgress,
        &MineBlockPos,
        &MineItem,
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
            current_mining_pos,
            current_mining_item,
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
                current_mining_pos,
                current_mining_item,
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

            if block_is_solid && get_mine_progress() {
                //
            }
        }
    }
}

/// A player left clicked on a block, used for stuff like interacting with note
/// blocks.
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

/// A component that stores the position of the block we're currently mining.
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct MineBlockPos(pub Option<BlockPos>);

/// A component that contains the item we're currently using to mine. If we're
/// not mining anything, it'll be [`ItemSlot::Empty`].
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct MineItem(pub ItemSlot);

/// Sent when we completed mining a block.
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
        &mut CurrentSequenceNumber,
    )>,
    instances: Res<InstanceContainer>,
) {
    for event in events.iter() {
        let (instance_name, game_mode, inventory, sequence_number) =
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

        // TODO when item categories are implemented: can't breaks blocks in
        // creative mode if we're holding a sword or trident

        // if game_mode.current == GameMode::Creative {
        //     let held_item = inventory.held_item().kind();
        // }

        let Some(block) = instance.get_block_state(&event.position) else { continue };

        // also related to the above todo: ignore if it's a "game master block"
        // and we're not allowed to use them

        if block == BlockState::AIR {
            continue;
        }

        // block.
    }
}
