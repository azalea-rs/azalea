use serde_json::json;

/// Tell Mojang's servers that you are going to join a multiplayer server.
/// The server ID is an empty string.
pub async fn join(
    access_token: &str,
    public_key: &str,
    private_key: &str,
    undashed_uuid: &str,
    server_id: &str,
) {
    let client = reqwest::Client::new();

    let server_hash = azalea_crypto::hex_digest(&azalea_crypto::digest_data(
        server_id,
        public_key,
        private_key,
    ));

    client
        .post("https://sessionserver.mojang.com/session/minecraft/join")
        .json(json! {
            accessToken: access_token,
            selectedProfile: undashed_uuid,
            serverId: server_hash
        })
        .send()
        .await;
}
