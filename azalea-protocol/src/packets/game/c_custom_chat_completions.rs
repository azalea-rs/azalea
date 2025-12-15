use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundCustomChatCompletions {
    pub action: Action,
    pub entries: Vec<String>,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Add = 0,
    Remove = 1,
    Set = 2,
}
