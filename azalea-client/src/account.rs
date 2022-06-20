use crate::Client;
use azalea_protocol::ServerAddress;

///! Connect to Minecraft servers.

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

    pub async fn join(&self, address: &ServerAddress) -> Result<Client, String> {
        Client::join(self, address).await
    }
}
