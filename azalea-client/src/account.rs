//! Connect to Minecraft servers.

use crate::get_mc_dir;
use uuid::Uuid;

/// Something that can join Minecraft servers.
///
/// To join a server using this account, use [`crate::Client::join`].
///
/// # Examples
///
/// ```rust,no_run
/// use azalea_client::Account;
///
/// # #[tokio::main]
/// # async fn main() {
/// let account = Account::microsoft("example@example.com").await;
/// // or Account::offline("example");
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Account {
    /// The Minecraft username of the account.
    pub username: String,
    /// The access token for authentication. You can obtain one of these
    /// manually from azalea-auth.
    pub access_token: Option<String>,
    /// Only required for online-mode accounts.
    pub uuid: Option<uuid::Uuid>,
}

impl Account {
    /// An offline account does not authenticate with Microsoft's servers, and
    /// as such can only join offline mode servers. This is useful for testing
    /// in LAN worlds.
    pub fn offline(username: &str) -> Self {
        Self {
            username: username.to_string(),
            access_token: None,
            uuid: None,
        }
    }

    /// This will create an online-mode account by authenticating with
    /// Microsoft's servers. Note that the email given is actually only used as
    /// a key for the cache, but it's recommended to use the real email to
    /// avoid confusion.
    pub async fn microsoft(email: &str) -> Result<Self, azalea_auth::AuthError> {
        let minecraft_dir = get_mc_dir::minecraft_dir().unwrap_or_else(|| {
            panic!(
                "No {} environment variable found",
                get_mc_dir::home_env_var()
            )
        });
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
}
