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
use tracing::{debug, warn};

use crate::{
    client::InConfigState,
    packet_handling::{configuration::SendConfigurationEvent, login::InLoginState},
};

pub struct ConfigurationPlugin;
impl Plugin for ConfigurationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_end_login_state
                .before(crate::packet_handling::configuration::handle_send_packet_event),
        );
    }
}

fn handle_end_login_state(
    mut removed: RemovedComponents<InLoginState>,
    query: Query<&ClientInformation>,
    mut send_packet_events: EventWriter<SendConfigurationEvent>,
) {
    for entity in removed.read() {
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

        let client_information = match query.get(entity).ok() {
            Some(i) => i,
            None => {
                warn!("ClientInformation component was not set before leaving login state, using a default");
                &ClientInformation::default()
            }
        };

        debug!("Writing ClientInformation while in config state: {client_information:?}");
        send_packet_events.send(SendConfigurationEvent::new(
            entity,
            ServerboundClientInformation {
                information: client_information.clone(),
            },
        ));
    }
}
