use azalea_client::connect::join_server;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let address = "95.111.249.143:10000";
    let _response = join_server(&address.try_into().unwrap()).await.unwrap();
    // println!("{}", response.description.to_ansi(None));
    println!("connected");
}
