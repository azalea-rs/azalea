use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerInfoPacket {
    pub action: Action,
    pub entries: Vec<todo!()>,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Action {
    AddPlayer = 0,
    UpdateGameMode = 1,
    UpdateLatency = 2,
    UpdateDisplayName = 3,
    RemovePlayer = 4,
}
