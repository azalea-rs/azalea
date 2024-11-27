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

use crate::{client::InConfigurationState, packet_handling::configuration::SendConfigurationEvent};

pub struct ConfigurationPlugin;
impl Plugin for ConfigurationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_in_configuration_state
                .after(crate::packet_handling::configuration::handle_send_packet_event),
        );
    }
}

fn handle_in_configuration_state(
    query: Query<(Entity, &ClientInformation), Added<InConfigurationState>>,
    mut send_packet_events: EventWriter<SendConfigurationEvent>,
) {
    for (entity, client_information) in query.iter() {
        let mut brand_data = Vec::new();
        // they don't have to know :)
        "vanilla".azalea_write(&mut brand_data).unwrap();
        send_packet_events.send(SendConfigurationEvent::new(
            entity,
            ServerboundCustomPayload {
                identifier: ResourceLocation::new("brand"),
                data: brand_data.into(),
            },
        ));

        send_packet_events.send(SendConfigurationEvent::new(
            entity,
            ServerboundClientInformation {
                information: client_information.clone(),
            },
        ));
    }
}
