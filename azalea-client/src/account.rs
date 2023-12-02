//! Connect to Minecraft servers.

use std::sync::Arc;

use crate::get_mc_dir;
use azalea_auth::certs::{Certificates, FetchCertificatesError};
use azalea_auth::AccessTokenResponse;
use bevy_ecs::component::Component;
use parking_lot::Mutex;
use thiserror::Error;
use uuid::Uuid;

/// Something that can join Minecraft servers.
///
/// To join a server using this account, use [`Client::join`] or
/// [`azalea::ClientBuilder`].
///
/// Note that this is also a component that our clients have.
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
#[derive(Clone, Debug, Component)]
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
    /// [`Account`]. This is used for automatic reauthentication when we get
    /// "Invalid Session" errors. If you don't need that feature (like in
    /// offline mode), then you can set this to `AuthOpts::default()`.
    pub account_opts: AccountOpts,

    /// The certificates used for chat signing.
    ///
    /// This is set when you call [`Self::request_certs`], but you only
    /// need to if the servers you're joining require it.
    pub certs: Option<Certificates>,
}

/// The parameters that were passed for creating the associated [`Account`].
#[derive(Clone, Debug)]
pub enum AccountOpts {
    Offline {
        username: String,
    },
    Microsoft {
        email: String,
    },
    MicrosoftWithAccessToken {
        msa: Arc<Mutex<azalea_auth::cache::ExpiringValue<AccessTokenResponse>>>,
    },
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
            account_opts: AccountOpts::Offline {
                username: username.to_string(),
            },
            certs: None,
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
            account_opts: AccountOpts::Microsoft {
                email: email.to_string(),
            },
            // we don't do chat signing by default unless the user asks for it
            certs: None,
        })
    }

    /// This will create an online-mode account through
    /// [`azalea_auth::get_minecraft_token`] so you can have more control over
    /// the authentication process (like doing your own caching or
    /// displaying the Microsoft user code to the user in a different way).
    ///
    /// Note that this will not refresh the token when it expires.
    ///
    /// ```
    /// # use azalea_client::Account;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::Client::new();
    ///
    /// let res = azalea_auth::get_ms_link_code(&client).await?;
    /// println!(
    ///     "Go to {} and enter the code {}",
    ///     res.verification_uri, res.user_code
    /// );
    /// let msa = azalea_auth::get_ms_auth_token(&client, res).await?;
    /// Account::with_microsoft_access_token(msa).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn with_microsoft_access_token(
        mut msa: azalea_auth::cache::ExpiringValue<AccessTokenResponse>,
    ) -> Result<Self, azalea_auth::AuthError> {
        let client = reqwest::Client::new();

        if msa.is_expired() {
            tracing::trace!("refreshing Microsoft auth token");
            msa = azalea_auth::refresh_ms_auth_token(&client, &msa.data.refresh_token).await?;
        }

        let msa_token = &msa.data.access_token;

        let res = azalea_auth::get_minecraft_token(&client, msa_token).await?;

        let profile = azalea_auth::get_profile(&client, &res.minecraft_access_token).await?;

        Ok(Self {
            username: profile.name,
            access_token: Some(Arc::new(Mutex::new(res.minecraft_access_token))),
            uuid: Some(profile.id),
            account_opts: AccountOpts::MicrosoftWithAccessToken {
                msa: Arc::new(Mutex::new(msa)),
            },
            certs: None,
        })
    }
    /// Refresh the access_token for this account to be valid again.
    ///
    /// This requires the `auth_opts` field to be set correctly (which is done
    /// by default if you used the constructor functions). Note that if the
    /// Account is offline-mode then this function won't do anything.
    pub async fn refresh(&self) -> Result<(), azalea_auth::AuthError> {
        match &self.account_opts {
            // offline mode doesn't need to refresh so just don't do anything lol
            AccountOpts::Offline { .. } => Ok(()),
            AccountOpts::Microsoft { email } => {
                let new_account = Account::microsoft(email).await?;
                let access_token_mutex = self.access_token.as_ref().unwrap();
                let new_access_token = new_account.access_token.unwrap().lock().clone();
                *access_token_mutex.lock() = new_access_token;
                Ok(())
            }
            AccountOpts::MicrosoftWithAccessToken { msa } => {
                let msa_value = msa.lock().clone();
                let new_account = Account::with_microsoft_access_token(msa_value).await?;

                let access_token_mutex = self.access_token.as_ref().unwrap();
                let new_access_token = new_account.access_token.unwrap().lock().clone();

                *access_token_mutex.lock() = new_access_token;
                let AccountOpts::MicrosoftWithAccessToken { msa: new_msa } =
                    new_account.account_opts
                else {
                    unreachable!()
                };
                *msa.lock() = new_msa.lock().clone();

                Ok(())
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum RequestCertError {
    #[error("Failed to fetch certificates")]
    FetchCertificates(#[from] FetchCertificatesError),
    #[error("You can't request certificates for an offline account")]
    NoAccessToken,
}

impl Account {
    /// Request the certificates used for chat signing and set it in
    /// [`Self::certs`].
    pub async fn request_certs(&mut self) -> Result<(), RequestCertError> {
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(RequestCertError::NoAccessToken)?
            .lock()
            .clone();
        let certs = azalea_auth::certs::fetch_certificates(&access_token).await?;
        self.certs = Some(certs);

        Ok(())
    }
}
