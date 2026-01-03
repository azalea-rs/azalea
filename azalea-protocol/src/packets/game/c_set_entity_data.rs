use azalea_buf::AzBuf;
use azalea_entity::EntityMetadataItems;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
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
