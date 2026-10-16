use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundPickItemFromBlock {
    pub pos: BlockPos,
    pub include_data: bool,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::AzBuf;

    use super::*;

    #[test]
    fn test_pick_item_from_block_roundtrip() {
        let packet = ServerboundPickItemFromBlock {
            pos: BlockPos::new(4, -61, -7),
            include_data: true,
        };
        let mut bytes = Vec::new();
        packet.azalea_write(&mut bytes).unwrap();

        // packed long + bool
        assert_eq!(bytes.len(), 8 + 1);

        let read = ServerboundPickItemFromBlock::azalea_read(&mut Cursor::new(bytes.as_slice())).unwrap();
        assert_eq!(read, packet);
    }
}
