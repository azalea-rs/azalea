//! Connect to Minecraft servers.

use crate::{client::JoinError, Client, Event};
use azalea_protocol::ServerAddress;
use tokio::sync::mpsc::UnboundedReceiver;

/// Something that can join Minecraft servers.
pub struct Account {
    pub username: String,
    /// The access token for authentication. You can obtain one of these
    /// manually from azalea-auth.
    pub access_token: Option<String>,
    /// Only required for online-mode accounts.
    pub uuid: Option<uuid::Uuid>,
}
impl Account {
    pub fn offline(username: &str) -> Self {
        Self {
            username: username.to_string(),
            access_token: None,
            uuid: None,
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
