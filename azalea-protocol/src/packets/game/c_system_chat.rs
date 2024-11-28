use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket, PartialEq)]
pub struct ClientboundSystemChat {
    pub content: FormattedText,
    pub overlay: bool,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::AzaleaRead;

    use super::*;

    #[test]
    fn test_c_system_chat_packet() {
        #[rustfmt::skip]
        let bytes = [
            10, 9, 0, 4, 119, 105, 116, 104, 10, 0, 0, 0, 2, 10, 0, 10, 104, 111, 118, 101, 114, 69, 118, 101, 110, 116, 10, 0, 8, 99, 111, 110, 116, 101, 110, 116, 115, 8, 0, 4, 110, 97, 109, 101, 0, 3, 112, 121, 53, 11, 0, 2, 105, 100, 0, 0, 0, 4, 101, 54, 191, 237, 134, 149, 72, 253, 131, 161, 236, 210, 76, 242, 160, 253, 8, 0, 4, 116, 121, 112, 101, 0, 16, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 112, 108, 97, 121, 101, 114, 0, 8, 0, 6, 97, 99, 116, 105, 111, 110, 0, 11, 115, 104, 111, 119, 95, 101, 110, 116, 105, 116, 121, 0, 10, 0, 10, 99, 108, 105, 99, 107, 69, 118, 101, 110, 116, 8, 0, 6, 97, 99, 116, 105, 111, 110, 0, 15, 115, 117, 103, 103, 101, 115, 116, 95, 99, 111, 109, 109, 97, 110, 100, 8, 0, 5, 118, 97, 108, 117, 101, 0, 10, 47, 116, 101, 108, 108, 32, 112, 121, 53, 32, 0, 8, 0, 9, 105, 110, 115, 101, 114, 116, 105, 111, 110, 0, 3, 112, 121, 53, 8, 0, 4, 116, 101, 120, 116, 0, 3, 112, 121, 53, 0, 9, 0, 4, 119, 105, 116, 104, 10, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 1, 0, 10, 0, 10, 104, 111, 118, 101, 114, 69, 118, 101, 110, 116, 10, 0, 8, 99, 111, 110, 116, 101, 110, 116, 115, 8, 0, 2, 105, 100, 0, 25, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 100, 105, 97, 109, 111, 110, 100, 95, 112, 105, 99, 107, 97, 120, 101, 8, 0, 3, 116, 97, 103, 0, 10, 123, 68, 97, 109, 97, 103, 101, 58, 48, 125, 0, 8, 0, 6, 97, 99, 116, 105, 111, 110, 0, 9, 115, 104, 111, 119, 95, 105, 116, 101, 109, 0, 9, 0, 4, 119, 105, 116, 104, 10, 0, 0, 0, 1, 9, 0, 5, 101, 120, 116, 114, 97, 10, 0, 0, 0, 1, 8, 0, 9, 116, 114, 97, 110, 115, 108, 97, 116, 101, 0, 30, 105, 116, 101, 109, 46, 109, 105, 110, 101, 99, 114, 97, 102, 116, 46, 100, 105, 97, 109, 111, 110, 100, 95, 112, 105, 99, 107, 97, 120, 101, 0, 8, 0, 4, 116, 101, 120, 116, 0, 0, 0, 8, 0, 5, 99, 111, 108, 111, 114, 0, 5, 119, 104, 105, 116, 101, 8, 0, 9, 116, 114, 97, 110, 115, 108, 97, 116, 101, 0, 20, 99, 104, 97, 116, 46, 115, 113, 117, 97, 114, 101, 95, 98, 114, 97, 99, 107, 101, 116, 115, 0, 10, 0, 10, 104, 111, 118, 101, 114, 69, 118, 101, 110, 116, 10, 0, 8, 99, 111, 110, 116, 101, 110, 116, 115, 8, 0, 4, 110, 97, 109, 101, 0, 3, 112, 121, 53, 11, 0, 2, 105, 100, 0, 0, 0, 4, 101, 54, 191, 237, 134, 149, 72, 253, 131, 161, 236, 210, 76, 242, 160, 253, 8, 0, 4, 116, 121, 112, 101, 0, 16, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 112, 108, 97, 121, 101, 114, 0, 8, 0, 6, 97, 99, 116, 105, 111, 110, 0, 11, 115, 104, 111, 119, 95, 101, 110, 116, 105, 116, 121, 0, 10, 0, 10, 99, 108, 105, 99, 107, 69, 118, 101, 110, 116, 8, 0, 6, 97, 99, 116, 105, 111, 110, 0, 15, 115, 117, 103, 103, 101, 115, 116, 95, 99, 111, 109, 109, 97, 110, 100, 8, 0, 5, 118, 97, 108, 117, 101, 0, 10, 47, 116, 101, 108, 108, 32, 112, 121, 53, 32, 0, 8, 0, 9, 105, 110, 115, 101, 114, 116, 105, 111, 110, 0, 3, 112, 121, 53, 8, 0, 4, 116, 101, 120, 116, 0, 3, 112, 121, 53, 0, 8, 0, 9, 116, 114, 97, 110, 115, 108, 97, 116, 101, 0, 28, 99, 111, 109, 109, 97, 110, 100, 115, 46, 103, 105, 118, 101, 46, 115, 117, 99, 99, 101, 115, 115, 46, 115, 105, 110, 103, 108, 101, 0, 8, 0, 5, 99, 111, 108, 111, 114, 0, 4, 103, 114, 97, 121, 1, 0, 6, 105, 116, 97, 108, 105, 99, 1, 8, 0, 9, 116, 114, 97, 110, 115, 108, 97, 116, 101, 0, 15, 99, 104, 97, 116, 46, 116, 121, 112, 101, 46, 97, 100, 109, 105, 110, 0, 0
        ];

        let packet = ClientboundSystemChat::azalea_read(&mut Cursor::new(&bytes)).unwrap();
        assert_eq!(
            packet.content.to_string(),
            "[py5: Gave 1 [Diamond Pickaxe] to py5]".to_string()
        );
    }

    #[test]
    fn test_translate_with_string_array_c_system_chat_packet() {
        #[rustfmt::skip]
        let bytes = [
            10, 9, 0, 4, 119, 105, 116, 104, 8, 0, 0, 0, 1, 0, 14, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 100, 117, 115, 116, 8, 0, 9, 116, 114, 97, 110, 115, 108, 97, 116, 101, 0, 25, 99, 111, 109, 109, 97, 110, 100, 115, 46, 112, 97, 114, 116, 105, 99, 108, 101, 46, 115, 117, 99, 99, 101, 115, 115, 0, 0
        ];
        let packet = ClientboundSystemChat::azalea_read(&mut Cursor::new(&bytes)).unwrap();
        assert_eq!(
            packet.content.to_string(),
            "Displaying particle minecraft:dust".to_string()
        );
    }
}