use azalea_protocol::{
    common::client_information::ClientInformation,
    packets::config::s_client_information::ServerboundClientInformation,
};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use tracing::{debug, warn};

use super::packet::config::SendConfigPacketEvent;
use crate::{brand::send_brand, packet::login::InLoginState};

/// Send [`ServerboundClientInformation`] on join.
pub struct ClientInformationPlugin;
impl Plugin for ClientInformationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, send_client_information.after(send_brand));
    }
}

pub fn send_client_information(
    mut commands: Commands,
    mut removed: RemovedComponents<InLoginState>,
    query: Query<&ClientInformation>,
) {
    for entity in removed.read() {
        let client_information = match query.get(entity).ok() {
            Some(i) => i,
            None => {
                warn!(
                    "ClientInformation component was not set before leaving login state, using a default"
                );
                &ClientInformation::default()
            }
        };

        debug!("Writing ClientInformation while in config state: {client_information:?}");
        commands.trigger(SendConfigPacketEvent::new(
            entity,
            ServerboundClientInformation {
                information: client_information.clone(),
            },
        ));
    }
}
