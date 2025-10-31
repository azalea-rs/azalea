use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ServerboundGamePacket)]
pub struct ServerboundContainerClose {
    #[var]
    pub container_id: i32,
}
