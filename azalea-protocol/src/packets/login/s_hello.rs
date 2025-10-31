use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundLoginPacket;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, AzBuf, PartialEq, ServerboundLoginPacket)]
pub struct ServerboundHello {
    #[limit(16)]
    pub name: String,
    pub profile_id: Uuid,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::{AzaleaRead as _, AzaleaWrite as _};

    use super::*;

    #[test]
    fn test_read_write() {
        let packet = ServerboundHello {
            name: "test".to_owned(),
            profile_id: Uuid::nil(),
        };
        let mut buf: Vec<u8> = Vec::new();
        packet.azalea_write(&mut buf).unwrap();
        let packet2 = ServerboundHello::azalea_read(&mut Cursor::new(&buf)).unwrap();
        assert_eq!(packet, packet2);
    }
}
