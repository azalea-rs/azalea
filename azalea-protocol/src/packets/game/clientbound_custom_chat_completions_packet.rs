use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundCustomChatCompletionsPacket {
    pub action: Action,
    pub entries: Vec<String>,
}

#[derive(Clone, Debug, McBuf)]
pub enum Action {
    Add = 0,
    Remove = 1,
    Set = 2,
}
