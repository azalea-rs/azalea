use std::path::PathBuf;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cache_file = PathBuf::from("example_cache.json");

    let auth_result = azalea_auth::auth(
        "example@example.com",
        azalea_auth::AuthOpts {
            cache_file: Some(cache_file),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    println!("{auth_result:?}");
}
