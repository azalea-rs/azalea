//! A "simple" server that gets login information and proxies connections.
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
        ConnectionProtocol, PROTOCOL_VERSION,
    },
    read::ReadPacketError,
};
use futures::FutureExt;
use log::{error, info, warn};
use once_cell::sync::Lazy;
use std::error::Error;
use tokio::{
    io::{self, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::Level;

const LISTEN_ADDR: &str = "127.0.0.1:25566";
const PROXY_ADDR: &str = "127.0.0.1:25565";

const PROXY_DESC: &str = "An Azalea Minecraft Proxy";

// String must be formatted like "data:image/png;base64,<data>"
static PROXY_FAVICON: Lazy<Option<String>> = Lazy::new(|| None);

static PROXY_VERSION: Lazy<Version> = Lazy::new(|| Version {
    name: "1.19.3".to_string(),
    protocol: PROTOCOL_VERSION as i32,
});

const PROXY_PLAYERS: Players = Players {
    max: 1,
    online: 0,
    sample: Vec::new(),
};

const PROXY_PREVIEWS_CHAT: Option<bool> = Some(false);
const PROXY_SECURE_CHAT: Option<bool> = Some(false);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Bind to an address and port
    let listener = TcpListener::bind(LISTEN_ADDR).await?;
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
        Ok(packet) => match packet {
            ServerboundHandshakePacket::ClientIntention(packet) => {
                info!(
                    "New connection: {0}, Version {1}, {2:?}",
                    ip.ip(),
                    packet.protocol_version,
                    packet.intention
                );
                packet
            }
        },
        Err(e) => {
            let e = e.into();
            warn!("Error during intent: {e}");
            return Err(e);
        }
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
                                    description: PROXY_DESC.into(),
                                    favicon: PROXY_FAVICON.clone(),
                                    players: PROXY_PLAYERS.clone(),
                                    version: PROXY_VERSION.clone(),
                                    previews_chat: PROXY_PREVIEWS_CHAT,
                                    enforces_secure_chat: PROXY_SECURE_CHAT,
                                }
                                .get(),
                            )
                            .await?;
                        }
                        ServerboundStatusPacket::PingRequest(p) => {
                            conn.write(ClientboundPongResponsePacket { time: p.time }.get())
                                .await?;
                            break;
                        }
                    },
                    Err(e) => match *e {
                        ReadPacketError::ConnectionClosed => {
                            break;
                        }
                        e => {
                            warn!("Error during status: {e}");
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
                    Ok(p) => {
                        if let ServerboundLoginPacket::Hello(hello) = p {
                            info!(
                                "Player \'{0}\' from {1} logging in with uuid: {2}",
                                hello.username,
                                ip.ip(),
                                if let Some(id) = hello.profile_id {
                                    id.to_string()
                                } else {
                                    String::new()
                                }
                            );

                            tokio::spawn(transfer(conn.unwrap()?, intent, hello).map(|r| {
                                if let Err(e) = r {
                                    error!("Failed to proxy: {e}");
                                }
                            }));

                            break;
                        }
                    }
                    Err(e) => match *e {
                        ReadPacketError::ConnectionClosed => {
                            break;
                        }
                        e => {
                            warn!("Error during login: {e}");
                            return Err(e.into());
                        }
                    },
                }
            }
        }
        _ => {
            warn!("Client provided weird intent: {:?}", intent.intention);
        }
    }

    Ok(())
}

async fn transfer(
    mut inbound: TcpStream,
    intent: ClientIntentionPacket,
    hello: ServerboundHelloPacket,
) -> Result<(), Box<dyn Error>> {
    let outbound = TcpStream::connect(PROXY_ADDR).await?;
    let name = hello.username.clone();
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
    info!("Player \'{name}\' left the game");

    Ok(())
}
