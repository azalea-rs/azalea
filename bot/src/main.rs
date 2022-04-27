#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // let address = "95.111.249.143:10000";
    let address = "localhost:53810";
    // let response = azalea_client::ping::ping_server(&address.try_into().unwrap())
    //     .await
    //     .unwrap();

    // println!("{}", response.description.to_ansi(None));
    let _response = azalea_client::connect::join_server(&address.try_into().unwrap())
        .await
        .unwrap();
    println!("connected");
}
