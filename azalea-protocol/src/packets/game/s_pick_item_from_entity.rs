use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundPickItemFromEntity {
    #[var]
    pub id: u32,
    pub include_data: bool,
}
