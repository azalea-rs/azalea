use azalea_buf::AzaleaWrite;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol::{
    common::client_information::ClientInformation,
    packets::config::{
        s_client_information::ServerboundClientInformation,
        s_custom_payload::ServerboundCustomPayload,
    },
};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::{client::InConfigState, packet::config::SendConfigPacketEvent};

pub struct BrandPlugin;
impl Plugin for BrandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_in_configuration_state.before(crate::packet::config::handle_send_packet_event),
        );
    }
}

pub fn handle_in_configuration_state(
    query: Query<(Entity, &ClientInformation), Added<InConfigState>>,
    mut send_packet_events: EventWriter<SendConfigPacketEvent>,
) {
    for (entity, client_information) in query.iter() {
        let mut brand_data = Vec::new();
        // azalea pretends to be vanilla everywhere else so it makes sense to lie here
        // too
        "vanilla".azalea_write(&mut brand_data).unwrap();
        send_packet_events.send(SendConfigPacketEvent::new(
            entity,
            ServerboundCustomPayload {
                identifier: ResourceLocation::new("brand"),
                data: brand_data.into(),
            },
        ));

        send_packet_events.send(SendConfigPacketEvent::new(
            entity,
            ServerboundClientInformation {
                information: client_information.clone(),
            },
        ));
    }
}
