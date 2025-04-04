use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundContainerButtonClick {
    #[var]
    pub container_id: i32,
    #[var]
    pub button_id: u32,
}
