use crate::packets::ProtocolPacket;
use azalea_buf::McBufVarReadable;
use azalea_buf::{read_varint_async, BufReadError};
use azalea_crypto::Aes128CfbDec;
use flate2::read::ZlibDecoder;
use log::{log_enabled, trace};
use std::{
    cell::Cell,
    io::Read,
    pin::Pin,
    task::{Context, Poll},
};
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncReadExt};

#[derive(Error, Debug)]
pub enum ReadPacketError {
    #[error("Error reading packet {packet_name} ({packet_id}): {source}")]
    Parse {
        packet_id: u32,
        packet_name: String,
        source: BufReadError,
    },
    #[error("Unknown packet id {id} in state {state_name}")]
    UnknownPacketId { state_name: String, id: u32 },
    #[error("Couldn't read packet id")]
    ReadPacketId { source: BufReadError },
    #[error("Couldn't decompress packet")]
    Decompress {
        #[from]
        source: DecompressionError,
    },
    #[error("Frame splitter error")]
    FrameSplitter {
        #[from]
        source: FrameSplitterError,
    },
    #[error("Leftover data after reading packet {packet_name}: {data:?}")]
    LeftoverData { data: Vec<u8>, packet_name: String },
}

#[derive(Error, Debug)]
pub enum FrameSplitterError {
    #[error("Couldn't read VarInt length for packet. The previous packet may have been corrupted")]
    LengthRead {
        #[from]
        source: BufReadError,
    },
    #[error("Io error")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("Packet is longer than {max} bytes (is {size})")]
    BadLength { max: u32, size: u32 },
}

async fn frame_splitter<R: ?Sized>(mut stream: &mut R) -> Result<Vec<u8>, FrameSplitterError>
where
    R: AsyncRead + std::marker::Unpin + std::marker::Send,
{
    // Packet Length
    let length = read_varint_async(&mut stream).await? as u32;

    // TODO: read individual tcp packets so we don't need this
    // https://github.com/tokio-rs/tokio/blob/master/examples/print_each_packet.rs
    let max_length: u32 = 2u32.pow(20u32); // 1mb, arbitrary
    if length > max_length {
        // minecraft *probably* won't send packets bigger than this
        return Err(FrameSplitterError::BadLength {
            max: max_length,
            size: length,
        });
    }

    let mut buf = vec![0; length as usize];
    stream.read_exact(&mut buf).await?;

    Ok(buf)
}

fn packet_decoder<P: ProtocolPacket>(stream: &mut impl Read) -> Result<P, ReadPacketError> {
    // Packet ID
    let packet_id =
        u32::var_read_from(stream).map_err(|e| ReadPacketError::ReadPacketId { source: e })?;
    P::read(packet_id.try_into().unwrap(), stream)
}

// this is always true in multiplayer, false in singleplayer
static VALIDATE_DECOMPRESSED: bool = true;

pub static MAXIMUM_UNCOMPRESSED_LENGTH: u32 = 2097152;

#[derive(Error, Debug)]
pub enum DecompressionError {
    #[error("Couldn't read VarInt length for data")]
    LengthReadError {
        #[from]
        source: BufReadError,
    },
    #[error("Io error")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("Badly compressed packet - size of {size} is below server threshold of {threshold}")]
    BelowCompressionThreshold { size: u32, threshold: u32 },
    #[error(
        "Badly compressed packet - size of {size} is larger than protocol maximum of {maximum}"
    )]
    AboveCompressionThreshold { size: u32, maximum: u32 },
}

fn compression_decoder(
    stream: &mut impl Read,
    compression_threshold: u32,
) -> Result<Vec<u8>, DecompressionError> {
    // Data Length
    let n = u32::var_read_from(stream)?;
    if n == 0 {
        // no data size, no compression
        let mut buf = vec![];
        stream.read_to_end(&mut buf)?;
        return Ok(buf);
    }

    if VALIDATE_DECOMPRESSED {
        if n < compression_threshold {
            return Err(DecompressionError::BelowCompressionThreshold {
                size: n,
                threshold: compression_threshold,
            });
        }
        if n > MAXIMUM_UNCOMPRESSED_LENGTH {
            return Err(DecompressionError::AboveCompressionThreshold {
                size: n,
                maximum: MAXIMUM_UNCOMPRESSED_LENGTH,
            });
        }
    }

    let mut decoded_buf = vec![];
    let mut decoder = ZlibDecoder::new(stream);
    decoder.read_to_end(&mut decoded_buf)?;

    Ok(decoded_buf)
}

struct EncryptedStream<'a, R>
where
    R: AsyncRead + std::marker::Unpin + std::marker::Send,
{
    cipher: Cell<&'a mut Option<Aes128CfbDec>>,
    stream: &'a mut Pin<&'a mut R>,
}

impl<R> AsyncRead for EncryptedStream<'_, R>
where
    R: AsyncRead + Unpin + Send,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        // i hate this
        let polled = self.as_mut().stream.as_mut().poll_read(cx, buf);
        match polled {
            Poll::Ready(r) => {
                // if we don't check for the remaining then we decrypt big packets incorrectly
                // (but only on linux and release mode for some reason LMAO)
                if buf.remaining() == 0 {
                    if let Some(cipher) = self.as_mut().cipher.get_mut() {
                        azalea_crypto::decrypt_packet(cipher, buf.filled_mut());
                    }
                }
                Poll::Ready(r)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pub async fn read_packet<'a, P: ProtocolPacket, R>(
    stream: &'a mut R,
    compression_threshold: Option<u32>,
    cipher: &mut Option<Aes128CfbDec>,
) -> Result<P, ReadPacketError>
where
    R: AsyncRead + std::marker::Unpin + std::marker::Send + std::marker::Sync,
{
    // if we were given a cipher, decrypt the packet
    let mut encrypted_stream = EncryptedStream {
        cipher: Cell::new(cipher),
        stream: &mut Pin::new(stream),
    };

    let mut buf = frame_splitter(&mut encrypted_stream).await?;

    if let Some(compression_threshold) = compression_threshold {
        buf = compression_decoder(&mut buf.as_slice(), compression_threshold)?;
    }

    if log_enabled!(log::Level::Trace) {
        let buf_string: String = {
            if buf.len() > 100 {
                let cut_off_buf = &buf[..100];
                format!("{cut_off_buf:?}...")
            } else {
                format!("{buf:?}")
            }
        };
        trace!("Reading packet with bytes: {buf_string}");
    }

    let packet = packet_decoder(&mut buf.as_slice())?;

    Ok(packet)
}
