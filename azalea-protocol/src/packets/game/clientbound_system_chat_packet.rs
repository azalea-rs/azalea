use azalea_chat::component::Component;
use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSystemChatPacket {
    pub content: Component,
    #[var]
    pub type_id: i32,
}
