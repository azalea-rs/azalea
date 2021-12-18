use std::io::Cursor;

use crate::{connect::PacketFlow, mc_buf::Readable, packets::ProtocolPacket};
use async_compression::tokio::bufread::ZlibDecoder;
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

async fn frame_splitter<R>(stream: &mut R) -> Result<Vec<u8>, String>
where
    R: AsyncRead + std::marker::Unpin + std::marker::Send,
{
    // Packet Length
    let length_result = stream.read_varint().await;
    match length_result {
        Ok(length) => {
            let mut buf = vec![0; length as usize];

            stream
                .read_exact(&mut buf)
                .await
                .map_err(|e| e.to_string())?;

            Ok(buf)
        }
        Err(e) => Err("length wider than 21-bit".to_string()),
    }
}

async fn packet_decoder<P: ProtocolPacket, R>(
    stream: &mut R,
    flow: &PacketFlow,
) -> Result<P, String>
where
    R: AsyncRead + std::marker::Unpin + std::marker::Send,
{
    // Packet ID
    let packet_id = stream.read_varint().await?;
    Ok(P::read(packet_id.try_into().unwrap(), flow, stream).await?)
}

// this is always true in multiplayer, false in singleplayer
static VALIDATE_DECOMPRESSED: bool = true;

pub static MAXIMUM_UNCOMPRESSED_LENGTH: u32 = 8388608;

async fn compression_decoder<R>(
    stream: &mut R,
    compression_threshold: u32,
) -> Result<Vec<u8>, String>
where
    R: AsyncRead + std::marker::Unpin + std::marker::Send,
{
    // Data Length
    let n: u32 = stream.read_varint().await?.try_into().unwrap();
    if n == 0 {
        // no data size, no compression
        let mut buf = vec![];
        stream
            .read_to_end(&mut buf)
            .await
            .map_err(|e| e.to_string())?;
        return Ok(buf);
    }

    if VALIDATE_DECOMPRESSED {
        if n < compression_threshold {
            return Err(format!(
                "Badly compressed packet - size of {} is below server threshold of {}",
                n, compression_threshold
            ));
        }
        if n > MAXIMUM_UNCOMPRESSED_LENGTH.into() {
            return Err(format!(
                "Badly compressed packet - size of {} is larger than protocol maximum of {}",
                n, MAXIMUM_UNCOMPRESSED_LENGTH
            ));
        }
    }

    let mut buf = vec![];
    stream
        .read_to_end(&mut buf)
        .await
        .map_err(|e| e.to_string())?;

    let mut decoded_buf = vec![];
    let mut decoder = ZlibDecoder::new(buf.as_slice());
    decoder
        .read_to_end(&mut decoded_buf)
        .await
        .map_err(|e| e.to_string())?;

    Ok(decoded_buf)
}

pub async fn read_packet<P: ProtocolPacket, R>(
    flow: &PacketFlow,
    stream: &mut R,
    compression_threshold: Option<u32>,
) -> Result<P, String>
where
    R: AsyncRead + std::marker::Unpin + std::marker::Send,
{
    let mut buf = frame_splitter(stream).await?;
    if let Some(compression_threshold) = compression_threshold {
        buf = compression_decoder(&mut buf.as_slice(), compression_threshold).await?;
    }
    let packet = packet_decoder(&mut buf.as_slice(), flow).await?;

    return Ok(packet);
}
