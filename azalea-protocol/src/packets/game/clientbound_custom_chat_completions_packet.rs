use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundCustomChatCompletionsPacket {
    pub action: Action,
    pub entries: Vec<String>,
}

#[derive(Clone, Debug, McBuf, Copy)]
pub enum Action {
    Add = 0,
    Remove = 1,
    Set = 2,
}
