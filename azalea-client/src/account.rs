//! Connect to Minecraft servers.

use crate::{client::JoinError, get_mc_dir, Client, Event};
use azalea_protocol::ServerAddress;
use tokio::sync::mpsc::UnboundedReceiver;
use uuid::Uuid;

/// Something that can join Minecraft servers.
#[derive(Clone, Debug)]
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

    pub async fn microsoft(email: &str) -> Result<Self, azalea_auth::AuthError> {
        let minecraft_dir = get_mc_dir::minecraft_dir().unwrap();
        let auth_result = azalea_auth::auth(
            email,
            azalea_auth::AuthOpts {
                cache_file: Some(minecraft_dir.join("azalea-auth.json")),
                ..Default::default()
            },
        )
        .await?;
        Ok(Self {
            username: auth_result.profile.name,
            access_token: Some(auth_result.access_token),
            uuid: Some(Uuid::parse_str(&auth_result.profile.id).expect("Invalid UUID")),
        })
    }

    /// Joins the Minecraft server on the given address using this account.
    pub async fn join(
        &self,
        address: &ServerAddress,
    ) -> Result<(Client, UnboundedReceiver<Event>), JoinError> {
        Client::join(self, address).await
    }
}
