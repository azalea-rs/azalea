use azalea_buf::AzBuf;
use azalea_entity::EntityMetadataItems;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundSetEntityData {
    #[var]
    pub id: MinecraftEntityId,
    pub packed_items: EntityMetadataItems,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::AzaleaRead;

    use super::*;

    #[test]
    fn test_read_write_hypixel_entity_data() {
        let contents = [161, 226, 1, 10, 18, 1, 20, 38, 124, 175, 198, 255];
        let mut buf = Cursor::new(contents.as_slice());
        let packet = ClientboundSetEntityData::azalea_read(&mut buf).unwrap();
        println!("{packet:?}");

        assert_eq!(buf.position(), contents.len() as u64);

        let mut buf = Vec::new();
        packet.write(&mut buf).unwrap();
        assert_eq!(buf, contents);
    }

    #[test]
    fn test_read_hypixel_entity_data_2() {
        let contents = [
            161, 21, 2, 6, 0, 5, 8, 0, 6, 21, 0, 7, 1, 0, 1, 1, 172, 2, 3, 8, 0, 4, 8, 0, 9, 1, 0,
            0, 0, 0, 8, 7, 1, 186, 9, 2, 0, 5, 10, 9, 0, 5, 101, 120, 116, 114, 97, 10, 0, 0, 0, 1,
            8, 0, 5, 99, 111, 108, 111, 114, 0, 9, 100, 97, 114, 107, 95, 97, 113, 117, 97, 1, 0,
            4, 98, 111, 108, 100, 1, 8, 0, 4, 116, 101, 120, 116, 0, 18, 67, 108, 111, 117, 100,
            32, 82, 101, 103, 101, 110, 101, 114, 97, 116, 105, 111, 110, 0, 8, 0, 4, 116, 101,
            120, 116, 0, 0, 1, 0, 6, 105, 116, 97, 108, 105, 99, 0, 0, 41, 1, 31, 0, 0, 0, 255,
        ];
        let mut buf = Cursor::new(contents.as_slice());
        let packet = ClientboundSetEntityData::azalea_read(&mut buf).unwrap();
        println!("{packet:?}");

        assert_eq!(buf.position(), contents.len() as u64);
    }

    #[test]
    fn test_read_6b6t_entity_data() {
        let contents = [
            254, 180, 160, 8, 11, 33, 190, 230, 102, 102, 0, 0, 0, 0, 191, 0, 0, 0, 12, 33, 63,
            102, 102, 102, 63, 25, 153, 154, 63, 102, 102, 102, 23, 14, 234, 64, 255,
        ];
        let mut buf = Cursor::new(contents.as_slice());
        let packet = ClientboundSetEntityData::azalea_read(&mut buf).unwrap();
        println!("{packet:?}");

        assert_eq!(buf.position(), contents.len() as u64);
    }
}
