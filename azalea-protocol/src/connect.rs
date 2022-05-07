//! parse sending and receiving packets with a server.

use crate::packets::game::GamePacket;
use crate::packets::handshake::HandshakePacket;
use crate::packets::login::LoginPacket;
use crate::packets::status::StatusPacket;
use crate::read::read_packet;
use crate::write::write_packet;
use crate::ServerIpAddress;
use azalea_crypto::{Aes128CfbDec, Aes128CfbEnc};
use tokio::net::TcpStream;

#[derive(Debug, Clone, Copy)]
pub enum PacketFlow {
    ClientToServer,
    ServerToClient,
}

pub struct HandshakeConnection {
    pub flow: PacketFlow,
    /// The buffered writer
    pub stream: TcpStream,
}

pub struct GameConnection {
    pub flow: PacketFlow,
    /// The buffered writer
    pub stream: TcpStream,
    pub compression_threshold: Option<u32>,
    pub enc_cipher: Option<Aes128CfbEnc>,
    pub dec_cipher: Option<Aes128CfbDec>,
}

pub struct StatusConnection {
    pub flow: PacketFlow,
    /// The buffered writer
    pub stream: TcpStream,
}

pub struct LoginConnection {
    pub flow: PacketFlow,
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

        Ok(HandshakeConnection {
            flow: PacketFlow::ServerToClient,
            stream,
        })
    }

    pub fn login(self) -> LoginConnection {
        LoginConnection {
            flow: self.flow,
            stream: self.stream,
            compression_threshold: None,
            enc_cipher: None,
            dec_cipher: None,
        }
    }

    pub fn status(self) -> StatusConnection {
        StatusConnection {
            flow: self.flow,
            stream: self.stream,
        }
    }

    pub async fn read(&mut self) -> Result<HandshakePacket, String> {
        read_packet::<HandshakePacket, _>(&self.flow, &mut self.stream, None, &mut None).await
    }

    /// Write a packet to the server
    pub async fn write(&mut self, packet: HandshakePacket) {
        write_packet(packet, &mut self.stream, None, &mut None).await;
    }
}

impl GameConnection {
    pub async fn read(&mut self) -> Result<GamePacket, String> {
        read_packet::<GamePacket, _>(
            &self.flow,
            &mut self.stream,
            self.compression_threshold,
            &mut self.dec_cipher,
        )
        .await
    }

    /// Write a packet to the server
    pub async fn write(&mut self, packet: GamePacket) {
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
    pub async fn read(&mut self) -> Result<StatusPacket, String> {
        read_packet::<StatusPacket, _>(&self.flow, &mut self.stream, None, &mut None).await
    }

    /// Write a packet to the server
    pub async fn write(&mut self, packet: StatusPacket) {
        write_packet(packet, &mut self.stream, None, &mut None).await;
    }
}

impl LoginConnection {
    pub async fn read(&mut self) -> Result<LoginPacket, String> {
        read_packet::<LoginPacket, _>(
            &self.flow,
            &mut self.stream,
            self.compression_threshold,
            &mut self.dec_cipher,
        )
        .await
    }

    /// Write a packet to the server
    pub async fn write(&mut self, packet: LoginPacket) {
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
            flow: self.flow,
            stream: self.stream,
            compression_threshold: self.compression_threshold,
            enc_cipher: self.enc_cipher,
            dec_cipher: self.dec_cipher,
        }
    }
}
