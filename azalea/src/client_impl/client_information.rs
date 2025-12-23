use azalea_client::ClientInformation;
use azalea_protocol::packets::game;
use tracing::debug;

use crate::Client;

impl Client {
    /// Tell the server we changed our game options (i.e. render distance, main
    /// hand).
    ///
    /// If this is not set before the login packet, the default will be sent.
    ///
    /// ```rust,no_run
    /// # use azalea::{Client, ClientInformation};
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
