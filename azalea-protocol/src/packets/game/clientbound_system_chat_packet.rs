use azalea_buf::McBuf;
use azalea_chat::component::Component;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSystemChatPacket {
    pub content: Component,
    #[var]
    pub type_id: i32,
}
