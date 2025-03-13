use azalea_client::InConfigState;
use azalea_client::chunks::handle_chunk_batch_finished_event;
use azalea_client::inventory::InventorySet;
use azalea_client::packet::config::SendConfigPacketEvent;
use azalea_client::packet::game::SendPacketEvent;
use azalea_client::packet::{death_event_on_0_health, game::ResourcePackEvent};
use azalea_client::respawn::perform_respawn;
use azalea_protocol::packets::config;
use azalea_protocol::packets::game::s_resource_pack::{self, ServerboundResourcePack};
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
    mut send_config_packet_events: EventWriter<SendConfigPacketEvent>,
    query_in_config_state: Query<Option<&InConfigState>>,
) {
    for event in events.read() {
        let Ok(in_config_state_option) = query_in_config_state.get(event.entity) else {
            continue;
        };

        if in_config_state_option.is_some() {
            send_config_packet_events.send(SendConfigPacketEvent::new(
                event.entity,
                config::ServerboundResourcePack {
                    id: event.id,
                    action: config::s_resource_pack::Action::Accepted,
                },
            ));
            send_config_packet_events.send(SendConfigPacketEvent::new(
                event.entity,
                config::ServerboundResourcePack {
                    id: event.id,
                    action: config::s_resource_pack::Action::SuccessfullyLoaded,
                },
            ));
        } else {
            send_packet_events.send(SendPacketEvent::new(
                event.entity,
                ServerboundResourcePack {
                    id: event.id,
                    action: s_resource_pack::Action::Accepted,
                },
            ));
            send_packet_events.send(SendPacketEvent::new(
                event.entity,
                ServerboundResourcePack {
                    id: event.id,
                    action: s_resource_pack::Action::SuccessfullyLoaded,
                },
            ));
        }
    }
}
