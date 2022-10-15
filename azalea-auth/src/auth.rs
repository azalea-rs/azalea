//! Handle Minecraft (Xbox) authentication.


use anyhow::anyhow;
use serde::Deserialize;
use serde_json::json;
use std::{collections::HashMap, time::Instant};

#[derive(Default)]
pub struct AuthOpts {
    /// Whether we should check if the user actually owns the game. This will
    /// fail if the user has Xbox Game Pass! Note that this isn't really
    /// necessary, since getting the user profile will check this anyways.
    pub check_ownership: bool,
    // /// Whether we should get the Minecraft profile data (i.e. username, uuid,
    // /// skin, etc) for the player.
    // pub get_profile: bool,
}

/// Authenticate with authenticate with Microsoft. If the data isn't cached,
/// they'll be asked to go to log into Microsoft in a web page.
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

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    token_type: String,
    expires_in: u64,
    scope: String,
    access_token: String,
    refresh_token: String,
    user_id: String,
}

#[allow(unused)]
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

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct MinecraftAuthResponse {
    username: String,
    roles: Vec<String>,
    access_token: String,
    token_type: String,
    expires_in: u64,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct GameOwnershipResponse {
    items: Vec<GameOwnershipItem>,
    signature: String,
    key_id: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct GameOwnershipItem {
    name: String,
    signature: String,
}

#[derive(Debug, Deserialize)]
pub struct ProfileResponse {
    pub id: String,
    pub name: String,
    pub skins: Vec<serde_json::Value>,
    pub capes: Vec<serde_json::Value>,
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
            // i thought this was supposed to be d={} but it doesn't work for
            // me when i add it ??????
            "RpsTicket": format!("{}", access_token)
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
    });
    let payload = auth_json.to_string();
    // let signature = sign(
    //     "https://user.auth.xboxlive.com/user/authenticate",
    //     "",
    //     &payload,
    // )?;
    println!("auth_json: {:#?}", auth_json);
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
    println!("got res: {:?}", res);

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
