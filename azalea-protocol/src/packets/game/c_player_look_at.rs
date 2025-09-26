use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundPlayerLookAt {
    pub from_anchor: Anchor,
    pub pos: Vec3,
    pub entity: Option<AtEntity>,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq)]
pub enum Anchor {
    Feet = 0,
    Eyes = 1,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct AtEntity {
    #[var]
    pub entity: u32,
    pub to_anchor: Anchor,
}
