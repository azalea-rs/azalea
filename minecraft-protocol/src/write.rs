use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{mc_buf::Writable, packets::ProtocolPacket};

pub async fn write_packet(packet: impl ProtocolPacket, stream: &mut TcpStream) {
    // TODO: implement compression

    // packet structure:
    // length (varint) + id (varint) + data

    // write the packet id
    let mut id_and_data_buf = vec![];
    id_and_data_buf.write_varint(packet.id() as i32);
    packet.write(&mut id_and_data_buf);

    // write the packet data

    // make a new buffer that has the length at the beginning
    // and id+data at the end
    let mut complete_buf: Vec<u8> = Vec::new();
    complete_buf.write_varint(id_and_data_buf.len() as i32);
    complete_buf.append(&mut id_and_data_buf);

    // finally, write and flush to the stream
    stream.write_all(&complete_buf).await.unwrap();
    stream.flush().await.unwrap();
}
