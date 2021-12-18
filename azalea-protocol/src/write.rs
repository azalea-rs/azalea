use std::io::Read;

use crate::{mc_buf::Writable, packets::ProtocolPacket, read::MAXIMUM_UNCOMPRESSED_LENGTH};
use async_compression::tokio::bufread::ZlibEncoder;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpStream,
};

fn frame_prepender(data: &mut Vec<u8>) -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    buf.write_varint(data.len() as i32)
        .map_err(|e| e.to_string())?;
    buf.append(data);
    Ok(buf)
}

fn packet_encoder<P: ProtocolPacket + std::fmt::Debug>(packet: &P) -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    buf.write_varint(packet.id() as i32)
        .map_err(|e| e.to_string())?;
    packet.write(&mut buf);
    if buf.len() > MAXIMUM_UNCOMPRESSED_LENGTH as usize {
        return Err(format!(
            "Packet too big (is {} bytes, should be less than {}): {:?}",
            buf.len(),
            MAXIMUM_UNCOMPRESSED_LENGTH,
            packet
        ));
    }
    Ok(buf)
}

async fn compression_encoder(data: &[u8], compression_threshold: u32) -> Result<Vec<u8>, String> {
    let n = data.len();
    // if it's less than the compression threshold, don't compress
    if n < compression_threshold as usize {
        let mut buf = Vec::new();
        buf.write_varint(0).map_err(|e| e.to_string())?;
        buf.write_all(data).await.map_err(|e| e.to_string())?;
        Ok(buf)
    } else {
        // otherwise, compress
        let mut deflater = ZlibEncoder::new(data);
        // write deflated data to buf
        let mut buf = Vec::new();
        deflater
            .read_to_end(&mut buf)
            .await
            .map_err(|e| e.to_string())?;
        Ok(buf)
    }
}

pub async fn write_packet<P>(packet: P, stream: &mut TcpStream, compression_threshold: Option<u32>)
where
    P: ProtocolPacket + std::fmt::Debug,
{
    let mut buf = packet_encoder(&packet).unwrap();
    if let Some(threshold) = compression_threshold {
        buf = compression_encoder(&buf, threshold).await.unwrap();
    }
    buf = frame_prepender(&mut buf).unwrap();
    stream.write_all(&buf).await.unwrap();
}
