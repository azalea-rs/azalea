use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerLookAtPacket {
    pub from_anchor: Anchor,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub entity: Option<AtEntity>,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Anchor {
    Feet = 0,
    Eyes = 1,
}

#[derive(McBuf, Clone, Debug)]
pub struct AtEntity {
    #[var]
    pub entity: u32,
    pub to_anchor: Anchor,
}
