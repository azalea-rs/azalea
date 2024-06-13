use azalea_buf::McBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundTabListPacket {
    pub header: FormattedText,
    pub footer: FormattedText,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::McBufReadable;

    use super::*;

    #[test]
    fn test_packet_from_viaversion() {
        #[rustfmt::skip]
        let mut bytes = Cursor::new(&[
            10, 9, 0, 5, 101, 120, 116, 114, 97, 10, 0, 0, 0, 3, 1, 0, 4, 98, 111, 108, 100, 1, 8, 0, 4, 116, 101, 120, 116, 0, 16, 50, 66, 85, 73, 76, 68, 69, 82, 83, 50, 84, 79, 79, 76, 83, 10, 8, 0, 5, 99, 111, 108, 111, 114, 0, 4, 103, 114, 97, 121, 0, 8, 0, 0, 0, 1, 10, 0, 8, 0, 5, 99, 111, 108, 111, 114, 0, 4, 103, 111, 108, 100, 8, 0, 4, 116, 101, 120, 116, 0, 27, 80, 101, 110, 100, 105, 110, 103, 32, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 32, 116, 111, 32, 50, 98, 50, 116, 10, 0, 8, 0, 4, 116, 101, 120, 116, 0, 1, 10, 0, 10, 9, 0, 5, 101, 120, 116, 114, 97, 10, 0, 0, 0, 1, 8, 0, 5, 99, 111, 108, 111, 114, 0, 4, 103, 111, 108, 100, 8, 0, 4, 116, 101, 120, 116, 0, 72, 84, 104, 105, 115, 32, 97, 99, 99, 111, 117, 110, 116, 32, 104, 97, 115, 32, 112, 114, 105, 111, 114, 105, 116, 121, 32, 115, 116, 97, 116, 117, 115, 32, 97, 110, 100, 32, 119, 105, 108, 108, 32, 98, 101, 32, 112, 108, 97, 99, 101, 100, 32, 105, 110, 32, 97, 32, 115, 104, 111, 114, 116, 101, 114, 32, 113, 117, 101, 117, 101, 46, 10, 0, 8, 0, 4, 116, 101, 120, 116, 0, 1, 10, 0
        ][..]);
        let _packet = ClientboundTabListPacket::read_from(&mut bytes).unwrap();
        assert!(bytes.get_ref()[bytes.position() as usize..].is_empty());
    }
}
