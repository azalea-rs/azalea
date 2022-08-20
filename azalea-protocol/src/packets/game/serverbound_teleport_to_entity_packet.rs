use azalea_buf::McBuf;
use packet_macros::ServerboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundTeleportToEntityPacket {
    pub uuid: Uuid,
}
