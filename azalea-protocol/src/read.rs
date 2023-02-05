//! Read packets from a stream.

use crate::packets::ProtocolPacket;
use azalea_buf::BufReadError;
use azalea_buf::McBufVarReadable;
use azalea_crypto::Aes128CfbDec;
use bytes::Buf;
use bytes::BytesMut;
use flate2::read::ZlibDecoder;
use futures::StreamExt;
use log::{log_enabled, trace};
use std::backtrace::Backtrace;
use std::{
    fmt::Debug,
    io::{Cursor, Read},
};
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Error, Debug)]
pub enum ReadPacketError {
    #[error("Error reading packet {packet_name} (id {packet_id}): {source}")]
    Parse {
        packet_id: u32,
        packet_name: String,
        backtrace: Box<Backtrace>,
        source: BufReadError,
    },
    #[error("Unknown packet id {id} in state {state_name}")]
    UnknownPacketId { state_name: String, id: u32 },
    #[error("Couldn't read packet id")]
    ReadPacketId { source: BufReadError },
    #[error(transparent)]
    Decompress {
        #[from]
        #[backtrace]
        source: DecompressionError,
    },
    #[error(transparent)]
    FrameSplitter {
        #[from]
        #[backtrace]
        source: FrameSplitterError,
    },
    #[error("Leftover data after reading packet {packet_name}: {data:?}")]
    LeftoverData { data: Vec<u8>, packet_name: String },
    #[error(transparent)]
    IoError {
        #[from]
        #[backtrace]
        source: std::io::Error,
    },
    #[error("Connection closed")]
    ConnectionClosed,
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
        #[backtrace]
        source: std::io::Error,
    },
    #[error("Packet is longer than {max} bytes (is {size})")]
    BadLength { max: usize, size: usize },
    #[error("Connection reset by peer")]
    ConnectionReset,
    #[error("Connection closed")]
    ConnectionClosed,
}

/// Read a length, then read that amount of bytes from `BytesMut`. If there's
/// not enough data, return None
fn parse_frame(buffer: &mut BytesMut) -> Result<BytesMut, FrameSplitterError> {
    // copy the buffer first and read from the copy, then once we make sure
    // the packet is all good we read it fully
    let mut buffer_copy = Cursor::new(&buffer[..]);
    // Packet Length
    let length = match u32::var_read_from(&mut buffer_copy) {
        Ok(length) => length as usize,
        Err(err) => match err {
            BufReadError::Io { source } => return Err(FrameSplitterError::Io { source }),
            _ => return Err(err.into()),
        },
    };

    if length > buffer_copy.remaining() {
        return Err(FrameSplitterError::BadLength {
            max: buffer_copy.remaining(),
            size: length,
        });
    }

    // we read from the copy and we know it's legit, so we can take those bytes
    // from the real buffer now

    // the length of the varint that says the length of the whole packet
    let varint_length = buffer.remaining() - buffer_copy.remaining();

    buffer.advance(varint_length);
    let data = buffer.split_to(length);

    Ok(data)
}

fn frame_splitter(buffer: &mut BytesMut) -> Result<Option<Vec<u8>>, FrameSplitterError> {
    // https://tokio.rs/tokio/tutorial/framing
    let read_frame = parse_frame(buffer);
    match read_frame {
        Ok(frame) => return Ok(Some(frame.to_vec())),
        Err(err) => match err {
            FrameSplitterError::BadLength { .. } | FrameSplitterError::Io { .. } => {
                // we probably just haven't read enough yet
            }
            _ => return Err(err),
        },
    }

    Ok(None)
}

fn packet_decoder<P: ProtocolPacket + Debug>(
    stream: &mut Cursor<&[u8]>,
) -> Result<P, Box<ReadPacketError>> {
    // Packet ID
    let packet_id =
        u32::var_read_from(stream).map_err(|e| ReadPacketError::ReadPacketId { source: e })?;
    P::read(packet_id, stream)
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
        #[backtrace]
        source: std::io::Error,
    },
    #[error("Badly compressed packet - size of {size} is below server threshold of {threshold}")]
    BelowCompressionThreshold { size: u32, threshold: u32 },
    #[error(
        "Badly compressed packet - size of {size} is larger than protocol maximum of {maximum}"
    )]
    AboveCompressionThreshold { size: u32, maximum: u32 },
}

/// Get the decompressed bytes from a packet. It must have been decrypted
/// first.
fn compression_decoder(
    stream: &mut Cursor<&[u8]>,
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

/// Read a single packet from a stream.
///
/// The buffer is required because servers may send multiple packets in the
/// same frame, so we need to store the packet data that's left to read.
///
/// The current protocol state must be passed as a generic.
pub async fn read_packet<'a, P: ProtocolPacket + Debug, R>(
    stream: &'a mut R,
    buffer: &mut BytesMut,
    compression_threshold: Option<u32>,
    cipher: &mut Option<Aes128CfbDec>,
) -> Result<P, Box<ReadPacketError>>
where
    R: AsyncRead + std::marker::Unpin + std::marker::Send + std::marker::Sync,
{
    let mut framed = FramedRead::new(stream, BytesCodec::new());
    let mut buf = loop {
        if let Some(buf) = frame_splitter(buffer).map_err(ReadPacketError::from)? {
            // we got a full packet!!
            break buf;
        } else {
            // no full packet yet :( keep reading
        };

        // if we were given a cipher, decrypt the packet
        if let Some(message) = framed.next().await {
            let mut bytes = message.map_err(ReadPacketError::from)?;

            if let Some(cipher) = cipher {
                azalea_crypto::decrypt_packet(cipher, &mut bytes);
            }

            buffer.extend_from_slice(&bytes);
        } else {
            return Err(Box::new(ReadPacketError::ConnectionClosed));
        };
    };

    if let Some(compression_threshold) = compression_threshold {
        buf = compression_decoder(&mut Cursor::new(&buf[..]), compression_threshold)
            .map_err(ReadPacketError::from)?;
    }

    if log_enabled!(log::Level::Trace) {
        let buf_string: String = {
            if buf.len() > 500 {
                let cut_off_buf = &buf[..500];
                format!("{cut_off_buf:?}...")
            } else {
                format!("{buf:?}")
            }
        };
        trace!("Reading packet with bytes: {buf_string}");
    }

    let packet = packet_decoder(&mut Cursor::new(&buf[..]))?;

    Ok(packet)
}
