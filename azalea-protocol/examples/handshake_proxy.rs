//! A "simple" server that gets login information and proxies connections.
//! After login all connections are encrypted and Azalea cannot read them.

use std::{error::Error, sync::LazyLock};

use azalea_protocol::{
    connect::Connection,
    packets::{
        handshake::{
            s_intention::ServerboundIntention, ClientboundHandshakePacket,
            ServerboundHandshakePacket,
        },
        login::{s_hello::ServerboundHello, ServerboundLoginPacket},
        status::{
            c_pong_response::ClientboundPongResponse,
            c_status_response::{ClientboundStatusResponse, Players, Version},
            ServerboundStatusPacket,
        },
        ClientIntention, PROTOCOL_VERSION, VERSION_NAME,
    },
    read::ReadPacketError,
};
use futures::FutureExt;
use tokio::{
    io::{self, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::Level;
use tracing::{error, info, warn};

const LISTEN_ADDR: &str = "127.0.0.1:25566";
const PROXY_ADDR: &str = "127.0.0.1:25565";

const PROXY_DESC: &str = "An Azalea Minecraft Proxy";

// String must be formatted like "data:image/png;base64,<data>"
static PROXY_FAVICON: LazyLock<Option<String>> = LazyLock::new(|| None);

static PROXY_VERSION: LazyLock<Version> = LazyLock::new(|| Version {
    name: VERSION_NAME.to_string(),
    protocol: PROTOCOL_VERSION,
});

const PROXY_PLAYERS: Players = Players {
    max: 1,
    online: 0,
    sample: Vec::new(),
};

const PROXY_SECURE_CHAT: Option<bool> = Some(false);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Bind to an address and port
    let listener = TcpListener::bind(LISTEN_ADDR).await?;

    info!("Listening on {LISTEN_ADDR}, proxying to {PROXY_ADDR}");

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
            ServerboundHandshakePacket::Intention(packet) => {
                info!(
                    "New connection from {}, hostname {:?}:{}, version {}, {:?}",
                    ip.ip(),
                    packet.hostname,
                    packet.port,
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
        ClientIntention::Status => {
            let mut conn = conn.status();
            loop {
                match conn.read().await {
                    Ok(p) => match p {
                        ServerboundStatusPacket::StatusRequest(_) => {
                            conn.write(ClientboundStatusResponse {
                                description: PROXY_DESC.into(),
                                favicon: PROXY_FAVICON.clone(),
                                players: PROXY_PLAYERS.clone(),
                                version: PROXY_VERSION.clone(),
                                enforces_secure_chat: PROXY_SECURE_CHAT,
                            })
                            .await?;
                        }
                        ServerboundStatusPacket::PingRequest(p) => {
                            conn.write(ClientboundPongResponse { time: p.time }).await?;
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
        ClientIntention::Login => {
            let mut conn = conn.login();
            loop {
                match conn.read().await {
                    Ok(p) => {
                        if let ServerboundLoginPacket::Hello(hello) = p {
                            info!(
                                "Player \'{0}\' from {1} logging in with uuid: {2}",
                                hello.name,
                                ip.ip(),
                                hello.profile_id.to_string()
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
        ClientIntention::Transfer => {
            warn!("Client attempted to join via transfer")
        }
    }

    Ok(())
}

async fn transfer(
    mut inbound: TcpStream,
    intent: ServerboundIntention,
    hello: ServerboundHello,
) -> Result<(), Box<dyn Error>> {
    let outbound = TcpStream::connect(PROXY_ADDR).await?;
    let name = hello.name.clone();
    outbound.set_nodelay(true)?;

    // Repeat the intent and hello packet
    // received earlier to the proxy target
    let mut outbound_conn: Connection<ClientboundHandshakePacket, ServerboundHandshakePacket> =
        Connection::wrap(outbound);
    outbound_conn.write(intent).await?;

    let mut outbound_conn = outbound_conn.login();
    outbound_conn.write(hello).await?;

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
