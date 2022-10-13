//! Handle Minecraft authentication.

pub mod game_profile;

use std::{
    collections::HashMap,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::anyhow;
use azalea_buf::McBufWritable;
use num_bigint::BigUint;
use reqwest::Url;
use serde::Deserialize;
use serde_json::json;

#[derive(Default)]
pub struct AuthOpts {
    /// Whether we should check if the user actually owns the game. This will
    /// fail if the user has Xbox Game Pass! Note that this isn't really
    /// necessary, since getting the user profile will check this anyways.
    pub check_ownership: bool,
}

pub async fn auth(opts: Option<AuthOpts>) -> anyhow::Result<AuthResult> {
    let opts = opts.unwrap_or_default();

    let client = reqwest::Client::new();

    let auth_token_res = interactive_get_auth_token(&client).await?;
    // TODO: cache this
    println!("Got access token: {}", auth_token_res.access_token);

    let xbl_auth = auth_with_xbox_live(&client, &auth_token_res.access_token).await?;

    let xsts_token = obtain_xsts_for_minecraft(&client, &xbl_auth).await?;

    let minecraft_access_token =
        auth_with_minecraft(&client, &xbl_auth.user_hash, &xsts_token).await?;

    if opts.check_ownership {
        let has_game = check_ownership(&client, &minecraft_access_token).await?;
        if !has_game {
            panic!(
                "The Minecraft API is indicating that you don't own the game. \
				If you're using Xbox Game Pass, set `check_ownership` to false in the auth options."
            );
        }
    }

    let profile = get_profile(&client, &minecraft_access_token).await?;

    Ok(AuthResult {
        access_token: minecraft_access_token,
        profile,
    })
}

#[derive(Debug)]
pub struct AuthResult {
    pub access_token: String,
    pub profile: ProfileResponse,
}

#[derive(Debug, Deserialize)]
pub struct DeviceCodeResponse {
    user_code: String,
    device_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    token_type: String,
    expires_in: u64,
    scope: String,
    access_token: String,
    refresh_token: String,
    user_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxLiveAuthResponse {
    issue_instant: String,
    not_after: String,
    token: String,
    display_claims: HashMap<String, Vec<HashMap<String, String>>>,
}

/// Just the important data
pub struct XboxLiveAuth {
    token: String,
    user_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct MinecraftAuthResponse {
    username: String,
    roles: Vec<String>,
    access_token: String,
    token_type: String,
    expires_in: u64,
}

#[derive(Debug, Deserialize)]
pub struct GameOwnershipResponse {
    items: Vec<GameOwnershipItem>,
    signature: String,
    key_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GameOwnershipItem {
    name: String,
    signature: String,
}

#[derive(Debug, Deserialize)]
pub struct ProfileResponse {
    id: String,
    name: String,
    skins: Vec<serde_json::Value>,
    capes: Vec<serde_json::Value>,
}

/// Asks the user to go to a webpage and log in with Microsoft.
async fn interactive_get_auth_token(
    client: &reqwest::Client,
) -> anyhow::Result<AccessTokenResponse> {
    // nintendo switch (real)
    let client_id = "00000000441cc96b";

    let res = client
        .post("https://login.live.com/oauth20_connect.srf")
        .form(&vec![
            ("scope", "service::user.auth.xboxlive.com::MBI_SSL"),
            ("client_id", client_id),
            ("response_type", "device_code"),
        ])
        .send()
        .await?
        .json::<DeviceCodeResponse>()
        .await?;
    println!("{:?}", res);
    println!(
        "Go to {} and enter the code {}",
        res.verification_uri, res.user_code
    );

    let access_token_response: AccessTokenResponse;

    let expire_time = Instant::now() + std::time::Duration::from_secs(res.expires_in);

    while Instant::now() < expire_time {
        tokio::time::sleep(std::time::Duration::from_secs(res.interval)).await;

        println!("trying");
        if let Ok(res) = client
            .post(format!(
                "https://login.live.com/oauth20_token.srf?client_id={}",
                client_id
            ))
            .form(&vec![
                ("client_id", client_id),
                ("device_code", &res.device_code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ])
            .send()
            .await?
            .json::<AccessTokenResponse>()
            .await
        {
            access_token_response = res;
            return Ok(access_token_response);
        }
    }

    Err(anyhow!("Authentication timed out"))
}

async fn auth_with_xbox_live(
    client: &reqwest::Client,
    access_token: &str,
) -> anyhow::Result<XboxLiveAuth> {
    let auth_json = json!({
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": format!("d={}", access_token)
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
    });
    let payload = auth_json.to_string();
    let signature = sign(
        "https://user.auth.xboxlive.com/user/authenticate",
        "",
        &payload,
    )?;
    println!("auth_json: {:#?}", auth_json);
    let res = client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("x-xbl-contract-version", "2")
        .header("Cache-Control", "no-store, must-revalidate, no-cache")
        .header("Signature", base64::encode(signature))
        .body(payload)
        .send()
        .await?
        .json::<XboxLiveAuthResponse>()
        .await?;
    println!("got res: {:?}", res);
    panic!("ok");

    Ok(XboxLiveAuth {
        token: res.token,
        user_hash: res.display_claims["xui"].get(0).unwrap()["uhs"].clone(),
    })
}

async fn obtain_xsts_for_minecraft(
    client: &reqwest::Client,
    xbl_auth: &XboxLiveAuth,
) -> anyhow::Result<String> {
    let res = client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .header("Accept", "application/json")
        .json(&json!({
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [xbl_auth.token]
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
        }))
        .send()
        .await?
        .json::<XboxLiveAuthResponse>()
        .await?;
    println!("{:?}", res);

    Ok(res.token)
}

async fn auth_with_minecraft(
    client: &reqwest::Client,
    user_hash: &str,
    xsts_token: &str,
) -> anyhow::Result<String> {
    let res = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .header("Accept", "application/json")
        .json(&json!({
            "identityToken": format!("XBL3.0 x={};{}", user_hash, xsts_token)
        }))
        .send()
        .await?
        .json::<MinecraftAuthResponse>()
        .await?;
    println!("{:?}", res);

    Ok(res.access_token)
}

async fn check_ownership(
    client: &reqwest::Client,
    minecraft_access_token: &str,
) -> anyhow::Result<bool> {
    let res = client
        .get("https://api.minecraftservices.com/entitlements/mcstore")
        .header(
            "Authorization",
            format!("Bearer {}", minecraft_access_token),
        )
        .send()
        .await?
        .json::<GameOwnershipResponse>()
        .await?;
    println!("{:?}", res);

    // TODO: we *should* check with mojang's public key that the signatures are right

    Ok(!res.items.is_empty())
}

async fn get_profile(
    client: &reqwest::Client,
    minecraft_access_token: &str,
) -> anyhow::Result<ProfileResponse> {
    let res = client
        .get("https://api.minecraftservices.com/minecraft/profile")
        .header(
            "Authorization",
            format!("Bearer {}", minecraft_access_token),
        )
        .send()
        .await?
        .json::<ProfileResponse>()
        .await?;
    println!("{:?}", res);

    Ok(res)
}

// from https://github.com/PrismarineJS/prismarine-auth/blob/master/src/TokenManagers/XboxTokenManager.js#L112
fn sign(url: &str, authorization_token: &str, payload: &str) -> anyhow::Result<Vec<u8>> {
    // const windowsTimestamp = (BigInt((Date.now() / 1000) | 0) + 11644473600n) * 10000000n
    // // Only the /uri?and-query-string
    // const pathAndQuery = new URL(url).pathname

    // // Allocate the buffer for signature, TS, path, tokens and payload and NUL termination
    // const allocSize = /* sig */ 5 + /* ts */ 9 + /* POST */ 5 + pathAndQuery.length + 1 + authorizationToken.length + 1 + payload.length + 1
    // const buf = SmartBuffer.fromSize(allocSize)
    // buf.writeInt32BE(1) // Policy Version
    // buf.writeUInt8(0)
    // buf.writeBigUInt64BE(windowsTimestamp)
    // buf.writeUInt8(0) // null term
    // buf.writeStringNT('POST')
    // buf.writeStringNT(pathAndQuery)
    // buf.writeStringNT(authorizationToken)
    // buf.writeStringNT(payload)

    let windows_timestamp =
        BigUint::from((SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 11644473600))
            * 10000000;
    let path_and_query = Url::parse(url)?.path();
    let mut buf = Vec::new();
    1u32.write_into(&mut buf)?; // policy version
    0u8.write_into(&mut buf)?;
    windows_timestamp.write_into(&mut buf)?;
    0u8.write_into(&mut buf)?; // null term
    "POST".write_into(&mut buf)?;
    path_and_query.write_into(&mut buf)?;
    authorization_token.write_into(&mut buf)?;
    payload.write_into(&mut buf)?;

    // const keyPair = crypto.generateKeyPairSync('ec', { namedCurve: 'P-256' })

    // // Get the signature from the payload
    // const signature = crypto.sign('SHA256', buf.toBuffer(), { key: this.key.privateKey, dsaEncoding: 'ieee-p1363' })

    // const header = SmartBuffer.fromSize(signature.length + 12)
    // header.writeInt32BE(1) // Policy Version
    // header.writeBigUInt64BE(windowsTimestamp)
    // header.writeBuffer(signature) // Add signature at end of header

    // return header.toBuffer()

    let key_pair = EcKeyPair::generate(&EcParams::by_curve_nid(Nid::X9_62_PRIME256V1)?)?;

    let signature =
        key_pair
            .private_key()
            .sign(MessageDigest::sha256(), &buf, &mut BigNumContext::new()?)?;

    let mut header = Vec::new();
    1u32.write_into(&mut header)?; // policy version
    windows_timestamp.write_into(&mut header)?;
    signature.write_into(&mut header)?;

    Ok(header)
}
