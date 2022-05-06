use azalea_client::{Account, Event};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // let address = "95.111.249.143:10000";
    let address = "172.23.192.1:62522";
    // let response = azalea_client::ping::ping_server(&address.try_into().unwrap())
    //     .await
    //     .unwrap();

    // println!("{}", response.description.to_ansi(None));
    let account = Account::offline("bot");
    let mut client = account.join(&address.try_into().unwrap()).await.unwrap();
    println!("connected");

    while let Some(e) = client.next().await {
        match e {
            Event::Login => {}
        }
    }

    println!("done");
}
