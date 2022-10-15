#[tokio::main]
async fn main() {
    let auth_result = azalea_auth::auth(azalea_auth::AuthOpts::default())
        .await
        .unwrap();
    println!("{:?}", auth_result);
}
