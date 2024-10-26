//! Write packets to a stream.

use std::{fmt::Debug, io::Read};

use azalea_buf::McBufVarWritable;
use azalea_crypto::Aes128CfbEnc;
use flate2::{bufread::ZlibEncoder, Compression};
use thiserror::Error;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use tracing::trace;

use crate::{packets::ProtocolPacket, read::MAXIMUM_UNCOMPRESSED_LENGTH};

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

pub fn serialize_packet<P: ProtocolPacket + Debug>(
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

pub fn compression_encoder(
    data: &[u8],
    compression_threshold: u32,
) -> Result<Vec<u8>, PacketCompressError> {
    let n = data.len();
    // if it's less than the compression threshold, don't compress
    if n < compression_threshold as usize {
        let mut buf = Vec::new();
        0.var_write_into(&mut buf)?;
        std::io::Write::write_all(&mut buf, data)?;
        Ok(buf)
    } else {
        // otherwise, compress
        let mut deflater = ZlibEncoder::new(data, Compression::default());
        // write deflated data to buf
        let mut compressed_data = Vec::new();
        deflater.read_to_end(&mut compressed_data)?;

        // prepend the length
        let mut len_prepended_compressed_data = Vec::new();
        (data.len() as u32).var_write_into(&mut len_prepended_compressed_data)?;
        len_prepended_compressed_data.append(&mut compressed_data);

        Ok(len_prepended_compressed_data)
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
    trace!("Sending packet: {packet:?}");
    let raw_packet = serialize_packet(packet).unwrap();
    write_raw_packet(&raw_packet, stream, compression_threshold, cipher).await
}

pub async fn write_raw_packet<W>(
    raw_packet: &[u8],
    stream: &mut W,
    compression_threshold: Option<u32>,
    cipher: &mut Option<Aes128CfbEnc>,
) -> std::io::Result<()>
where
    W: AsyncWrite + Unpin + Send,
{
    trace!("Writing raw packet: {raw_packet:?}");
    let mut raw_packet = raw_packet.to_vec();
    if let Some(threshold) = compression_threshold {
        raw_packet = compression_encoder(&raw_packet, threshold).unwrap();
    }
    raw_packet = frame_prepender(raw_packet).unwrap();
    // if we were given a cipher, encrypt the packet
    if let Some(cipher) = cipher {
        azalea_crypto::encrypt_packet(cipher, &mut raw_packet);
    }
    stream.write_all(&raw_packet).await
}
