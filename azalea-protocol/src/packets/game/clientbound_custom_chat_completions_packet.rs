use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundCustomChatCompletionsPacket {
    pub action: Action,
    pub entries: Vec<String>,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Action {
    Add = 0,
    Remove = 1,
    Set = 2,
}
