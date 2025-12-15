use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::packets::common::CommonPlayerSpawnInfo;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundRespawn {
    pub common: CommonPlayerSpawnInfo,
    pub data_to_keep: u8,
}
