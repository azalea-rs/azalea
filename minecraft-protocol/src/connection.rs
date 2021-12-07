use crate::{mc_buf, packets::Packet, ServerIpAddress};
use bytes::BytesMut;
use std::io::{Cursor, Read, Write};
use tokio::io::AsyncWriteExt;
use tokio::{
    io::{AsyncReadExt, BufReader, BufWriter, SeekFrom, AsyncSeek, AsyncSeekExt},
    net::TcpStream,
};

pub enum PacketFlow {
    ClientToServer,
    ServerToClient,
}

pub struct Connection {
    pub flow: PacketFlow,
    /// The buffered writer
    pub stream: TcpStream,
}

impl Connection {
    pub async fn new(address: &ServerIpAddress) -> Result<Connection, String> {
        let ip = address.ip;
        let port = address.port;

        let stream = TcpStream::connect(format!("{}:{}", ip, port))
            .await
            .map_err(|_| "Failed to connect to server")?;

        // enable tcp_nodelay
        stream
            .set_nodelay(true)
            .expect("Error enabling tcp_nodelay");

        Ok(Connection {
            flow: PacketFlow::ClientToServer,
            stream,
        })
    }

    pub async fn read_packet(&mut self) -> Result<(), String> {
        // what this does:
        // 1. reads the first 5 bytes, probably only some of this will be used to get the packet length
        // 2. how much we should read = packet length - 5
        // 3. read the rest of the packet and add it to the cursor

        // the first thing minecraft sends us is the length as a varint, which can be up to 5 bytes long
        let mut buf = BufReader::with_capacity(5 * 1024, &mut self.stream);
        let (packet_size, packet_size_varint_size) = mc_buf::read_varint(&mut buf).await?;
        // then, minecraft tells us the packet id as a single byte
        let packet_id = mc_buf::read_byte(&mut buf).await?;

        // read the rest of the packet
        let mut packet_data = Vec::with_capacity(
            (
                packet_size // the total size of the packet
                - 1 // we just read the packet id, so we don't read that byte again
            ) as usize);
        buf.read_buf(&mut packet_data).await.unwrap();
        println!("packet {}", packet_id);
        // println!(
        //     "packet id {}: {}",
        //     packet_id,
        //     String::from_utf8(packet_data.clone()).unwrap()
        // );

        Ok(())
    }

    /// Write a packet to the server
    pub async fn send_packet(&mut self, packet: &dyn Packet) {
        // packet structure:
        // length + id + data

        // write the packet id
        let mut id_and_data_buf = vec![packet.get_id()];

        // write the packet data
        packet.write(&mut id_and_data_buf);

        // make a new buffer that has the length at the beginning
        // and id+data at the end
        let mut complete_buf: Vec<u8> = Vec::new();
        mc_buf::write_varint(&mut complete_buf, id_and_data_buf.len() as u32);
        complete_buf.append(&mut id_and_data_buf);

        // finally, write and flush to the stream
        self.stream.write_all(&complete_buf).await.unwrap();
        self.stream.flush().await.unwrap();
    }
}
