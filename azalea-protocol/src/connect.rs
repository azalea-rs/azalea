//! parse sending and receiving packets with a server.

use crate::packets::game::{ClientboundGamePacket, ServerboundGamePacket};
use crate::packets::handshake::{ClientboundHandshakePacket, ServerboundHandshakePacket};
use crate::packets::login::{ClientboundLoginPacket, ServerboundLoginPacket};
use crate::packets::status::{ClientboundStatusPacket, ServerboundStatusPacket};
use crate::read::read_packet;
use crate::write::write_packet;
use crate::ServerIpAddress;
use azalea_crypto::{Aes128CfbDec, Aes128CfbEnc};
use tokio::net::TcpStream;

pub struct HandshakeConnection {
    /// The buffered writer
    pub stream: TcpStream,
}

pub struct GameConnection {
    /// The buffered writer
    pub stream: TcpStream,
    pub compression_threshold: Option<u32>,
    pub enc_cipher: Option<Aes128CfbEnc>,
    pub dec_cipher: Option<Aes128CfbDec>,
}

pub struct StatusConnection {
    /// The buffered writer
    pub stream: TcpStream,
}

pub struct LoginConnection {
    /// The buffered writer
    pub stream: TcpStream,
    pub compression_threshold: Option<u32>,
    pub enc_cipher: Option<Aes128CfbEnc>,
    pub dec_cipher: Option<Aes128CfbDec>,
}

impl HandshakeConnection {
    pub async fn new(address: &ServerIpAddress) -> Result<HandshakeConnection, String> {
        let ip = address.ip;
        let port = address.port;

        let stream = TcpStream::connect(format!("{}:{}", ip, port))
            .await
            .map_err(|_| "Failed to connect to server")?;

        // enable tcp_nodelay
        stream
            .set_nodelay(true)
            .expect("Error enabling tcp_nodelay");

        Ok(HandshakeConnection { stream })
    }

    pub fn login(self) -> LoginConnection {
        LoginConnection {
            stream: self.stream,
            compression_threshold: None,
            enc_cipher: None,
            dec_cipher: None,
        }
    }

    pub fn status(self) -> StatusConnection {
        StatusConnection {
            stream: self.stream,
        }
    }

    pub async fn read(&mut self) -> Result<ClientboundHandshakePacket, String> {
        read_packet::<ClientboundHandshakePacket, _>(&mut self.stream, None, &mut None).await
    }

    /// Write a packet to the server
    pub async fn write(&mut self, packet: ServerboundHandshakePacket) {
        write_packet(packet, &mut self.stream, None, &mut None).await;
    }
}

impl GameConnection {
    pub async fn read(&mut self) -> Result<ClientboundGamePacket, String> {
        read_packet::<ClientboundGamePacket, _>(
            &mut self.stream,
            self.compression_threshold,
            &mut self.dec_cipher,
        )
        .await
    }

    /// Write a packet to the server
    pub async fn write(&mut self, packet: ServerboundGamePacket) {
        write_packet(
            packet,
            &mut self.stream,
            self.compression_threshold,
            &mut self.enc_cipher,
        )
        .await;
    }
}

impl StatusConnection {
    pub async fn read(&mut self) -> Result<ClientboundStatusPacket, String> {
        read_packet::<ClientboundStatusPacket, _>(&mut self.stream, None, &mut None).await
    }

    /// Write a packet to the server
    pub async fn write(&mut self, packet: ServerboundStatusPacket) {
        write_packet(packet, &mut self.stream, None, &mut None).await;
    }
}

impl LoginConnection {
    pub async fn read(&mut self) -> Result<ClientboundLoginPacket, String> {
        read_packet::<ClientboundLoginPacket, _>(
            &mut self.stream,
            self.compression_threshold,
            &mut self.dec_cipher,
        )
        .await
    }

    /// Write a packet to the server
    pub async fn write(&mut self, packet: ServerboundLoginPacket) {
        write_packet(
            packet,
            &mut self.stream,
            self.compression_threshold,
            &mut self.enc_cipher,
        )
        .await;
    }

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

    pub fn game(self) -> GameConnection {
        GameConnection {
            stream: self.stream,
            compression_threshold: self.compression_threshold,
            enc_cipher: self.enc_cipher,
            dec_cipher: self.dec_cipher,
        }
    }
}
