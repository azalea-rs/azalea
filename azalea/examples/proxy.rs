//! A `simple` server that gets login information and proxies connections.
//! After login all connections are encrypted and Azalea cannot read them.

use azalea_protocol::{
    connect::Connection,
    packets::{
        handshake::{
            client_intention_packet::ClientIntentionPacket, ClientboundHandshakePacket,
            ServerboundHandshakePacket,
        },
        login::{serverbound_hello_packet::ServerboundHelloPacket, ServerboundLoginPacket},
        status::{
            clientbound_pong_response_packet::ClientboundPongResponsePacket,
            clientbound_status_response_packet::{
                ClientboundStatusResponsePacket, Players, Version,
            },
            ServerboundStatusPacket,
        },
        ConnectionProtocol,
    },
    read::ReadPacketError,
};
use futures::FutureExt;
use log::{error, info, warn};
use std::error::Error;
use tokio::{
    io::{self, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Bind to an address and port
    let listener = TcpListener::bind("127.0.0.1:25566").await?;
    loop {
        // When a connection is made, pass it off to another thread
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: TcpStream) -> anyhow::Result<()> {
    stream.set_nodelay(true)?;
    let ip = stream.peer_addr()?;
    let mut conn: Connection<ServerboundHandshakePacket, ClientboundHandshakePacket> =
        Connection::wrap(stream);

    // The first packet sent from a client is the intent packet.
    // This specifies whether the client is pinging
    // the server or is going to join the game.
    let intent = match conn.read().await {
        Err(e) => {
            let e = e.into();
            error!("{e}");
            return Err(e);
        }
        Ok(p) => match p {
            ServerboundHandshakePacket::ClientIntention(i) => {
                info!(
                    "New connection: {0}, Version {1}, {2:?}",
                    ip, i.protocol_version, i.intention
                );
                i
            }
        },
    };

    match intent.intention {
        // If the client is pinging the proxy,
        // reply with the information below.
        ConnectionProtocol::Status => {
            let mut conn = conn.status();
            loop {
                match conn.read().await {
                    Ok(p) => match p {
                        ServerboundStatusPacket::StatusRequest(_) => {
                            conn.write(
                                ClientboundStatusResponsePacket {
                                    description: String::from("An Azalea Minecraft Proxy").into(),
                                    favicon: None,
                                    players: Players {
                                        max: 1,
                                        online: 0,
                                        sample: Vec::new(),
                                    },
                                    version: Version {
                                        name: String::from("1.19.3"),
                                        protocol: 761,
                                    },
                                    previews_chat: Some(false),
                                    enforces_secure_chat: Some(false),
                                }
                                .get(),
                            )
                            .await?;
                        }
                        ServerboundStatusPacket::PingRequest(p) => {
                            conn.write(ClientboundPongResponsePacket { time: p.time }.get())
                                .await?;
                        }
                    },
                    Err(e) => match *e {
                        ReadPacketError::ConnectionClosed => {
                            break;
                        }
                        e => {
                            error!("Error during status: {e}");
                            return Err(e.into());
                        }
                    },
                }
            }
        }
        // If the client intends to join the proxy,
        // wait for them to send the `Hello` packet to
        // log their username and uuid, then forward the
        // connection along to the proxy target.
        ConnectionProtocol::Login => {
            let mut conn = conn.login();
            loop {
                match conn.read().await {
                    Ok(p) => match p {
                        ServerboundLoginPacket::Hello(hello) => {
                            let id = if let Some(id) = hello.profile_id {
                                id.to_string()
                            } else {
                                "".to_string()
                            };
                            info!("Player {0} logging in with uuid:{id}", hello.name);
                            tokio::spawn(
                                transfer(
                                    conn.unwrap()?,
                                    "127.0.0.1:25565".to_string(),
                                    intent,
                                    hello,
                                )
                                .map(|r| {
                                    if let Err(e) = r {
                                        error!("Failed to transfer; error={e}");
                                    }
                                }),
                            );
                            break;
                        }
                        _ => {}
                    },
                    Err(e) => match *e {
                        ReadPacketError::ConnectionClosed => {
                            break;
                        }
                        e => {
                            error!("Error during login: {e}");
                            return Err(e.into());
                        }
                    },
                }
            }
        }
        _ => {
            warn!("Client provided weird intent {intent:?}, closing connection");
            return Ok(());
        }
    }

    Ok(())
}

async fn transfer(
    mut inbound: TcpStream,
    proxy_addr: String,
    intent: ClientIntentionPacket,
    hello: ServerboundHelloPacket,
) -> Result<(), Box<dyn Error>> {
    let outbound = TcpStream::connect(proxy_addr).await?;
    let name = hello.name.clone();
    outbound.set_nodelay(true)?;

    // Repeat the intent and hello packet
    // recieved earlier to the proxy target
    let mut outbound_conn: Connection<ClientboundHandshakePacket, ServerboundHandshakePacket> =
        Connection::wrap(outbound);
    outbound_conn.write(intent.get()).await?;

    let mut outbound_conn = outbound_conn.login();
    outbound_conn.write(hello.get()).await?;

    let mut outbound = outbound_conn.unwrap()?;

    // Split the incoming and outgoing connections in
    // halves and handle each pair on separate threads.
    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;
    info!("Player {name} left the game");

    Ok(())
}
