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
use thiserror::Error;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

pub struct ReadConnection<R: ProtocolPacket> {
    pub read_stream: OwnedReadHalf,
    pub compression_threshold: Option<u32>,
    pub dec_cipher: Option<Aes128CfbDec>,
    _reading: PhantomData<R>,
}

pub struct WriteConnection<W: ProtocolPacket> {
    pub write_stream: OwnedWriteHalf,
    pub compression_threshold: Option<u32>,
    pub enc_cipher: Option<Aes128CfbEnc>,
    _writing: PhantomData<W>,
}

pub struct Connection<R: ProtocolPacket, W: ProtocolPacket> {
    pub reader: ReadConnection<R>,
    pub writer: WriteConnection<W>,
}

impl<R> ReadConnection<R>
where
    R: ProtocolPacket + Debug,
{
    pub async fn read(&mut self) -> Result<R, ReadPacketError> {
        read_packet::<R, _>(
            &mut self.read_stream,
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
    /// Write a packet to the server
    pub async fn write(&mut self, packet: W) -> std::io::Result<()> {
        write_packet(
            packet,
            &mut self.write_stream,
            self.compression_threshold,
            &mut self.enc_cipher,
        )
        .await
    }
}

impl<R, W> Connection<R, W>
where
    R: ProtocolPacket + Debug,
    W: ProtocolPacket + Debug,
{
    pub async fn read(&mut self) -> Result<R, ReadPacketError> {
        self.reader.read().await
    }

    /// Write a packet to the server
    pub async fn write(&mut self, packet: W) -> std::io::Result<()> {
        self.writer.write(packet).await
    }
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

impl Connection<ClientboundHandshakePacket, ServerboundHandshakePacket> {
    pub async fn new(address: &ServerIpAddress) -> Result<Self, ConnectionError> {
        let ip = address.ip;
        let port = address.port;

        let stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;

        // enable tcp_nodelay
        stream.set_nodelay(true)?;

        let (read_stream, write_stream) = stream.into_split();

        Ok(Connection {
            reader: ReadConnection {
                read_stream,
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

    pub fn login(self) -> Connection<ClientboundLoginPacket, ServerboundLoginPacket> {
        Connection::from(self)
    }

    pub fn status(self) -> Connection<ClientboundStatusPacket, ServerboundStatusPacket> {
        Connection::from(self)
    }
}

impl Connection<ClientboundLoginPacket, ServerboundLoginPacket> {
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

    pub fn set_encryption_key(&mut self, key: [u8; 16]) {
        // minecraft has a cipher decoder and encoder, i don't think it matters though?
        let (enc_cipher, dec_cipher) = azalea_crypto::create_cipher(&key);
        self.writer.enc_cipher = Some(enc_cipher);
        self.reader.dec_cipher = Some(dec_cipher);
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
            reader: ReadConnection {
                read_stream: connection.reader.read_stream,
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
}
