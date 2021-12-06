use crate::{connection::Connection, resolver, ServerAddress};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

struct ServerStatus {}

async fn write_byte(buf: &mut Vec<u8>, n: u8) {
    buf.write_u8(n).await.unwrap();
    println!("write_byte: {}", n);
}

async fn write_bytes(buf: &mut Vec<u8>, bytes: &[u8]) {
    buf.write_all(bytes).await.unwrap();
    println!("write_bytes: {:?}", buf);
}

async fn write_varint(buf: &mut Vec<u8>, mut n: u32) {
    loop {
        if (n & 0xFFFFFF80) == 0 {
            write_byte(buf, n as u8).await;
            return ();
        }
        write_byte(buf, (n & 0x7F | 0x80) as u8).await;
        n >>= 7;
    }
}

async fn write_utf(buf: &mut Vec<u8>, string: &[u8], len: usize) {
    if string.len() > len {
        panic!(
            "String too big (was {} bytes encoded, max {})",
            string.len(),
            len
        );
    }
    write_varint(buf, string.len() as u32).await;
    write_bytes(buf, string).await;
}

async fn write_short(buf: &mut Vec<u8>, n: u16) {
    buf.write_u16(n).await.unwrap();
    println!("write_short: {}", n);
}

pub async fn ping_server(address: &ServerAddress) -> Result<(), String> {
    let resolved_address = resolver::resolve_address(&address).await?;

    let mut conn = Connection::new(&resolved_address).await?;

    // protocol version is 757

    // client intention packet
    // friendlyByteBuf.writeVarInt(this.protocolVersion);
    // friendlyByteBuf.writeUtf(this.hostName);
    // friendlyByteBuf.writeShort(this.port);
    // friendlyByteBuf.writeVarInt(this.intention.getId());

    println!("resolved_address {}", &resolved_address.ip);
    println!("writing intention packet {}", address.host);

    let mut buf: Vec<u8> = vec![0x00]; // 0 is the packet id for handshake
    write_varint(&mut buf, 757).await;
    write_utf(&mut buf, address.host.as_bytes(), 32767).await;
    write_short(&mut buf, address.port).await;
    write_varint(&mut buf, 1).await;

    let mut full_buffer = vec![];
    write_varint(&mut full_buffer, buf.len() as u32).await; // length of 1st packet id + data as VarInt
    full_buffer.append(&mut buf);
    full_buffer.extend_from_slice(&[
        1,    // length of 2nd packet id + data as VarInt
        0x00, // 2nd packet id: 0 for request as VarInt
    ]);

    conn.stream.write_all(&full_buffer).await.unwrap();
    conn.stream.flush().await.unwrap();

    // log what the server sends back
    loop {
        if 0 == conn.stream.read_buf(&mut conn.buffer).await.unwrap() {
            // The remote closed the connection. For this to be a clean
            // shutdown, there should be no data in the read buffer. If
            // there is, this means that the peer closed the socket while
            // sending a frame.

            // log conn.buffer
            println!("{:?}", conn.buffer);
            if conn.buffer.is_empty() {
                println!("buffer is empty ok");
                return Ok(());
            } else {
                return Err("connection reset by peer".into());
            }
        }
    }

    Ok(())
}
