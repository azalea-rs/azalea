use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundServerDataPacket {
    pub motd: Option<Component>,
    pub icon_base64: Option<String>,
    pub previews_chat: bool,
    pub enforces_secure_chat: bool,
}
