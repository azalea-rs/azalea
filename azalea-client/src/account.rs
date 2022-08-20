//! Connect to Minecraft servers.

use crate::{client::JoinError, Client, Event};
use azalea_protocol::ServerAddress;
use tokio::sync::mpsc::UnboundedReceiver;

/// Something that can join Minecraft servers.
pub struct Account {
    pub username: String,
}
impl Account {
    pub fn offline(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }

    /// Joins the Minecraft server on the given address using this account.
    pub async fn join(
        &self,
        address: &ServerAddress,
    ) -> Result<(Client, UnboundedReceiver<Event>), JoinError> {
        Client::join(self, address).await
    }
}
