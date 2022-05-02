use crate::{
    connect::PacketFlow,
    mc_buf::{read_varint_async, Readable},
    packets::ProtocolPacket,
};
use azalea_crypto::Aes128CfbDec;
use flate2::read::ZlibDecoder;
use std::{
    cell::Cell,
    io::Read,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncReadExt};

async fn frame_splitter<R: ?Sized>(mut stream: &mut R) -> Result<Vec<u8>, String>
where
    R: AsyncRead + std::marker::Unpin + std::marker::Send,
{
    // Packet Length
    let length_result = read_varint_async(&mut stream).await;
    match length_result {
        Ok(length) => {
            let mut buf = vec![0; length as usize];

            stream
                .read_exact(&mut buf)
                .await
                .map_err(|e| e.to_string())?;

            Ok(buf)
        }
        Err(_) => Err("length wider than 21-bit".to_string()),
    }
}

fn packet_decoder<P: ProtocolPacket>(
    stream: &mut impl Read,
    flow: &PacketFlow,
) -> Result<P, String> {
    // Packet ID
    let packet_id = stream.read_varint()?;
    P::read(packet_id.try_into().unwrap(), flow, stream)
}

// this is always true in multiplayer, false in singleplayer
static VALIDATE_DECOMPRESSED: bool = true;

pub static MAXIMUM_UNCOMPRESSED_LENGTH: u32 = 2097152;

fn compression_decoder(
    stream: &mut impl Read,
    compression_threshold: u32,
) -> Result<Vec<u8>, String> {
    // Data Length
    let n: u32 = stream.read_varint()?.try_into().unwrap();
    if n == 0 {
        // no data size, no compression
        let mut buf = vec![];
        stream.read_to_end(&mut buf).map_err(|e| e.to_string())?;
        return Ok(buf);
    }

    if VALIDATE_DECOMPRESSED {
        if n < compression_threshold {
            return Err(format!(
                "Badly compressed packet - size of {} is below server threshold of {}",
                n, compression_threshold
            ));
        }
        if n > MAXIMUM_UNCOMPRESSED_LENGTH {
            return Err(format!(
                "Badly compressed packet - size of {} is larger than protocol maximum of {}",
                n, MAXIMUM_UNCOMPRESSED_LENGTH
            ));
        }
    }

    let mut decoded_buf = vec![];
    let mut decoder = ZlibDecoder::new(stream);
    decoder
        .read_to_end(&mut decoded_buf)
        .map_err(|e| e.to_string())?;

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
                match r {
                    Ok(()) => Poll::Ready(Ok(())),
                    Err(e) => panic!("{:?}", e),
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pub async fn read_packet<'a, P: ProtocolPacket, R>(
    flow: &PacketFlow,
    stream: &'a mut R,
    compression_threshold: Option<u32>,
    cipher: &mut Option<Aes128CfbDec>,
) -> Result<P, String>
where
    R: AsyncRead + std::marker::Unpin + std::marker::Send + std::marker::Sync,
{
    // let start_time = std::time::Instant::now();

    // println!("decrypting packet ({}ms)", start_time.elapsed().as_millis());
    // if we were given a cipher, decrypt the packet
    let mut encrypted_stream = EncryptedStream {
        cipher: Cell::new(cipher),
        stream: &mut Pin::new(stream),
    };

    // println!("splitting packet ({}ms)", start_time.elapsed().as_millis());
    let mut buf = frame_splitter(&mut encrypted_stream).await?;

    if let Some(compression_threshold) = compression_threshold {
        // println!(
        //     "decompressing packet ({}ms)",
        //     start_time.elapsed().as_millis()
        // );
        buf = compression_decoder(&mut &buf[..], compression_threshold)?;
    }

    // println!("decoding packet ({}ms)", start_time.elapsed().as_millis());
    let packet = packet_decoder(&mut buf.as_slice(), flow)?;
    // println!("decoded packet ({}ms)", start_time.elapsed().as_millis());

    Ok(packet)
}
