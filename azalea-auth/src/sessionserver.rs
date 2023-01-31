//! Tell Mojang you're joining a multiplayer server.
use log::debug;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

use crate::game_profile::{GameProfile, SerializableGameProfile};

#[derive(Debug, Error)]
pub enum ClientSessionServerError {
    #[error("Error sending HTTP request to sessionserver: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Multiplayer is not enabled for this account")]
    MultiplayerDisabled,
    #[error("This account has been banned from multiplayer")]
    Banned,
    #[error("The authentication servers are currently not reachable")]
    AuthServersUnreachable,
    #[error("Invalid or expired session")]
    InvalidSession,
    #[error("Unknown sessionserver error: {0}")]
    Unknown(String),
    #[error("Forbidden operation (expired session?)")]
    ForbiddenOperation,
    #[error("RateLimiter disallowed request")]
    RateLimited,
    #[error("Unexpected response from sessionserver (status code {status_code}): {body}")]
    UnexpectedResponse { status_code: u16, body: String },
}

#[derive(Debug, Error)]
pub enum ServerSessionServerError {
    #[error("Error sending HTTP request to sessionserver: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Invalid or expired session")]
    InvalidSession,
    #[error("Unexpected response from sessionserver (status code {status_code}): {body}")]
    UnexpectedResponse { status_code: u16, body: String },
    #[error("Unknown sessionserver error: {0}")]
    Unknown(String),
}

#[derive(Deserialize)]
pub struct ForbiddenError {
    pub error: String,
    pub path: String,
}

/// Tell Mojang's servers that you are going to join a multiplayer server,
/// which is required to join online-mode servers. The server ID is an empty
/// string.
pub async fn join(
    access_token: &str,
    public_key: &[u8],
    private_key: &[u8],
    uuid: &Uuid,
    server_id: &str,
) -> Result<(), ClientSessionServerError> {
    let client = reqwest::Client::new();

    let server_hash = azalea_crypto::hex_digest(&azalea_crypto::digest_data(
        server_id.as_bytes(),
        public_key,
        private_key,
    ));

    let mut encode_buffer = Uuid::encode_buffer();
    let undashed_uuid = uuid.simple().encode_lower(&mut encode_buffer);

    let data = json!({
        "accessToken": access_token,
        "selectedProfile": undashed_uuid,
        "serverId": server_hash
    });
    let res = client
        .post("https://sessionserver.mojang.com/session/minecraft/join")
        .json(&data)
        .send()
        .await?;

    match res.status() {
        StatusCode::NO_CONTENT => Ok(()),
        StatusCode::FORBIDDEN => {
            let forbidden = res.json::<ForbiddenError>().await?;
            match forbidden.error.as_str() {
                "InsufficientPrivilegesException" => {
                    Err(ClientSessionServerError::MultiplayerDisabled)
                }
                "UserBannedException" => Err(ClientSessionServerError::Banned),
                "AuthenticationUnavailableException" => {
                    Err(ClientSessionServerError::AuthServersUnreachable)
                }
                "InvalidCredentialsException" => Err(ClientSessionServerError::InvalidSession),
                "ForbiddenOperationException" => Err(ClientSessionServerError::ForbiddenOperation),
                _ => Err(ClientSessionServerError::Unknown(forbidden.error)),
            }
        }
        StatusCode::TOO_MANY_REQUESTS => Err(ClientSessionServerError::RateLimited),
        status_code => {
            // log the headers
            debug!("Error headers: {:#?}", res.headers());
            let body = res.text().await?;
            Err(ClientSessionServerError::UnexpectedResponse {
                status_code: status_code.as_u16(),
                body,
            })
        }
    }
}

/// Ask Mojang's servers if the player joining is authenticated.
/// Included in the reply is the player's skin and cape.
/// The IP field is optional and equivalent to enabling
/// 'prevent-proxy-connections' in server.properties
pub async fn serverside_auth(
    username: &str,
    public_key: &[u8],
    private_key: &[u8; 16],
    ip: Option<&str>,
) -> Result<GameProfile, ServerSessionServerError> {
    let hash = azalea_crypto::hex_digest(&azalea_crypto::digest_data(
        "".as_bytes(),
        public_key,
        private_key,
    ));

    let url = reqwest::Url::parse_with_params(
        "https://sessionserver.mojang.com/session/minecraft/hasJoined",
        if let Some(ip) = ip {
            vec![("username", username), ("serverId", &hash), ("ip", ip)]
        } else {
            vec![("username", username), ("serverId", &hash)]
        },
    )
    .expect("URL should always be valid");

    let res = reqwest::get(url).await?;

    match res.status() {
        StatusCode::OK => {}
        StatusCode::NO_CONTENT => {
            return Err(ServerSessionServerError::InvalidSession);
        }
        StatusCode::FORBIDDEN => {
            return Err(ServerSessionServerError::Unknown(
                res.json::<ForbiddenError>().await?.error,
            ))
        }
        status_code => {
            // log the headers
            debug!("Error headers: {:#?}", res.headers());
            let body = res.text().await?;
            return Err(ServerSessionServerError::UnexpectedResponse {
                status_code: status_code.as_u16(),
                body,
            });
        }
    };

    Ok(res.json::<SerializableGameProfile>().await?.into())
}
