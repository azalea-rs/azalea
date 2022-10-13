#[tokio::main]
async fn main() {
    let auth_result = azalea_auth::auth(None).await.unwrap();
    println!("{:?}", auth_result);
}
