use crate::{connect::PacketFlow, mc_buf::Readable, packets::ProtocolPacket};
use async_compression::tokio::bufread::ZlibDecoder;
use tokio::{
    io::{AsyncReadExt, BufReader},
    net::TcpStream,
};

pub async fn read_packet<P: ProtocolPacket>(
    flow: &PacketFlow,
    stream: &mut TcpStream,
    compression_threshold: Option<u32>,
) -> Result<P, String> {
    // what this does:
    // 1. reads the first 5 bytes, probably only some of this will be used to get the packet length
    // 2. how much we should read = packet length - 5
    // 3. read the rest of the packet and add it to the cursor
    // 4. figure out what packet this is and parse it

    // the first thing minecraft sends us is the length as a varint, which can be up to 5 bytes long
    let mut buf = BufReader::with_capacity(4 * 1024 * 1024, stream);

    // Packet Length
    let packet_size = buf.read_varint().await?;

    // if there's no compression, we can just read the rest of the packet normally
    if compression_threshold.is_none() {
        // then, minecraft tells us the packet id as a varint
        let packet_id = buf.read_varint().await?;

        // if we recognize the packet id, parse it

        println!("reading uncompressed packet id: {}", packet_id);
        let packet = P::read(packet_id.try_into().unwrap(), flow, &mut buf).await?;

        return Ok(packet);
    }

    println!("compressed packet size: {}", packet_size);

    // there's compression
    // Data Length
    let data_size = buf.read_varint().await?;
    println!("data size: {}", data_size);

    // this packet has no compression
    if data_size == 0 {
        // Packet ID
        let packet_id = buf.read_varint().await?;
        println!(
            "reading compressed packet without compression packet id: {}",
            packet_id
        );
        let packet = P::read(packet_id.try_into().unwrap(), flow, &mut buf).await?;
        return Ok(packet);
    }

    // this packet has compression
    let packet_size_varint_size = buf.get_varint_size(packet_size);

    let mut compressed_data = vec![0; packet_size as usize - packet_size_varint_size as usize];
    buf.read_exact(compressed_data.as_mut_slice())
        .await
        .expect("Not enough compressed data");

    let mut z = ZlibDecoder::new(compressed_data.as_slice());

    // Packet ID
    let packet_id = z.read_varint().await.unwrap();
    println!("reading compressed packet id: {}", packet_id);

    if let Ok(packet) = P::read(packet_id as u32, flow, &mut z).await {
        Ok(packet)
    } else {
        // read the rest of the bytes
        let packet_id_varint_size = z.get_varint_size(packet_id);
        let mut buf = vec![0; packet_size as usize - packet_id_varint_size as usize];
        z.read_exact(buf.as_mut_slice()).await.unwrap();
        println!("{:?}", buf);

        Err(format!("Error on packet id: {}", packet_id))
    }
}
