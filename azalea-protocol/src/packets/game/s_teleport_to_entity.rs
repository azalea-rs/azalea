use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundTeleportToEntity {
    pub uuid: Uuid,
}
