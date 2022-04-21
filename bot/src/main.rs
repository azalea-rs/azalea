use azalea_client::connect::join_server;
use azalea_client::ping::ping_server;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let address = "95.111.249.143:10000";
    // let address = "localhost:63482";
    let response = ping_server(&address.try_into().unwrap()).await.unwrap();
    // let _response = join_server(&address.try_into().unwrap()).await.unwrap();
    println!("{}", response.description.to_ansi(None));
    println!("connected");
}
