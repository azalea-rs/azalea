use crate::packets::ProtocolPacket;
use azalea_buf::BufReadError;
use azalea_buf::McBufVarReadable;
use azalea_crypto::Aes128CfbDec;
use bytes::BytesMut;
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
    BadLength { max: usize, size: usize },
    #[error("Connection reset by peer")]
    ConnectionReset,
    #[error("Connection closed")]
    ConnectionClosed,
}

/// Read a length, then read that amount of bytes from BytesMut. If there's not
/// enough data, return None
fn parse_frame(buffer: &mut BytesMut) -> Result<Vec<u8>, FrameSplitterError> {
    let buffer_copy: &mut &[u8] = &mut &buffer[..];
    // Packet Length
    let length = match u32::var_read_from(buffer_copy) {
        Ok(length) => length as usize,
        Err(err) => match err {
            BufReadError::Io(io_err) => return Err(FrameSplitterError::Io { source: io_err }),
            _ => return Err(err.into()),
        },
    };

    if length > buffer_copy.len() {
        return Err(FrameSplitterError::BadLength {
            max: buffer_copy.len(),
            size: length,
        });
    }

    // we read from the copy and we know it's legit, so we can take those bytes
    // from the real buffer now

    // the length of the varint that says the length of the whole packet
    let varint_length = buffer.len() - buffer_copy.len();
    let data = buffer_copy[..length].to_vec();
    let _ = buffer.split_to(length + varint_length);

    Ok(data)
}

async fn frame_splitter<R: ?Sized + Sized>(
    stream: &mut R,
    buffer: &mut BytesMut,
) -> Result<Vec<u8>, FrameSplitterError>
where
    R: AsyncRead + std::marker::Unpin + std::marker::Send,
{
    // https://tokio.rs/tokio/tutorial/framing
    loop {
        let read_frame = parse_frame(buffer);
        match read_frame {
            Ok(frame) => return Ok(frame),
            Err(err) => match err {
                FrameSplitterError::BadLength { .. } | FrameSplitterError::Io { .. } => {
                    // we probably just haven't read enough yet
                }
                _ => return Err(err),
            },
        }

        let read_buf: usize = AsyncReadExt::read_buf(stream, buffer).await?;
        if 0 == read_buf {
            // The remote closed the connection. For this to be
            // a clean shutdown, there should be no data in the
            // read buffer. If there is, this means that the
            // peer closed the socket while sending a frame.
            if buffer.is_empty() {
                return Err(FrameSplitterError::ConnectionClosed);
            } else {
                return Err(FrameSplitterError::ConnectionReset);
            }
        }
    }
}

fn packet_decoder<P: ProtocolPacket>(stream: &mut &[u8]) -> Result<P, ReadPacketError> {
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
    stream: &mut &[u8],
    compression_threshold: u32,
) -> Result<Vec<u8>, DecompressionError> {
    // Data Length
    let n = u32::var_read_from(stream)?;
    if n == 0 {
        // no data size, no compression
        let mut buf = vec![];
        std::io::Read::read_to_end(stream, &mut buf)?;
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
    buffer: &mut BytesMut,
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

    let mut buf = frame_splitter(&mut encrypted_stream, buffer).await?;

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
