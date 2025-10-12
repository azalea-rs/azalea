//! Connect to remote servers/clients.

use std::{
    fmt::{self, Debug, Display},
    io::{self, Cursor},
    marker::PhantomData,
    net::SocketAddr,
};

use azalea_auth::{
    game_profile::GameProfile,
    sessionserver::{ClientSessionServerError, ServerSessionServerError},
};
use azalea_crypto::{Aes128CfbDec, Aes128CfbEnc};
use thiserror::Error;
use tokio::{
    io::{AsyncWriteExt, BufStream},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf, ReuniteError},
    },
};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    packets::{
        ProtocolPacket,
        config::{ClientboundConfigPacket, ServerboundConfigPacket},
        game::{ClientboundGamePacket, ServerboundGamePacket},
        handshake::{ClientboundHandshakePacket, ServerboundHandshakePacket},
        login::{ClientboundLoginPacket, ServerboundLoginPacket, c_hello::ClientboundHello},
        status::{ClientboundStatusPacket, ServerboundStatusPacket},
    },
    read::{ReadPacketError, deserialize_packet, read_raw_packet, try_read_raw_packet},
    write::{serialize_packet, write_raw_packet},
};

pub struct RawReadConnection {
    pub read_stream: OwnedReadHalf,
    pub buffer: Cursor<Vec<u8>>,
    pub compression_threshold: Option<u32>,
    pub dec_cipher: Option<Aes128CfbDec>,
}

pub struct RawWriteConnection {
    pub write_stream: OwnedWriteHalf,
    pub compression_threshold: Option<u32>,
    pub enc_cipher: Option<Aes128CfbEnc>,
}

/// The read half of a connection.
pub struct ReadConnection<R: ProtocolPacket> {
    pub raw: RawReadConnection,
    _reading: PhantomData<R>,
}

/// The write half of a connection.
pub struct WriteConnection<W: ProtocolPacket> {
    pub raw: RawWriteConnection,
    _writing: PhantomData<W>,
}

/// A connection that can read and write packets.
///
/// # Examples
///
/// Join an offline-mode server and go through the handshake.
/// ```rust,no_run
/// use azalea_protocol::{
///     connect::Connection,
///     packets::{
///         self, ClientIntention, PROTOCOL_VERSION,
///         handshake::ServerboundIntention,
///         login::{ClientboundLoginPacket, ServerboundHello, ServerboundKey},
///     },
///     resolver,
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let resolved_address = resolver::resolve_address(&"localhost".try_into().unwrap()).await?;
///     let mut conn = Connection::new(&resolved_address).await?;
///
///     // handshake
///     conn.write(ServerboundIntention {
///         protocol_version: PROTOCOL_VERSION,
///         hostname: resolved_address.ip().to_string(),
///         port: resolved_address.port(),
///         intention: ClientIntention::Login,
///     })
///     .await?;
///
///     let mut conn = conn.login();
///
///     // login
///     conn.write(ServerboundHello {
///         name: "bot".to_string(),
///         profile_id: uuid::Uuid::nil(),
///     })
///     .await?;
///
///     let (conn, game_profile) = loop {
///         let packet = conn.read().await?;
///         match packet {
///             ClientboundLoginPacket::Hello(p) => {
///                 let e = azalea_crypto::encrypt(&p.public_key, &p.challenge).unwrap();
///
///                 conn.write(ServerboundKey {
///                     key_bytes: e.encrypted_public_key,
///                     encrypted_challenge: e.encrypted_challenge,
///                 })
///                 .await?;
///                 conn.set_encryption_key(e.secret_key);
///             }
///             ClientboundLoginPacket::LoginCompression(p) => {
///                 conn.set_compression_threshold(p.compression_threshold);
///             }
///             ClientboundLoginPacket::LoginFinished(p) => {
///                 break (conn.config(), p.game_profile);
///             }
///             ClientboundLoginPacket::LoginDisconnect(p) => {
///                 eprintln!("login disconnect: {}", p.reason);
///                 return Err("login disconnect".into());
///             }
///             ClientboundLoginPacket::CustomQuery(p) => {}
///             ClientboundLoginPacket::CookieRequest(p) => {
///                 conn.write(packets::login::ServerboundCookieResponse {
///                     key: p.key,
///                     payload: None,
///                 })
///                 .await?;
///             }
///         }
///     };
///
///     Ok(())
/// }
/// ```
pub struct Connection<R: ProtocolPacket, W: ProtocolPacket> {
    pub reader: ReadConnection<R>,
    pub writer: WriteConnection<W>,
}

impl RawReadConnection {
    pub async fn read(&mut self) -> Result<Box<[u8]>, Box<ReadPacketError>> {
        read_raw_packet::<_>(
            &mut self.read_stream,
            &mut self.buffer,
            self.compression_threshold,
            &mut self.dec_cipher,
        )
        .await
    }

    pub fn try_read(&mut self) -> Result<Option<Box<[u8]>>, Box<ReadPacketError>> {
        try_read_raw_packet::<_>(
            &mut self.read_stream,
            &mut self.buffer,
            self.compression_threshold,
            &mut self.dec_cipher,
        )
    }
}

impl RawWriteConnection {
    pub async fn write(&mut self, packet: &[u8]) -> io::Result<()> {
        if let Err(e) = write_raw_packet(
            packet,
            &mut self.write_stream,
            self.compression_threshold,
            &mut self.enc_cipher,
        )
        .await
        {
            // detect broken pipe
            if e.kind() == io::ErrorKind::BrokenPipe {
                info!("Broken pipe, shutting down connection.");
                if let Err(e) = self.shutdown().await {
                    error!("Couldn't shut down: {}", e);
                }
            }
            return Err(e);
        }
        Ok(())
    }

    /// End the connection.
    pub async fn shutdown(&mut self) -> io::Result<()> {
        self.write_stream.shutdown().await
    }
}

impl<R> ReadConnection<R>
where
    R: ProtocolPacket + Debug,
{
    /// Read a packet from the stream.
    pub async fn read(&mut self) -> Result<R, Box<ReadPacketError>> {
        let raw_packet = self.raw.read().await?;
        deserialize_packet(&mut Cursor::new(&raw_packet))
    }

    /// Try to read a packet from the stream, or return Ok(None) if there's no
    /// packet.
    pub fn try_read(&mut self) -> Result<Option<R>, Box<ReadPacketError>> {
        let Some(raw_packet) = self.raw.try_read()? else {
            return Ok(None);
        };
        Ok(Some(deserialize_packet(&mut Cursor::new(&raw_packet))?))
    }
}
impl<W> WriteConnection<W>
where
    W: ProtocolPacket + Debug,
{
    /// Write a packet to the server.
    pub async fn write(&mut self, packet: W) -> io::Result<()> {
        self.raw.write(&serialize_packet(&packet).unwrap()).await
    }

    /// End the connection.
    pub async fn shutdown(&mut self) -> io::Result<()> {
        self.raw.shutdown().await
    }
}

impl<R, W> Connection<R, W>
where
    R: ProtocolPacket + Debug,
    W: ProtocolPacket + Debug,
{
    /// Read a packet from the other side of the connection.
    pub async fn read(&mut self) -> Result<R, Box<ReadPacketError>> {
        self.reader.read().await
    }

    /// Try to read a packet from the other side of the connection, or return
    /// Ok(None) if there's no packet to read.
    pub fn try_read(&mut self) -> Result<Option<R>, Box<ReadPacketError>> {
        self.reader.try_read()
    }

    /// Write a packet to the other side of the connection.
    pub async fn write(&mut self, packet: impl crate::packets::Packet<W>) -> io::Result<()> {
        let packet = packet.into_variant();
        self.writer.write(packet).await
    }

    /// Split the reader and writer into two objects.
    ///
    /// This doesn't allocate.
    #[must_use]
    pub fn into_split(self) -> (ReadConnection<R>, WriteConnection<W>) {
        (self.reader, self.writer)
    }

    /// Split the reader and writer into the state-agnostic
    /// [`RawReadConnection`] and [`RawWriteConnection`] types.
    ///
    /// This is meant to help with some types of proxies.
    #[must_use]
    pub fn into_split_raw(self) -> (RawReadConnection, RawWriteConnection) {
        (self.reader.raw, self.writer.raw)
    }
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("{0}")]
    Io(#[from] io::Error),
}

use socks5_impl::protocol::UserKey;

/// An address and authentication method for connecting to a Socks5 proxy.
#[derive(Debug, Clone)]
pub struct Proxy {
    pub addr: SocketAddr,
    pub auth: Option<UserKey>,
}

impl Proxy {
    pub fn new(addr: SocketAddr, auth: Option<UserKey>) -> Self {
        Self { addr, auth }
    }
}
impl Display for Proxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "socks5://")?;
        if let Some(auth) = &self.auth {
            write!(f, "{auth}@")?;
        }
        write!(f, "{}", self.addr)
    }
}

impl Connection<ClientboundHandshakePacket, ServerboundHandshakePacket> {
    /// Create a new connection to the given address.
    pub async fn new(address: &SocketAddr) -> Result<Self, ConnectionError> {
        let stream = TcpStream::connect(address).await?;

        // enable tcp_nodelay
        stream.set_nodelay(true)?;

        Self::new_from_stream(stream).await
    }

    /// Create a new connection to the given address and Socks5 proxy.
    ///
    /// If you're not using a proxy, use [`Self::new`] instead.
    pub async fn new_with_proxy(
        address: &SocketAddr,
        proxy: Proxy,
    ) -> Result<Self, ConnectionError> {
        let proxy_stream = TcpStream::connect(proxy.addr).await?;
        let mut stream = BufStream::new(proxy_stream);

        let _ = socks5_impl::client::connect(&mut stream, address, proxy.auth)
            .await
            .map_err(io::Error::other)?;

        Self::new_from_stream(stream.into_inner()).await
    }

    /// Create a new connection from an existing stream.
    ///
    /// Useful if you want to set custom options on the stream. Otherwise, just
    /// use [`Self::new`].
    pub async fn new_from_stream(stream: TcpStream) -> Result<Self, ConnectionError> {
        let (read_stream, write_stream) = stream.into_split();

        Ok(Connection {
            reader: ReadConnection {
                raw: RawReadConnection {
                    read_stream,
                    buffer: Cursor::new(Vec::new()),
                    compression_threshold: None,
                    dec_cipher: None,
                },
                _reading: PhantomData,
            },
            writer: WriteConnection {
                raw: RawWriteConnection {
                    write_stream,
                    compression_threshold: None,
                    enc_cipher: None,
                },
                _writing: PhantomData,
            },
        })
    }

    /// Change our state from handshake to login.
    ///
    /// This is the state that is used for logging in.
    #[must_use]
    pub fn login(self) -> Connection<ClientboundLoginPacket, ServerboundLoginPacket> {
        Connection::from(self)
    }

    /// Change our state from handshake to status.
    ///
    /// This is the state that is used for pinging the server.
    #[must_use]
    pub fn status(self) -> Connection<ClientboundStatusPacket, ServerboundStatusPacket> {
        Connection::from(self)
    }
}

impl Connection<ClientboundLoginPacket, ServerboundLoginPacket> {
    /// Set our compression threshold, i.e. the maximum size that a packet is
    /// allowed to be without getting compressed.
    ///
    /// Setting it to 0 means every packet will be compressed. If you set it to
    /// less than 0 then compression is disabled.
    pub fn set_compression_threshold(&mut self, threshold: i32) {
        // if you pass a threshold of less than 0, compression is disabled
        if threshold >= 0 {
            self.reader.raw.compression_threshold = Some(threshold as u32);
            self.writer.raw.compression_threshold = Some(threshold as u32);
        } else {
            self.reader.raw.compression_threshold = None;
            self.writer.raw.compression_threshold = None;
        }
    }

    /// Set the encryption key that is used to encrypt and decrypt packets.
    ///
    /// It's the same for both reading and writing.
    pub fn set_encryption_key(&mut self, key: [u8; 16]) {
        let (enc_cipher, dec_cipher) = azalea_crypto::create_cipher(&key);
        self.reader.raw.dec_cipher = Some(dec_cipher);
        self.writer.raw.enc_cipher = Some(enc_cipher);
    }

    /// Change our state from login to configuration.
    ///
    /// This is the state where the server sends us the registries and the
    /// resource pack.
    #[must_use]
    pub fn config(self) -> Connection<ClientboundConfigPacket, ServerboundConfigPacket> {
        Connection::from(self)
    }

    /// Authenticate with Minecraft's servers, which is required to join
    /// online-mode servers.
    ///
    /// This must happen when you get a `ClientboundLoginPacket::Hello` packet.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azalea_auth::AuthResult;
    /// use azalea_protocol::{
    ///     connect::Connection,
    ///     packets::login::{ClientboundLoginPacket, ServerboundKey},
    /// };
    /// use uuid::Uuid;
    /// # use azalea_protocol::ServerAddress;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let AuthResult {
    ///     access_token,
    ///     profile,
    /// } = azalea_auth::auth("example@example.com", azalea_auth::AuthOpts::default())
    ///     .await
    ///     .expect("Couldn't authenticate");
    /// #
    /// # let address = ServerAddress::try_from("example@example.com").unwrap();
    /// # let resolved_address = azalea_protocol::resolver::resolve_address(&address).await?;
    ///
    /// let mut conn = Connection::new(&resolved_address).await?;
    ///
    /// // transition to the login state, in a real program we would have done a handshake first
    /// let mut conn = conn.login();
    ///
    /// match conn.read().await? {
    ///     ClientboundLoginPacket::Hello(p) => {
    ///         // tell Mojang we're joining the server & enable encryption
    ///         let e = azalea_crypto::encrypt(&p.public_key, &p.challenge).unwrap();
    ///         conn.authenticate(&access_token, &profile.id, e.secret_key, &p)
    ///             .await?;
    ///         conn.write(ServerboundKey {
    ///             key_bytes: e.encrypted_public_key,
    ///             encrypted_challenge: e.encrypted_challenge,
    ///         })
    ///         .await?;
    ///         conn.set_encryption_key(e.secret_key);
    ///     }
    ///     _ => {}
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn authenticate(
        &self,
        access_token: &str,
        uuid: &Uuid,
        private_key: [u8; 16],
        packet: &ClientboundHello,
    ) -> Result<(), ClientSessionServerError> {
        azalea_auth::sessionserver::join(
            access_token,
            &packet.public_key,
            &private_key,
            uuid,
            &packet.server_id,
        )
        .await
    }
}

impl Connection<ServerboundHandshakePacket, ClientboundHandshakePacket> {
    /// Change our state from handshake to login.
    ///
    /// This is the state that is used while negotiating encryption and
    /// authenticating with Mojang.
    #[must_use]
    pub fn login(self) -> Connection<ServerboundLoginPacket, ClientboundLoginPacket> {
        Connection::from(self)
    }

    /// Change our state from handshake to status.
    ///
    /// This is the state that is used for pinging the server.
    #[must_use]
    pub fn status(self) -> Connection<ServerboundStatusPacket, ClientboundStatusPacket> {
        Connection::from(self)
    }
}

impl Connection<ServerboundLoginPacket, ClientboundLoginPacket> {
    /// Set our compression threshold, i.e. the maximum size that a packet is
    /// allowed to be without getting compressed.
    ///
    /// If you set it to less than 0 then compression gets disabled.
    pub fn set_compression_threshold(&mut self, threshold: i32) {
        // if you pass a threshold of less than 0, compression is disabled
        if threshold >= 0 {
            self.reader.raw.compression_threshold = Some(threshold as u32);
            self.writer.raw.compression_threshold = Some(threshold as u32);
        } else {
            self.reader.raw.compression_threshold = None;
            self.writer.raw.compression_threshold = None;
        }
    }

    /// Set the encryption key that is used to encrypt and decrypt packets.
    ///
    /// It's the same for both reading and writing.
    pub fn set_encryption_key(&mut self, key: [u8; 16]) {
        let (enc_cipher, dec_cipher) = azalea_crypto::create_cipher(&key);
        self.reader.raw.dec_cipher = Some(dec_cipher);
        self.writer.raw.enc_cipher = Some(enc_cipher);
    }

    /// Change our state from login to game.
    ///
    /// This is the state that's used when the client is actually in the game.
    #[must_use]
    pub fn game(self) -> Connection<ServerboundGamePacket, ClientboundGamePacket> {
        Connection::from(self)
    }

    /// Verify connecting clients have authenticated with Minecraft's servers.
    /// This must happen after the client sends a `ServerboundLoginPacket::Key`
    /// packet.
    pub async fn authenticate(
        &self,
        username: &str,
        public_key: &[u8],
        private_key: &[u8; 16],
        ip: Option<&str>,
    ) -> Result<GameProfile, ServerSessionServerError> {
        azalea_auth::sessionserver::serverside_auth(username, public_key, private_key, ip).await
    }

    /// Change our state back to configuration.
    #[must_use]
    pub fn config(self) -> Connection<ServerboundConfigPacket, ClientboundConfigPacket> {
        Connection::from(self)
    }
}

impl Connection<ServerboundConfigPacket, ClientboundConfigPacket> {
    /// Change our state from configuration to game.
    ///
    /// This is the state that's used when the client is actually in the world.
    #[must_use]
    pub fn game(self) -> Connection<ServerboundGamePacket, ClientboundGamePacket> {
        Connection::from(self)
    }
}

impl Connection<ClientboundConfigPacket, ServerboundConfigPacket> {
    /// Change our state from configuration to game.
    ///
    /// This is the state that's used when the client is actually in the world.
    #[must_use]
    pub fn game(self) -> Connection<ClientboundGamePacket, ServerboundGamePacket> {
        Connection::from(self)
    }
}

impl Connection<ClientboundGamePacket, ServerboundGamePacket> {
    /// Change our state back to configuration.
    #[must_use]
    pub fn config(self) -> Connection<ClientboundConfigPacket, ServerboundConfigPacket> {
        Connection::from(self)
    }
}
impl Connection<ServerboundGamePacket, ClientboundGamePacket> {
    /// Change our state back to configuration.
    #[must_use]
    pub fn config(self) -> Connection<ServerboundConfigPacket, ClientboundConfigPacket> {
        Connection::from(self)
    }
}

// rust doesn't let us implement From because allegedly it conflicts with
// `core`'s "impl<T> From<T> for T" so we do this instead
impl<R1, W1> Connection<R1, W1>
where
    R1: ProtocolPacket + Debug,
    W1: ProtocolPacket + Debug,
{
    /// Creates a `Connection` of a type from a `Connection` of another type.
    /// Useful for servers or custom packets.
    #[must_use]
    pub fn from<R2, W2>(connection: Connection<R1, W1>) -> Connection<R2, W2>
    where
        R2: ProtocolPacket + Debug,
        W2: ProtocolPacket + Debug,
    {
        Connection {
            reader: ReadConnection {
                raw: connection.reader.raw,
                _reading: PhantomData,
            },
            writer: WriteConnection {
                raw: connection.writer.raw,
                _writing: PhantomData,
            },
        }
    }

    /// Convert an existing `TcpStream` into a `Connection`. Useful for servers.
    pub fn wrap(stream: TcpStream) -> Connection<R1, W1> {
        let (read_stream, write_stream) = stream.into_split();

        Connection {
            reader: ReadConnection {
                raw: RawReadConnection {
                    read_stream,
                    buffer: Cursor::new(Vec::new()),
                    compression_threshold: None,
                    dec_cipher: None,
                },
                _reading: PhantomData,
            },
            writer: WriteConnection {
                raw: RawWriteConnection {
                    write_stream,
                    compression_threshold: None,
                    enc_cipher: None,
                },
                _writing: PhantomData,
            },
        }
    }

    /// Convert from a `Connection` into a `TcpStream`. Useful for servers.
    pub fn unwrap(self) -> Result<TcpStream, ReuniteError> {
        self.reader
            .raw
            .read_stream
            .reunite(self.writer.raw.write_stream)
    }
}
