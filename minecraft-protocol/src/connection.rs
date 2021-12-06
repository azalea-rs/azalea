use crate::ServerIpAddress;
use bytes::BytesMut;
use tokio::{io::BufWriter, net::TcpStream};

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
}
