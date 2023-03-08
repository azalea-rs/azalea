use azalea_core::{BlockPos, Direction};
use azalea_protocol::packets::game::{
    serverbound_interact_packet::InteractionHand,
    serverbound_use_item_on_packet::{BlockHitResult, ServerboundUseItemOnPacket},
};
use bevy_app::{App, Plugin};
use bevy_ecs::{component::Component, entity::Entity, event::EventReader, system::Query};
use derive_more::{Deref, DerefMut};
use log::warn;

use crate::{Client, LocalPlayer};

/// A plugin that allows clients to interact with blocks in the world.
pub struct InteractPlugin;
impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockInteractEvent>()
            .add_system(handle_block_interact_event);
    }
}

impl Client {
    /// Right click a block. The behavior of this depends on the target block,
    /// and it'll either place the block you're holding in your hand or use the
    /// block you clicked (like toggling a lever).
    ///
    /// Note that this may trigger anticheats as it doesn't take into account
    /// whether you're actually looking at the block.
    pub fn block_interact(&mut self, position: BlockPos) {
        self.ecs.lock().send_event(BlockInteractEvent {
            entity: self.entity,
            position,
        });
    }
}

/// Right click a block. The behavior of this depends on the target block,
/// and it'll either place the block you're holding in your hand or use the
/// block you clicked (like toggling a lever).
pub struct BlockInteractEvent {
    /// The local player entity that's opening the container.
    pub entity: Entity,
    /// The coordinates of the container.
    pub position: BlockPos,
}

/// The number of changes this client has made to blocks.
#[derive(Component, Copy, Clone, Debug, Default, Deref, DerefMut)]
pub struct CurrentSequenceNumber(u32);

fn handle_block_interact_event(
    mut events: EventReader<BlockInteractEvent>,
    mut query: Query<(&LocalPlayer, &mut CurrentSequenceNumber)>,
) {
    for event in events.iter() {
        let Ok((local_player, mut sequence_number)) = query.get_mut(event.entity) else {
            warn!("Sent BlockInteractEvent for entity that isn't LocalPlayer");
            continue;
        };

        // TODO: check to make sure we're within the world border

        **sequence_number += 1;

        // minecraft also does the interaction client-side (so it looks like clicking a
        // button is instant) but we don't really need that

        local_player.write_packet(
            ServerboundUseItemOnPacket {
                hand: InteractionHand::MainHand,
                block_hit: BlockHitResult {
                    block_pos: event.position,
                    direction: Direction::Up,
                    location: event.position.center(),
                    inside: false,
                },
                sequence: sequence_number.0,
            }
            .get(),
        )
    }
}
