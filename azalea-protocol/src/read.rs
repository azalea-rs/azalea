use tokio::{io::BufReader, net::TcpStream};

use crate::{connect::PacketFlow, mc_buf::Readable, packets::ProtocolPacket};

pub async fn read_packet<P: ProtocolPacket>(
    flow: &PacketFlow,
    stream: &mut TcpStream,
) -> Result<P, String> {
    // what this does:
    // 1. reads the first 5 bytes, probably only some of this will be used to get the packet length
    // 2. how much we should read = packet length - 5
    // 3. read the rest of the packet and add it to the cursor
    // 4. figure out what packet this is and parse it

    // the first thing minecraft sends us is the length as a varint, which can be up to 5 bytes long
    let mut buf = BufReader::with_capacity(4 * 1024 * 1024, stream);

    let _packet_size = buf.read_varint().await?;

    // then, minecraft tells us the packet id as a varint
    let packet_id = buf.read_varint().await?;

    // if we recognize the packet id, parse it

    let packet = P::read(packet_id.try_into().unwrap(), flow, &mut buf).await?;

    Ok(packet)
}
