#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // let address = "95.111.249.143:10000";
    let address = "localhost:50332";
    // let response = azalea_client::ping::ping_server(&address.try_into().unwrap())
    //     .await
    //     .unwrap();

    // println!("{}", response.description.to_ansi(None));
    let account = azalea_client::Account::offline("bot");
    let client = account.join(&address.try_into().unwrap()).await.unwrap();
    println!("connected");

    // loop {
    // match client.next().await {}
    // }
}
