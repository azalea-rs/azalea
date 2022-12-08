use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundLoginPacket;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, McBuf, ServerboundLoginPacket)]
pub struct ServerboundHelloPacket {
    pub name: String,
    pub profile_id: Option<Uuid>,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
    use azalea_buf::{McBufReadable, McBufWritable};

    #[test]
    fn test_read_write() {
        let packet = ServerboundHelloPacket {
            name: "test".to_string(),
            profile_id: Some(Uuid::nil()),
        };
        let mut buf: Vec<u8> = Vec::new();
        packet.write_into(&mut buf).unwrap();
        let packet2 = ServerboundHelloPacket::read_from(&mut Cursor::new(&buf)).unwrap();
        assert_eq!(packet, packet2);
    }
}
