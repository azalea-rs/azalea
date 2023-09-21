use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::packets::common::CommonPlayerSpawnInfo;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundRespawnPacket {
    pub common: CommonPlayerSpawnInfo,
    pub data_to_keep: u8,
}
