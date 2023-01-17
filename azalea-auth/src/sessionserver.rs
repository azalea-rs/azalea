//! Tell Mojang you're joining a multiplayer server.
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

use crate::game_profile::GameProfile;

#[derive(Debug, Error)]
pub enum SessionServerError {
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
    #[error("Unexpected response from sessionserver (status code {status_code}): {body}")]
    UnexpectedResponse { status_code: u16, body: String },
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
) -> Result<(), SessionServerError> {
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
        reqwest::StatusCode::NO_CONTENT => Ok(()),
        reqwest::StatusCode::FORBIDDEN => {
            let forbidden = res.json::<ForbiddenError>().await?;
            match forbidden.error.as_str() {
                "InsufficientPrivilegesException" => Err(SessionServerError::MultiplayerDisabled),
                "UserBannedException" => Err(SessionServerError::Banned),
                "AuthenticationUnavailableException" => {
                    Err(SessionServerError::AuthServersUnreachable)
                }
                "InvalidCredentialsException" => Err(SessionServerError::InvalidSession),
                "ForbiddenOperationException" => Err(SessionServerError::ForbiddenOperation),
                _ => Err(SessionServerError::Unknown(forbidden.error)),
            }
        }
        status_code => {
            // log the headers
            log::debug!("Error headers: {:#?}", res.headers());
            let body = res.text().await?;
            Err(SessionServerError::UnexpectedResponse {
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
    username: &String,
    public_key: &[u8],
    private_key: &[u8; 16],
    ip: Option<&String>,
) -> Result<GameProfile, SessionServerError> {
    let hash = azalea_crypto::hex_digest(&azalea_crypto::digest_data(
        "".as_bytes(),
        public_key,
        private_key,
    ));

    let mut url = format!("https://sessionserver.mojang.com/session/minecraft/hasJoined?username={username}&serverId={hash}");
    if let Some(ip) = ip {
        url = format!("{url}&ip={ip}");
    }

    let res = reqwest::get(url).await?;
    match res.status() {
        reqwest::StatusCode::OK => {}
        reqwest::StatusCode::NO_CONTENT => {
            return Err(SessionServerError::InvalidSession);
        }
        reqwest::StatusCode::FORBIDDEN => {
            return Err(SessionServerError::Unknown(
                res.json::<ForbiddenError>().await?.error,
            ))
        }
        status_code => {
            // log the headers
            log::debug!("Error headers: {:#?}", res.headers());
            let body = res.text().await?;
            return Err(SessionServerError::UnexpectedResponse {
                status_code: status_code.as_u16(),
                body,
            });
        }
    };

    Ok(res.json::<GameProfile>().await?)
}
