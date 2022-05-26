use azalea_chat::component::Component;
use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundServerDataPacket {
    pub motd: Option<Component>,
    pub icon_base64: Option<String>,
    pub previews_chat: bool,
}
