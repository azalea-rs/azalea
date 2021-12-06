use crate::{mc_buf, packets::Packet, ServerIpAddress};
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
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

    pub async fn read_packet(&mut self) {
        // the first thing minecraft sends us is the length as a varint, which can be up to 5 bytes
        let mut buf = Vec::new();
        self.stream.read_buf(&mut buf).await;
        mc_buf::read_varint(buf)
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
