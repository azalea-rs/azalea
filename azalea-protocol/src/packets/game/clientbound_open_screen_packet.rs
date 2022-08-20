use azalea_buf::McBuf;
use azalea_chat::component::Component;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundOpenScreenPacket {
    #[var]
    pub container_id: u32,
    // TODO: have an enum of this
    #[var]
    pub menu_type: u32,
    pub title: Component,
}
