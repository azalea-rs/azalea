//! Authenticate with Microsoft and get a Minecraft profile, but don't cache and
//! use our own code to display the link code.
//!
//! If you still want it to cache, look at the code in [`azalea_auth::auth`] and
//! see how that does it.

use std::error::Error;

use azalea_auth::ProfileResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let profile = auth().await?;
    println!("Logged in as {}", profile.name);

    Ok(())
}

// We will be using default `client_id` and `scope`
async fn auth() -> Result<ProfileResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let res = azalea_auth::get_ms_link_code(&client, None, None).await?;
    println!(
        "Go to {} and enter the code {}",
        res.verification_uri, res.user_code
    );
    let msa = azalea_auth::get_ms_auth_token(&client, res, None).await?;
    let auth_result = azalea_auth::get_minecraft_token(&client, &msa.data.access_token).await?;
    Ok(azalea_auth::get_profile(&client, &auth_result.minecraft_access_token).await?)
}
