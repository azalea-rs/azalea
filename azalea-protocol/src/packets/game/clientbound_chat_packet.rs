use azalea_chat::component::Component;
use packet_macros::{GamePacket, McBuf};
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundChatPacket {
    pub message: Component,
    pub type_: ChatType,
    pub sender: Uuid,
}

#[derive(Clone, Debug, Copy, McBuf)]
pub enum ChatType {
    Chat = 0,
    System = 1,
    GameInfo = 2,
}
