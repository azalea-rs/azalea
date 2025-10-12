use azalea_protocol::{
    common::client_information::ClientInformation,
    packets::{config::s_client_information::ServerboundClientInformation, game},
};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use tracing::{debug, warn};

use super::packet::config::SendConfigPacketEvent;
use crate::{Client, brand::send_brand, packet::login::InLoginState};

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

impl Client {
    /// Tell the server we changed our game options (i.e. render distance, main
    /// hand).
    ///
    /// If this is not set before the login packet, the default will be sent.
    ///
    /// ```rust,no_run
    /// # use azalea_client::{Client, ClientInformation};
    /// # async fn example(bot: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// bot.set_client_information(ClientInformation {
    ///     view_distance: 2,
    ///     ..Default::default()
    /// });
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_client_information(&self, client_information: ClientInformation) {
        self.query_self::<&mut ClientInformation, _>(|mut ci| {
            *ci = client_information.clone();
        });

        if self.logged_in() {
            debug!(
                "Sending client information (already logged in): {:?}",
                client_information
            );
            self.write_packet(game::s_client_information::ServerboundClientInformation {
                client_information,
            });
        }
    }
}
