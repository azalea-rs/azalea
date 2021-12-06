use crate::{friendly_byte_buf::FriendlyByteBuf, packets::Packet, ServerIpAddress};
use bytes::BytesMut;
use tokio::{
    io::{AsyncWriteExt, BufWriter},
    net::TcpStream,
};

pub enum PacketFlow {
    ClientToServer,
    ServerToClient,
}

pub struct Connection {
    pub flow: PacketFlow,
    pub stream: BufWriter<TcpStream>,
    /// The read buffer
    pub buffer: BytesMut,
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
            stream: BufWriter::new(stream),
            // 4mb read buffer
            buffer: BytesMut::with_capacity(4 * 1024 * 1024),
        })
    }

    /// Write a packet to the server
    pub async fn send_packet(&mut self, packet: &dyn Packet) {
        // packet structure:
        // length + id + data

        // Is this efficient? I have no idea, probably not.
        // getting rid of the FriendlyByteBuffer struct might help

        // write the packet id
        let mut id_and_data_buf = vec![packet.get_id()];

        // write the packet data
        let mut id_and_data_friendly_buf = FriendlyByteBuf::new(&mut id_and_data_buf);
        packet.write(&mut id_and_data_friendly_buf);

        // add the packet length to the beginning
        let mut complete_buf: Vec<u8> = Vec::new();
        let mut complete_friendly_buf = FriendlyByteBuf::new(&mut complete_buf);
        complete_friendly_buf.write_varint(id_and_data_buf.len() as u32);
        complete_buf.append(&mut id_and_data_buf);

        // finally, write and flush to the stream
        self.stream.write_all(&complete_buf).await.unwrap();
        self.stream.flush().await.unwrap();
    }
}
