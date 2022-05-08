use azalea_chat::component::Component;
use packet_macros::{GamePacket, McBufReadable, McBufWritable};
use uuid::Uuid;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundChatPacket {
    pub message: Component,
    pub type_: ChatType,
    pub sender: Uuid,
}

#[derive(Clone, Debug, Copy, McBufReadable, McBufWritable)]
pub enum ChatType {
    Chat = 0,
    System = 1,
    GameInfo = 2,
}
