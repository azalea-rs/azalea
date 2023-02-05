//! Connect to Minecraft servers.

use std::sync::Arc;

use crate::get_mc_dir;
use parking_lot::Mutex;
use uuid::Uuid;

/// Something that can join Minecraft servers.
///
/// To join a server using this account, use [`Client::join`] or
/// [`azalea::ClientBuilder`].
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
///
/// [`Client::join`]: crate::Client::join
/// [`azalea::ClientBuilder`]: https://docs.rs/azalea/latest/azalea/struct.ClientBuilder.html
#[derive(Clone, Debug)]
pub struct Account {
    /// The Minecraft username of the account.
    pub username: String,
    /// The access token for authentication. You can obtain one of these
    /// manually from azalea-auth.
    ///
    /// This is an `Arc<Mutex>` so it can be modified by [`Self::refresh`].
    pub access_token: Option<Arc<Mutex<String>>>,
    /// Only required for online-mode accounts.
    pub uuid: Option<Uuid>,

    /// The parameters (i.e. email) that were passed for creating this
    /// [`Account`]. This is used to for automatic reauthentication when we get
    /// "Invalid Session" errors. If you don't need that feature (like in
    /// offline mode), then you can set this to `AuthOpts::default()`.
    pub auth_opts: AuthOpts,
}

/// The parameters that were passed for creating the associated [`Account`].
#[derive(Clone, Debug)]
pub enum AuthOpts {
    Offline { username: String },
    // this is an enum so legacy Mojang auth can be added in the future
    Microsoft { email: String },
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
            auth_opts: AuthOpts::Offline {
                username: username.to_string(),
            },
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
            access_token: Some(Arc::new(Mutex::new(auth_result.access_token))),
            uuid: Some(auth_result.profile.id),
            auth_opts: AuthOpts::Microsoft {
                email: email.to_string(),
            },
        })
    }

    /// Refresh the access_token for this account to be valid again.
    ///
    /// This requires the `auth_opts` field to be set correctly (which is done
    /// by default if you used the constructor functions). Note that if the
    /// Account is offline-mode, this function won't do anything.
    pub async fn refresh(&self) -> Result<(), azalea_auth::AuthError> {
        match &self.auth_opts {
            // offline mode doesn't need to refresh so just don't do anything lol
            AuthOpts::Offline { .. } => Ok(()),
            AuthOpts::Microsoft { email } => {
                let new_account = Account::microsoft(email).await?;
                let access_token = self
                    .access_token
                    .as_ref()
                    .expect("Access token should always be set for Microsoft accounts");
                let new_access_token = new_account
                    .access_token
                    .expect("Access token should always be set for Microsoft accounts")
                    .lock()
                    .clone();
                *access_token.lock() = new_access_token;
                Ok(())
            }
        }
    }
}
