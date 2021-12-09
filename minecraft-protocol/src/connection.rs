//! parse sending and receiving packets with a server.

use crate::packets::ConnectionProtocol;
use crate::{mc_buf, packets::Packet, ServerIpAddress};
use tokio::io::AsyncWriteExt;
use tokio::{
    io::{AsyncReadExt, BufReader},
    net::TcpStream,
};

pub enum PacketFlow {
    ClientToServer,
    ServerToClient,
}

pub struct Connection {
    pub state: ConnectionProtocol,
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
            state: ConnectionProtocol::Handshaking,
            flow: PacketFlow::ClientToServer,
            stream,
        })
    }

    pub fn switch_state(&mut self, state: ConnectionProtocol) {
        self.state = state;
    }

    pub async fn read_packet(&mut self) -> Result<(), String> {
        // what this does:
        // 1. reads the first 5 bytes, probably only some of this will be used to get the packet length
        // 2. how much we should read = packet length - 5
        // 3. read the rest of the packet and add it to the cursor

        // the first thing minecraft sends us is the length as a varint, which can be up to 5 bytes long
        let mut buf = BufReader::with_capacity(4 * 1024 * 1024, &mut self.stream);
        println!("reading length varint");
        let (packet_size, packet_size_varint_size) = mc_buf::read_varint(&mut buf).await?;
        // then, minecraft tells us the packet id as a varint
        println!("reading id varint");
        let (packet_id, packet_id_size) = mc_buf::read_varint(&mut buf).await?;

        // if we recognize the packet id, parse it

        // TODO

        // otherwise, read the rest of the packet and throw it away
        let mut packet_data = Vec::with_capacity((packet_size - packet_id_size as i32) as usize);
        buf.read_buf(&mut packet_data).await.unwrap();
        println!("packet {:?}", packet_data);

        Ok(())
    }

    /// Write a packet to the server
    pub async fn send_packet(&mut self, packet: Packet<'_>) {
        // TODO: implement compression

        // packet structure:
        // length (varint) + id (varint) + data

        // write the packet id
        let mut id_and_data_buf = vec![];
        mc_buf::write_varint(&mut id_and_data_buf, packet.get_id() as i32);
        packet.write(&mut id_and_data_buf);

        // write the packet data

        // make a new buffer that has the length at the beginning
        // and id+data at the end
        let mut complete_buf: Vec<u8> = Vec::new();
        mc_buf::write_varint(&mut complete_buf, id_and_data_buf.len() as i32);
        complete_buf.append(&mut id_and_data_buf);

        // finally, write and flush to the stream
        self.stream.write_all(&complete_buf).await.unwrap();
        self.stream.flush().await.unwrap();
    }
}
