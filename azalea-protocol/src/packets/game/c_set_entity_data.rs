use azalea_buf::AzBuf;
use azalea_core::entity_id::MinecraftEntityId;
use azalea_entity::EntityMetadataItems;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundSetEntityData {
    #[var]
    pub id: MinecraftEntityId,
    pub packed_items: EntityMetadataItems,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::AzBuf;

    use crate::packets::game::ClientboundSetEntityData;

    #[test]
    fn read_set_entity_data() {
        // from hypixel
        #[rustfmt::skip]
        let contents = [173, 179, 148, 8, 10, 17, 1, 28, 38, 124, 175, 198, 255];
        let mut buf = Cursor::new(contents.as_slice());
        let _packet = ClientboundSetEntityData::azalea_read(&mut buf).unwrap();

        assert_eq!(buf.position(), contents.len() as u64);
    }
}
