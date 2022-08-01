//! parse sending and receiving packets with a server.

use crate::packets::game::{ClientboundGamePacket, ServerboundGamePacket};
use crate::packets::handshake::{ClientboundHandshakePacket, ServerboundHandshakePacket};
use crate::packets::login::{ClientboundLoginPacket, ServerboundLoginPacket};
use crate::packets::status::{ClientboundStatusPacket, ServerboundStatusPacket};
use crate::packets::ProtocolPacket;
use crate::read::{read_packet, ReadPacketError};
use crate::write::write_packet;
use crate::ServerIpAddress;
use azalea_crypto::{Aes128CfbDec, Aes128CfbEnc};
use std::fmt::Debug;
use std::marker::PhantomData;
use tokio::net::TcpStream;

pub struct Connection<R: ProtocolPacket, W: ProtocolPacket> {
    /// The buffered writer
    pub stream: TcpStream,
    pub compression_threshold: Option<u32>,
    pub enc_cipher: Option<Aes128CfbEnc>,
    pub dec_cipher: Option<Aes128CfbDec>,
    _reading: PhantomData<R>,
    _writing: PhantomData<W>,
}

impl<R, W> Connection<R, W>
where
    R: ProtocolPacket + Debug,
    W: ProtocolPacket + Debug,
{
    pub async fn read(&mut self) -> Result<R, ReadPacketError> {
        read_packet::<R, _>(
            &mut self.stream,
            self.compression_threshold,
            &mut self.dec_cipher,
        )
        .await
    }

    /// Write a packet to the server
    pub async fn write(&mut self, packet: W) {
        write_packet(
            packet,
            &mut self.stream,
            self.compression_threshold,
            &mut self.enc_cipher,
        )
        .await;
    }
}

impl Connection<ClientboundHandshakePacket, ServerboundHandshakePacket> {
    pub async fn new(address: &ServerIpAddress) -> Result<Self, String> {
        let ip = address.ip;
        let port = address.port;

        let stream = TcpStream::connect(format!("{}:{}", ip, port))
            .await
            .map_err(|_| "Failed to connect to server")?;

        // enable tcp_nodelay
        stream
            .set_nodelay(true)
            .expect("Error enabling tcp_nodelay");

        Ok(Connection {
            stream,
            compression_threshold: None,
            enc_cipher: None,
            dec_cipher: None,
            _reading: PhantomData,
            _writing: PhantomData,
        })
    }

    pub fn login(self) -> Connection<ClientboundLoginPacket, ServerboundLoginPacket> {
        Connection::from(self)
    }

    pub fn status(self) -> Connection<ClientboundStatusPacket, ServerboundStatusPacket> {
        Connection::from(self)
    }
}

impl Connection<ClientboundLoginPacket, ServerboundLoginPacket> {
    pub fn set_compression_threshold(&mut self, threshold: i32) {
        // if you pass a threshold of 0 or less, compression is disabled
        if threshold > 0 {
            self.compression_threshold = Some(threshold as u32);
        } else {
            self.compression_threshold = None;
        }
    }

    pub fn set_encryption_key(&mut self, key: [u8; 16]) {
        // minecraft has a cipher decoder and encoder, i don't think it matters though?
        let (enc_cipher, dec_cipher) = azalea_crypto::create_cipher(&key);
        self.enc_cipher = Some(enc_cipher);
        self.dec_cipher = Some(dec_cipher);
    }

    pub fn game(self) -> Connection<ClientboundGamePacket, ServerboundGamePacket> {
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
    fn from<R2, W2>(connection: Connection<R1, W1>) -> Connection<R2, W2>
    where
        R2: ProtocolPacket + Debug,
        W2: ProtocolPacket + Debug,
    {
        Connection {
            stream: connection.stream,
            compression_threshold: connection.compression_threshold,
            enc_cipher: connection.enc_cipher,
            dec_cipher: connection.dec_cipher,
            _reading: PhantomData,
            _writing: PhantomData,
        }
    }
}
