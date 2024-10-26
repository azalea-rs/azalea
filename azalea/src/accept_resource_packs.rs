use azalea_client::chunks::handle_chunk_batch_finished_event;
use azalea_client::inventory::InventorySet;
use azalea_client::packet_handling::game::SendPacketEvent;
use azalea_client::packet_handling::{death_event_on_0_health, game::ResourcePackEvent};
use azalea_client::respawn::perform_respawn;
use azalea_protocol::packets::game::serverbound_resource_pack_packet::{
    self, ServerboundResourcePackPacket,
};
use bevy_app::Update;
use bevy_ecs::prelude::*;

use crate::app::{App, Plugin};

/// A plugin that makes it so bots automatically accept resource packs.
#[derive(Clone, Default)]
pub struct AcceptResourcePacksPlugin;
impl Plugin for AcceptResourcePacksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            accept_resource_pack
                .before(perform_respawn)
                .after(death_event_on_0_health)
                .after(handle_chunk_batch_finished_event)
                .after(InventorySet),
        );
    }
}

fn accept_resource_pack(
    mut events: EventReader<ResourcePackEvent>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
) {
    for event in events.read() {
        send_packet_events.send(SendPacketEvent {
            entity: event.entity,
            packet: ServerboundResourcePackPacket {
                id: event.id,
                action: serverbound_resource_pack_packet::Action::Accepted,
            }
            .get(),
        });
        send_packet_events.send(SendPacketEvent {
            entity: event.entity,
            packet: ServerboundResourcePackPacket {
                id: event.id,
                action: serverbound_resource_pack_packet::Action::SuccessfullyLoaded,
            }
            .get(),
        });
    }
}
