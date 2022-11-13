//! Write packets to a stream.

use crate::{packets::ProtocolPacket, read::MAXIMUM_UNCOMPRESSED_LENGTH};
use async_compression::tokio::bufread::ZlibEncoder;
use azalea_buf::McBufVarWritable;
use azalea_crypto::Aes128CfbEnc;
use std::fmt::Debug;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Prepend the length of the packet to it.
fn frame_prepender(mut data: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
    let mut buf = Vec::new();
    (data.len() as u32).var_write_into(&mut buf)?;
    buf.append(&mut data);
    Ok(buf)
}

#[derive(Error, Debug)]
pub enum PacketEncodeError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("Packet too big (is {actual} bytes, should be less than {maximum}): {packet_string}")]
    TooBig {
        actual: usize,
        maximum: usize,
        packet_string: String,
    },
}

fn packet_encoder<P: ProtocolPacket + std::fmt::Debug>(
    packet: &P,
) -> Result<Vec<u8>, PacketEncodeError> {
    let mut buf = Vec::new();
    packet.id().var_write_into(&mut buf)?;
    packet.write(&mut buf)?;
    if buf.len() > MAXIMUM_UNCOMPRESSED_LENGTH as usize {
        return Err(PacketEncodeError::TooBig {
            actual: buf.len(),
            maximum: MAXIMUM_UNCOMPRESSED_LENGTH as usize,
            packet_string: format!("{packet:?}"),
        });
    }
    Ok(buf)
}

#[derive(Error, Debug)]
pub enum PacketCompressError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

async fn compression_encoder(
    data: &[u8],
    compression_threshold: u32,
) -> Result<Vec<u8>, PacketCompressError> {
    let n = data.len();
    // if it's less than the compression threshold, don't compress
    if n < compression_threshold as usize {
        let mut buf = Vec::new();
        0.var_write_into(&mut buf)?;
        buf.write_all(data).await?;
        Ok(buf)
    } else {
        // otherwise, compress
        let mut deflater = ZlibEncoder::new(data);
        // write deflated data to buf
        let mut buf = Vec::new();
        deflater.read_to_end(&mut buf).await?;
        Ok(buf)
    }
}

pub async fn write_packet<P, W>(
    packet: &P,
    stream: &mut W,
    compression_threshold: Option<u32>,
    cipher: &mut Option<Aes128CfbEnc>,
) -> std::io::Result<()>
where
    P: ProtocolPacket + Debug,
    W: AsyncWrite + Unpin + Send,
{
    let mut buf = packet_encoder(packet).unwrap();
    if let Some(threshold) = compression_threshold {
        buf = compression_encoder(&buf, threshold).await.unwrap();
    }
    buf = frame_prepender(buf).unwrap();
    // if we were given a cipher, encrypt the packet
    if let Some(cipher) = cipher {
        azalea_crypto::encrypt_packet(cipher, &mut buf);
    }
    stream.write_all(&buf).await
}
