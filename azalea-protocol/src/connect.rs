//! Connect to remote servers/clients.

use crate::packets::game::{ClientboundGamePacket, ServerboundGamePacket};
use crate::packets::handshake::{ClientboundHandshakePacket, ServerboundHandshakePacket};
use crate::packets::login::clientbound_hello_packet::ClientboundHelloPacket;
use crate::packets::login::{ClientboundLoginPacket, ServerboundLoginPacket};
use crate::packets::status::{ClientboundStatusPacket, ServerboundStatusPacket};
use crate::packets::ProtocolPacket;
use crate::read::{read_packet, ReadPacketError};
use crate::write::write_packet;
use azalea_auth::game_profile::GameProfile;
use azalea_auth::sessionserver::{ClientSessionServerError, ServerSessionServerError};
use azalea_crypto::{Aes128CfbDec, Aes128CfbEnc};
use bytes::BytesMut;
use log::{error, info};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf, ReuniteError};
use tokio::net::TcpStream;
use uuid::Uuid;

/// The read half of a connection.
pub struct ReadConnection<R: ProtocolPacket> {
    pub read_stream: OwnedReadHalf,
    pub buffer: BytesMut,
    pub compression_threshold: Option<u32>,
    pub dec_cipher: Option<Aes128CfbDec>,
    _reading: PhantomData<R>,
}

/// The write half of a connection.
pub struct WriteConnection<W: ProtocolPacket> {
    pub write_stream: OwnedWriteHalf,
    pub compression_threshold: Option<u32>,
    pub enc_cipher: Option<Aes128CfbEnc>,
    _writing: PhantomData<W>,
}

/// A connection that can read and write packets.
///
/// # Examples
///
/// Join an offline-mode server and go through the handshake.
/// ```rust,no_run
/// use azalea_protocol::{
///     resolver,
///     connect::Connection,
///     packets::{
///         ConnectionProtocol, PROTOCOL_VERSION,
///         login::{
///             ClientboundLoginPacket,
///             serverbound_hello_packet::ServerboundHelloPacket,
///             serverbound_key_packet::{ServerboundKeyPacket, NonceOrSaltSignature}
///         },
///         handshake::client_intention_packet::ClientIntentionPacket
///     }
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let resolved_address = resolver::resolve_address(&"localhost".try_into().unwrap()).await?;
///     let mut conn = Connection::new(&resolved_address).await?;
///
///     // handshake
///     conn.write(
///         ClientIntentionPacket {
///             protocol_version: PROTOCOL_VERSION,
///             hostname: resolved_address.ip().to_string(),
///             port: resolved_address.port(),
///             intention: ConnectionProtocol::Login,
///         }
///         .get(),
///     )
///     .await?;
///
///     let mut conn = conn.login();
///
///     // login
///     conn.write(
///         ServerboundHelloPacket {
///             username: "bot".to_string(),
///             public_key: None,
///             profile_id: None,
///         }
///         .get(),
///     )
///     .await?;
///
///     let (conn, game_profile) = loop {
///         let packet = conn.read().await?;
///         match packet {
///             ClientboundLoginPacket::Hello(p) => {
///                 let e = azalea_crypto::encrypt(&p.public_key, &p.nonce).unwrap();
///
///                 conn.write(
///                     ServerboundKeyPacket {
///                         key_bytes: e.encrypted_public_key,
///                         nonce_or_salt_signature: NonceOrSaltSignature::Nonce(e.encrypted_nonce),
///                     }
///                     .get(),
///                 )
///                 .await?;
///                 conn.set_encryption_key(e.secret_key);
///             }
///             ClientboundLoginPacket::LoginCompression(p) => {
///                 conn.set_compression_threshold(p.compression_threshold);
///             }
///             ClientboundLoginPacket::GameProfile(p) => {
///                 break (conn.game(), p.game_profile);
///             }
///             ClientboundLoginPacket::LoginDisconnect(p) => {
///                 eprintln!("login disconnect: {}", p.reason);
///                 return Err("login disconnect".into());
///             }
///             ClientboundLoginPacket::CustomQuery(p) => {}
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

impl<R> ReadConnection<R>
where
    R: ProtocolPacket + Debug,
{
    /// Read a packet from the stream.
    pub async fn read(&mut self) -> Result<R, Box<ReadPacketError>> {
        read_packet::<R, _>(
            &mut self.read_stream,
            &mut self.buffer,
            self.compression_threshold,
            &mut self.dec_cipher,
        )
        .await
    }
}
impl<W> WriteConnection<W>
where
    W: ProtocolPacket + Debug,
{
    /// Write a packet to the server.
    pub async fn write(&mut self, packet: W) -> std::io::Result<()> {
        if let Err(e) = write_packet(
            &packet,
            &mut self.write_stream,
            self.compression_threshold,
            &mut self.enc_cipher,
        )
        .await
        {
            // detect broken pipe
            if e.kind() == std::io::ErrorKind::BrokenPipe {
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
    pub async fn shutdown(&mut self) -> std::io::Result<()> {
        self.write_stream.shutdown().await
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

    /// Write a packet to the other side of the connection.
    pub async fn write(&mut self, packet: W) -> std::io::Result<()> {
        self.writer.write(packet).await
    }

    /// Split the reader and writer into two objects. This doesn't allocate.
    #[must_use]
    pub fn into_split(self) -> (ReadConnection<R>, WriteConnection<W>) {
        (self.reader, self.writer)
    }
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

impl Connection<ClientboundHandshakePacket, ServerboundHandshakePacket> {
    /// Create a new connection to the given address.
    pub async fn new(address: &SocketAddr) -> Result<Self, ConnectionError> {
        let stream = TcpStream::connect(address).await?;

        // enable tcp_nodelay
        stream.set_nodelay(true)?;

        let (read_stream, write_stream) = stream.into_split();

        Ok(Connection {
            reader: ReadConnection {
                read_stream,
                buffer: BytesMut::new(),
                compression_threshold: None,
                dec_cipher: None,
                _reading: PhantomData,
            },
            writer: WriteConnection {
                write_stream,
                compression_threshold: None,
                enc_cipher: None,
                _writing: PhantomData,
            },
        })
    }

    /// Change our state from handshake to login. This is the state that is used
    /// for logging in.
    #[must_use]
    pub fn login(self) -> Connection<ClientboundLoginPacket, ServerboundLoginPacket> {
        Connection::from(self)
    }

    /// Change our state from handshake to status. This is the state that is
    /// used for pinging the server.
    #[must_use]
    pub fn status(self) -> Connection<ClientboundStatusPacket, ServerboundStatusPacket> {
        Connection::from(self)
    }
}

impl Connection<ClientboundLoginPacket, ServerboundLoginPacket> {
    /// Set our compression threshold, i.e. the maximum size that a packet is
    /// allowed to be without getting compressed. If you set it to less than 0
    /// then compression gets disabled.
    pub fn set_compression_threshold(&mut self, threshold: i32) {
        // if you pass a threshold of less than 0, compression is disabled
        if threshold >= 0 {
            self.reader.compression_threshold = Some(threshold as u32);
            self.writer.compression_threshold = Some(threshold as u32);
        } else {
            self.reader.compression_threshold = None;
            self.writer.compression_threshold = None;
        }
    }

    /// Set the encryption key that is used to encrypt and decrypt packets. It's
    /// the same for both reading and writing.
    pub fn set_encryption_key(&mut self, key: [u8; 16]) {
        let (enc_cipher, dec_cipher) = azalea_crypto::create_cipher(&key);
        self.reader.dec_cipher = Some(dec_cipher);
        self.writer.enc_cipher = Some(enc_cipher);
    }

    /// Change our state from login to game. This is the state that's used when
    /// you're actually in the game.
    #[must_use]
    pub fn game(self) -> Connection<ClientboundGamePacket, ServerboundGamePacket> {
        Connection::from(self)
    }

    /// Authenticate with Minecraft's servers, which is required to join
    /// online-mode servers. This must happen when you get a
    /// `ClientboundLoginPacket::Hello` packet.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azalea_auth::AuthResult;
    /// use azalea_protocol::connect::Connection;
    /// use azalea_protocol::packets::login::{
    ///     ClientboundLoginPacket,
    ///     serverbound_key_packet::{ServerboundKeyPacket, NonceOrSaltSignature}
    /// };
    /// use uuid::Uuid;
    /// # use azalea_protocol::ServerAddress;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let AuthResult { access_token, profile } = azalea_auth::auth(
    ///     "example@example.com",
    ///     azalea_auth::AuthOpts::default()
    /// ).await.expect("Couldn't authenticate");
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
    ///         let e = azalea_crypto::encrypt(&p.public_key, &p.nonce).unwrap();
    ///         conn.authenticate(
    ///             &access_token,
    ///             &profile.id,
    ///             e.secret_key,
    ///             &p
    ///         ).await?;
    ///         conn.write(
    ///             ServerboundKeyPacket {
    ///                 key_bytes: e.encrypted_public_key,
    ///                 nonce_or_salt_signature: NonceOrSaltSignature::Nonce(e.encrypted_nonce),
    ///             }.get()
    ///         ).await?;
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
        packet: &ClientboundHelloPacket,
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
    /// Change our state from handshake to login. This is the state that is used
    /// for logging in.
    #[must_use]
    pub fn login(self) -> Connection<ServerboundLoginPacket, ClientboundLoginPacket> {
        Connection::from(self)
    }

    /// Change our state from handshake to status. This is the state that is
    /// used for pinging the server.
    #[must_use]
    pub fn status(self) -> Connection<ServerboundStatusPacket, ClientboundStatusPacket> {
        Connection::from(self)
    }
}

impl Connection<ServerboundLoginPacket, ClientboundLoginPacket> {
    /// Set our compression threshold, i.e. the maximum size that a packet is
    /// allowed to be without getting compressed. If you set it to less than 0
    /// then compression gets disabled.
    pub fn set_compression_threshold(&mut self, threshold: i32) {
        // if you pass a threshold of less than 0, compression is disabled
        if threshold >= 0 {
            self.reader.compression_threshold = Some(threshold as u32);
            self.writer.compression_threshold = Some(threshold as u32);
        } else {
            self.reader.compression_threshold = None;
            self.writer.compression_threshold = None;
        }
    }

    /// Set the encryption key that is used to encrypt and decrypt packets. It's
    /// the same for both reading and writing.
    pub fn set_encryption_key(&mut self, key: [u8; 16]) {
        let (enc_cipher, dec_cipher) = azalea_crypto::create_cipher(&key);
        self.reader.dec_cipher = Some(dec_cipher);
        self.writer.enc_cipher = Some(enc_cipher);
    }

    /// Change our state from login to game. This is the state that's used when
    /// the client is actually in the game.
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
                read_stream: connection.reader.read_stream,
                buffer: connection.reader.buffer,
                compression_threshold: connection.reader.compression_threshold,
                dec_cipher: connection.reader.dec_cipher,
                _reading: PhantomData,
            },
            writer: WriteConnection {
                compression_threshold: connection.writer.compression_threshold,
                write_stream: connection.writer.write_stream,
                enc_cipher: connection.writer.enc_cipher,
                _writing: PhantomData,
            },
        }
    }

    /// Convert an existing `TcpStream` into a `Connection`. Useful for servers.
    pub fn wrap(stream: TcpStream) -> Connection<R1, W1> {
        let (read_stream, write_stream) = stream.into_split();

        Connection {
            reader: ReadConnection {
                read_stream,
                buffer: BytesMut::new(),
                compression_threshold: None,
                dec_cipher: None,
                _reading: PhantomData,
            },
            writer: WriteConnection {
                write_stream,
                compression_threshold: None,
                enc_cipher: None,
                _writing: PhantomData,
            },
        }
    }

    /// Convert from a `Connection` into a `TcpStream`. Useful for servers.
    pub fn unwrap(self) -> Result<TcpStream, ReuniteError> {
        self.reader.read_stream.reunite(self.writer.write_stream)
    }
}
