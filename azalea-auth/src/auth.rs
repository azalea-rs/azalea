//! Handle Minecraft (Xbox) authentication.

use crate::cache::{self, CachedAccount, ExpiringValue};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    path::PathBuf,
    time::{Instant, SystemTime, UNIX_EPOCH},
};
use thiserror::Error;
use uuid::Uuid;

#[derive(Default)]
pub struct AuthOpts {
    /// Whether we should check if the user actually owns the game. This will
    /// fail if the user has Xbox Game Pass! Note that this isn't really
    /// necessary, since getting the user profile will check this anyways.
    pub check_ownership: bool,
    // /// Whether we should get the Minecraft profile data (i.e. username, uuid,
    // /// skin, etc) for the player.
    // pub get_profile: bool,
    /// The directory to store the cache in. If this is not set, caching is not
    /// done.
    pub cache_file: Option<PathBuf>,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error(
        "The Minecraft API is indicating that you don't own the game. \
        If you're using Xbox Game Pass, set `check_ownership` to false in the auth options."
    )]
    DoesNotOwnGame,
    #[error("Error getting Microsoft auth token: {0}")]
    GetMicrosoftAuthToken(#[from] GetMicrosoftAuthTokenError),
    #[error("Error refreshing Microsoft auth token: {0}")]
    RefreshMicrosoftAuthToken(#[from] RefreshMicrosoftAuthTokenError),
    #[error("Error getting Xbox Live auth token: {0}")]
    GetXboxLiveAuthToken(#[from] MinecraftXstsAuthError),
    #[error("Error getting Minecraft profile: {0}")]
    GetMinecraftProfile(#[from] GetProfileError),
    #[error("Error checking ownership: {0}")]
    CheckOwnership(#[from] CheckOwnershipError),
    #[error("Error getting Minecraft auth token: {0}")]
    GetMinecraftAuthToken(#[from] MinecraftAuthError),
    #[error("Error authenticating with Xbox Live: {0}")]
    GetXboxLiveAuth(#[from] XboxLiveAuthError),
}

/// Authenticate with Microsoft. If the data isn't cached,
/// they'll be asked to go to log into Microsoft in a web page.
///
/// The email is technically only used as a cache key, so it *could* be
/// anything. You should just have it be the actual email so it's not confusing
/// though, and in case the Microsoft API does start providing the real email.
///
/// If you want to use your own code to cache or show the auth code to the user
/// in a different way, use [`get_ms_link_code`], [`get_ms_auth_token`],
/// [`get_minecraft_token`] and [`get_profile`] instead.
pub async fn auth(email: &str, opts: AuthOpts) -> Result<AuthResult, AuthError> {
    let cached_account = if let Some(cache_file) = &opts.cache_file {
        cache::get_account_in_cache(cache_file, email).await
    } else {
        None
    };

    if cached_account.is_some() && !cached_account.as_ref().unwrap().mca.is_expired() {
        let account = cached_account.as_ref().unwrap();
        // the minecraft auth data is cached and not expired, so we can just
        // use that instead of doing auth all over again :)

        Ok(AuthResult {
            access_token: account.mca.data.access_token.clone(),
            profile: account.profile.clone(),
        })
    } else {
        let client = reqwest::Client::new();
        let mut msa = if let Some(account) = cached_account {
            account.msa
        } else {
            interactive_get_ms_auth_token(&client, email).await?
        };
        if msa.is_expired() {
            tracing::trace!("refreshing Microsoft auth token");
            match refresh_ms_auth_token(&client, &msa.data.refresh_token).await {
                Ok(new_msa) => msa = new_msa,
                Err(e) => {
                    // can't refresh, ask the user to auth again
                    tracing::error!("Error refreshing Microsoft auth token: {}", e);
                    msa = interactive_get_ms_auth_token(&client, email).await?;
                }
            }
        }

        let msa_token = &msa.data.access_token;
        tracing::trace!("Got access token: {msa_token}");

        let res = get_minecraft_token(&client, msa_token).await?;

        if opts.check_ownership {
            let has_game = check_ownership(&client, &res.minecraft_access_token).await?;
            if !has_game {
                return Err(AuthError::DoesNotOwnGame);
            }
        }

        let profile: ProfileResponse = get_profile(&client, &res.minecraft_access_token).await?;

        if let Some(cache_file) = opts.cache_file {
            if let Err(e) = cache::set_account_in_cache(
                &cache_file,
                email,
                CachedAccount {
                    email: email.to_string(),
                    mca: res.mca,
                    msa,
                    xbl: res.xbl,
                    profile: profile.clone(),
                },
            )
            .await
            {
                tracing::error!("{}", e);
            }
        }

        Ok(AuthResult {
            access_token: res.minecraft_access_token,
            profile,
        })
    }
}

/// Authenticate with Minecraft when we already have a Microsoft auth token.
///
/// Usually you don't need this since [`auth`] will call it for you, but it's
/// useful if you want more control over what it does.
///
/// If you don't have a Microsoft auth token, you can get it from
/// [`get_ms_link_code`] and then [`get_ms_auth_token`].
pub async fn get_minecraft_token(
    client: &reqwest::Client,
    msa: &str,
) -> Result<MinecraftTokenResponse, AuthError> {
    let xbl_auth = auth_with_xbox_live(client, msa).await?;

    let xsts_token = obtain_xsts_for_minecraft(
        client,
        &xbl_auth
            .get()
            .expect("Xbox Live auth token shouldn't have expired yet")
            .token,
    )
    .await?;

    // Minecraft auth
    let mca = auth_with_minecraft(client, &xbl_auth.data.user_hash, &xsts_token).await?;

    let minecraft_access_token: String = mca
        .get()
        .expect("Minecraft auth shouldn't have expired yet")
        .access_token
        .to_string();

    Ok(MinecraftTokenResponse {
        mca,
        xbl: xbl_auth,
        minecraft_access_token,
    })
}

#[derive(Debug)]
pub struct MinecraftTokenResponse {
    pub mca: ExpiringValue<MinecraftAuthResponse>,
    pub xbl: ExpiringValue<XboxLiveAuth>,
    pub minecraft_access_token: String,
}

#[derive(Debug)]
pub struct AuthResult {
    pub access_token: String,
    pub profile: ProfileResponse,
}

#[derive(Debug, Deserialize)]
pub struct DeviceCodeResponse {
    pub user_code: String,
    pub device_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccessTokenResponse {
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxLiveAuthResponse {
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
    pub display_claims: HashMap<String, Vec<HashMap<String, String>>>,
}

/// Just the important data
#[derive(Serialize, Deserialize, Debug)]
pub struct XboxLiveAuth {
    pub token: String,
    pub user_hash: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
pub struct MinecraftAuthResponse {
    pub username: String,
    pub roles: Vec<String>,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Deserialize)]
pub struct GameOwnershipResponse {
    pub items: Vec<GameOwnershipItem>,
    pub signature: String,
    pub key_id: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct GameOwnershipItem {
    pub name: String,
    pub signature: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProfileResponse {
    pub id: Uuid,
    pub name: String,
    pub skins: Vec<serde_json::Value>,
    pub capes: Vec<serde_json::Value>,
}

// nintendo switch (so it works for accounts that are under 18 years old)
const CLIENT_ID: &str = "00000000441cc96b";

#[derive(Debug, Error)]
pub enum GetMicrosoftAuthTokenError {
    #[error("Http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Authentication timed out")]
    Timeout,
}

/// Get the Microsoft link code that's shown to the user for logging into
/// Microsoft.
///
/// You should call [`get_ms_auth_token`] right after showing the user the
/// [`verification_uri`](DeviceCodeResponse::verification_uri) and
/// [`user_code`](DeviceCodeResponse::user_code).
///
/// If showing the link code in the terminal is acceptable, then you can just
/// use [`interactive_get_ms_auth_token`] instead.
///
/// ```
/// # async fn example(client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
/// let res = azalea_auth::get_ms_link_code(&client).await?;
/// println!(
///     "Go to {} and enter the code {}",
///     res.verification_uri, res.user_code
/// );
/// let msa = azalea_auth::get_ms_auth_token(client, res).await?;
/// let minecraft = azalea_auth::get_minecraft_token(client, &msa.data.access_token).await?;
/// let profile = azalea_auth::get_profile(&client, &minecraft.minecraft_access_token).await?;
/// # Ok(())
/// # }
/// ```
pub async fn get_ms_link_code(
    client: &reqwest::Client,
) -> Result<DeviceCodeResponse, GetMicrosoftAuthTokenError> {
    Ok(client
        .post("https://login.live.com/oauth20_connect.srf")
        .form(&vec![
            ("scope", "service::user.auth.xboxlive.com::MBI_SSL"),
            ("client_id", CLIENT_ID),
            ("response_type", "device_code"),
        ])
        .send()
        .await?
        .json::<DeviceCodeResponse>()
        .await?)
}

/// Wait until the user logged into Microsoft with the given code. You get the
/// device code response needed for this function from [`get_ms_link_code`].
///
/// You should pass the response from this to [`get_minecraft_token`].
pub async fn get_ms_auth_token(
    client: &reqwest::Client,
    res: DeviceCodeResponse,
) -> Result<ExpiringValue<AccessTokenResponse>, GetMicrosoftAuthTokenError> {
    let login_expires_at = Instant::now() + std::time::Duration::from_secs(res.expires_in);

    while Instant::now() < login_expires_at {
        tokio::time::sleep(std::time::Duration::from_secs(res.interval)).await;

        tracing::trace!("Polling to check if user has logged in...");
        if let Ok(access_token_response) = client
            .post(format!(
                "https://login.live.com/oauth20_token.srf?client_id={CLIENT_ID}"
            ))
            .form(&vec![
                ("client_id", CLIENT_ID),
                ("device_code", &res.device_code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ])
            .send()
            .await?
            .json::<AccessTokenResponse>()
            .await
        {
            tracing::trace!("access_token_response: {:?}", access_token_response);
            let expires_at = SystemTime::now()
                + std::time::Duration::from_secs(access_token_response.expires_in);
            return Ok(ExpiringValue {
                data: access_token_response,
                expires_at: expires_at
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs(),
            });
        }
    }

    Err(GetMicrosoftAuthTokenError::Timeout)
}

/// Asks the user to go to a webpage and log in with Microsoft. If you need to
/// access the code, then use [`get_ms_link_code`] and then
/// [`get_ms_auth_token`] instead.
pub async fn interactive_get_ms_auth_token(
    client: &reqwest::Client,
    email: &str,
) -> Result<ExpiringValue<AccessTokenResponse>, GetMicrosoftAuthTokenError> {
    let res = get_ms_link_code(client).await?;
    tracing::trace!("Device code response: {:?}", res);
    println!(
        "Go to \x1b[1m{}\x1b[m and enter the code \x1b[1m{}\x1b[m for \x1b[1m{}\x1b[m",
        res.verification_uri, res.user_code, email
    );

    get_ms_auth_token(client, res).await
}

#[derive(Debug, Error)]
pub enum RefreshMicrosoftAuthTokenError {
    #[error("Http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Error parsing JSON: {0}")]
    Json(#[from] serde_json::Error),
}

pub async fn refresh_ms_auth_token(
    client: &reqwest::Client,
    refresh_token: &str,
) -> Result<ExpiringValue<AccessTokenResponse>, RefreshMicrosoftAuthTokenError> {
    let access_token_response_text = client
        .post("https://login.live.com/oauth20_token.srf")
        .form(&vec![
            ("scope", "service::user.auth.xboxlive.com::MBI_SSL"),
            ("client_id", CLIENT_ID),
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ])
        .send()
        .await?
        .text()
        .await?;
    let access_token_response: AccessTokenResponse =
        serde_json::from_str(&access_token_response_text)?;

    let expires_at =
        SystemTime::now() + std::time::Duration::from_secs(access_token_response.expires_in);
    Ok(ExpiringValue {
        data: access_token_response,
        expires_at: expires_at
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs(),
    })
}

#[derive(Debug, Error)]
pub enum XboxLiveAuthError {
    #[error("Http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Invalid expiry date: {0}")]
    InvalidExpiryDate(String),
}

async fn auth_with_xbox_live(
    client: &reqwest::Client,
    access_token: &str,
) -> Result<ExpiringValue<XboxLiveAuth>, XboxLiveAuthError> {
    let auth_json = json!({
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            // i thought this was supposed to be d={} but it doesn't work for
            // me when i add it ??????
            "RpsTicket": format!("{access_token}")
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
    });
    let payload = auth_json.to_string();
    tracing::trace!("auth_json: {:#?}", auth_json);
    let res = client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("x-xbl-contract-version", "1")
        // .header("Cache-Control", "no-store, must-revalidate, no-cache")
        // .header("Signature", base64::encode(signature))
        .body(payload)
        .send()
        .await?
        .json::<XboxLiveAuthResponse>()
        .await?;
    tracing::trace!("Xbox Live auth response: {:?}", res);

    // not_after looks like 2020-12-21T19:52:08.4463796Z
    let expires_at = DateTime::parse_from_rfc3339(&res.not_after)
        .map_err(|e| XboxLiveAuthError::InvalidExpiryDate(format!("{}: {e}", res.not_after)))?
        .with_timezone(&Utc)
        .timestamp() as u64;
    Ok(ExpiringValue {
        data: XboxLiveAuth {
            token: res.token,
            user_hash: res.display_claims["xui"].first().unwrap()["uhs"].clone(),
        },
        expires_at,
    })
}

#[derive(Debug, Error)]
pub enum MinecraftXstsAuthError {
    #[error("Http error: {0}")]
    Http(#[from] reqwest::Error),
}

async fn obtain_xsts_for_minecraft(
    client: &reqwest::Client,
    xbl_auth_token: &str,
) -> Result<String, MinecraftXstsAuthError> {
    let res = client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .header("Accept", "application/json")
        .json(&json!({
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [xbl_auth_token.to_string()]
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
        }))
        .send()
        .await?
        .json::<XboxLiveAuthResponse>()
        .await?;
    tracing::trace!("Xbox Live auth response (for XSTS): {:?}", res);

    Ok(res.token)
}

#[derive(Debug, Error)]
pub enum MinecraftAuthError {
    #[error("Http error: {0}")]
    Http(#[from] reqwest::Error),
}

async fn auth_with_minecraft(
    client: &reqwest::Client,
    user_hash: &str,
    xsts_token: &str,
) -> Result<ExpiringValue<MinecraftAuthResponse>, MinecraftAuthError> {
    let res = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .header("Accept", "application/json")
        .json(&json!({
            "identityToken": format!("XBL3.0 x={user_hash};{xsts_token}")
        }))
        .send()
        .await?
        .json::<MinecraftAuthResponse>()
        .await?;
    tracing::trace!("{:?}", res);

    let expires_at = SystemTime::now() + std::time::Duration::from_secs(res.expires_in);
    Ok(ExpiringValue {
        data: res,
        // to seconds since epoch
        expires_at: expires_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
    })
}

#[derive(Debug, Error)]
pub enum CheckOwnershipError {
    #[error("Http error: {0}")]
    Http(#[from] reqwest::Error),
}

pub async fn check_ownership(
    client: &reqwest::Client,
    minecraft_access_token: &str,
) -> Result<bool, CheckOwnershipError> {
    let res = client
        .get("https://api.minecraftservices.com/entitlements/mcstore")
        .header("Authorization", format!("Bearer {minecraft_access_token}"))
        .send()
        .await?
        .json::<GameOwnershipResponse>()
        .await?;
    tracing::trace!("{:?}", res);

    // vanilla checks here to make sure the signatures are right, but it's not
    // actually required so we just don't

    Ok(!res.items.is_empty())
}

#[derive(Debug, Error)]
pub enum GetProfileError {
    #[error("Http error: {0}")]
    Http(#[from] reqwest::Error),
}

pub async fn get_profile(
    client: &reqwest::Client,
    minecraft_access_token: &str,
) -> Result<ProfileResponse, GetProfileError> {
    let res = client
        .get("https://api.minecraftservices.com/minecraft/profile")
        .header("Authorization", format!("Bearer {minecraft_access_token}"))
        .send()
        .await?
        .json::<ProfileResponse>()
        .await?;
    tracing::trace!("{:?}", res);

    Ok(res)
}
