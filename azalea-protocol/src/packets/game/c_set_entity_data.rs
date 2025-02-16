use azalea_buf::AzBuf;
use azalea_entity::EntityMetadataItems;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
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
        println!("{:?}", packet);

        assert_eq!(buf.position(), contents.len() as u64);

        let mut buf = Vec::new();
        packet.write(&mut buf).unwrap();
        assert_eq!(buf, contents);
    }
}
