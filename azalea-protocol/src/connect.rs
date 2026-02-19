//! Connect to remote servers/clients.

use std::{
    fmt::{self, Debug, Display},
    io::{self, Cursor},
    marker::PhantomData,
    net::SocketAddr,
};

#[cfg(feature = "online-mode")]
use azalea_auth::{
    game_profile::GameProfile,
    sessionserver::{ClientSessionServerError, ServerSessionServerError},
};
use azalea_crypto::{Aes128CfbDec, Aes128CfbEnc};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use http_proxy_client_async::{HeaderMap, HeaderValue, flow as http_proxy_flow};
use thiserror::Error;
use tokio::{
    io::AsyncWriteExt,
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf, ReuniteError},
    },
};
use tokio_socks::tcp::{Socks4Stream, Socks5Stream};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tracing::{error, info};
#[cfg(feature = "online-mode")]
use uuid::Uuid;

#[cfg(feature = "online-mode")]
use crate::packets::login::ClientboundHello;
use crate::{
    packets::{
        ProtocolPacket,
        config::{ClientboundConfigPacket, ServerboundConfigPacket},
        game::{ClientboundGamePacket, ServerboundGamePacket},
        handshake::{ClientboundHandshakePacket, ServerboundHandshakePacket},
        login::{ClientboundLoginPacket, ServerboundLoginPacket},
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
///         name: "bot".to_owned(),
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

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("{0}")]
    Io(#[from] io::Error),
}

use socks5_impl::protocol::UserKey;

/// The proxy protocol used by [`Proxy`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ProxyProtocol {
    Socks5,
    Socks4,
    Http,
}

impl ProxyProtocol {
    fn scheme(self) -> &'static str {
        match self {
            Self::Socks5 => "socks5",
            Self::Socks4 => "socks4",
            Self::Http => "http",
        }
    }
}

/// A proxy configuration used for both the Minecraft TCP connection and
/// sessionserver authentication requests.
///
/// Supported protocols are SOCKS5, SOCKS4, and HTTP (CONNECT).
#[derive(Clone, Debug)]
pub struct Proxy {
    pub protocol: ProxyProtocol,
    pub addr: SocketAddr,
    pub auth: Option<UserKey>,
}

impl Proxy {
    /// Create a SOCKS5 proxy.
    pub fn new(addr: SocketAddr, auth: Option<UserKey>) -> Self {
        Self::socks5(addr, auth)
    }

    pub fn socks5(addr: SocketAddr, auth: Option<UserKey>) -> Self {
        Self {
            protocol: ProxyProtocol::Socks5,
            addr,
            auth,
        }
    }

    /// Create a SOCKS4 proxy without a user id.
    pub fn socks4(addr: SocketAddr) -> Self {
        Self {
            protocol: ProxyProtocol::Socks4,
            addr,
            auth: None,
        }
    }

    /// Create a SOCKS4 proxy with a user id.
    pub fn socks4_with_user_id(addr: SocketAddr, user_id: impl Into<String>) -> Self {
        Self {
            protocol: ProxyProtocol::Socks4,
            addr,
            auth: Some(UserKey::new(user_id, "")),
        }
    }

    /// Create an HTTP proxy. The optional auth value is used for basic auth.
    pub fn http(addr: SocketAddr, auth: Option<UserKey>) -> Self {
        Self {
            protocol: ProxyProtocol::Http,
            addr,
            auth,
        }
    }

    pub fn protocol(&self) -> ProxyProtocol {
        self.protocol
    }
}
impl Display for Proxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}://", self.protocol.scheme())?;
        if let Some(auth) = &self.auth {
            write!(f, "{auth}@")?;
        }
        write!(f, "{}", self.addr)
    }
}

async fn connect_via_http_proxy(
    stream: TcpStream,
    address: &SocketAddr,
    auth: Option<&UserKey>,
) -> io::Result<TcpStream> {
    let mut headers = HeaderMap::new();
    if let Some(auth) = auth {
        let encoded = BASE64_STANDARD.encode(format!("{}:{}", auth.username, auth.password));
        let auth_value = HeaderValue::from_str(&format!("Basic {encoded}")).map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid HTTP proxy authorization header value: {err}"),
            )
        })?;
        headers.insert("Proxy-Authorization", auth_value);
    }

    // `http-proxy-client-async` writes `host:port`, so IPv6 needs brackets.
    let host = match address {
        SocketAddr::V4(v4) => v4.ip().to_string(),
        SocketAddr::V6(v6) => format!("[{}]", v6.ip()),
    };

    let mut compat_stream = stream.compat();
    // Keep read size minimal to avoid over-reading bytes that belong to the
    // tunneled Minecraft stream.
    let mut read_buf = [0u8; 1];
    let outcome = http_proxy_flow::handshake(
        &mut compat_stream,
        &host,
        address.port(),
        &headers,
        &mut read_buf,
    )
    .await?;

    if outcome.response_parts.status_code / 100 != 2 {
        return Err(io::Error::other(format!(
            "HTTP proxy CONNECT failed: {} {}",
            outcome.response_parts.status_code, outcome.response_parts.reason_phrase
        )));
    }
    if !outcome.data_after_handshake.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "HTTP proxy returned unexpected extra bytes after CONNECT response",
        ));
    }

    Ok(compat_stream.into_inner())
}

#[cfg(feature = "online-mode")]
impl From<Proxy> for reqwest::Proxy {
    fn from(proxy: Proxy) -> Self {
        reqwest::Proxy::all(proxy.to_string())
            .expect("azalea proxies should not fail to parse as reqwest proxies")
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

    /// Create a new connection to the given address and proxy.
    ///
    /// If you're not using a proxy, use [`Self::new`] instead.
    pub async fn new_with_proxy(
        address: &SocketAddr,
        proxy: Proxy,
    ) -> Result<Self, ConnectionError> {
        let stream = match proxy.protocol {
            ProxyProtocol::Socks5 => {
                let stream = if let Some(auth) = proxy.auth.as_ref() {
                    Socks5Stream::connect_with_password(
                        proxy.addr,
                        *address,
                        &auth.username,
                        &auth.password,
                    )
                    .await
                } else {
                    Socks5Stream::connect(proxy.addr, *address).await
                }
                .map_err(io::Error::other)?;
                stream.into_inner()
            }
            ProxyProtocol::Socks4 => {
                let maybe_user_id = proxy
                    .auth
                    .as_ref()
                    .map(|auth| auth.username.as_str())
                    .filter(|user_id| !user_id.is_empty());
                let stream = if let Some(user_id) = maybe_user_id {
                    Socks4Stream::connect_with_userid(proxy.addr, *address, user_id).await
                } else {
                    Socks4Stream::connect(proxy.addr, *address).await
                }
                .map_err(io::Error::other)?;
                stream.into_inner()
            }
            ProxyProtocol::Http => {
                let stream = TcpStream::connect(proxy.addr).await?;
                connect_via_http_proxy(stream, address, proxy.auth.as_ref()).await?
            }
        };

        Self::new_from_stream(stream).await
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
    ///         conn.authenticate(&access_token, &profile.id, e.secret_key, &p, None)
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
    #[cfg(feature = "online-mode")]
    pub async fn authenticate(
        &self,
        access_token: &str,
        uuid: &Uuid,
        private_key: [u8; 16],
        packet: &ClientboundHello,
        sessionserver_proxy: Option<Proxy>,
    ) -> Result<(), ClientSessionServerError> {
        use azalea_auth::sessionserver::{self, SessionServerJoinOpts};

        sessionserver::join(SessionServerJoinOpts {
            access_token,
            public_key: &packet.public_key,
            private_key: &private_key,
            uuid,
            server_id: &packet.server_id,
            proxy: sessionserver_proxy.map(Proxy::into),
        })
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
    #[cfg(feature = "online-mode")]
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