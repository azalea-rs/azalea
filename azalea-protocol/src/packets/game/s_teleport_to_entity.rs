use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundTeleportToEntity {
    pub uuid: Uuid,
}
