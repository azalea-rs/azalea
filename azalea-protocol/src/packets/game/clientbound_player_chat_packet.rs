use azalea_chat::component::Component;
use azalea_crypto::SaltSignaturePair;
use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundPlayerChatPacket {
    pub signed_content: Component,
    pub unsigned_content: Option<Component>,
    #[var]
    pub type_id: i32,
    pub sender: ChatSender,
    pub timestamp: u64,
    pub salt_signature: SaltSignaturePair,
}

#[derive(Clone, Debug, McBuf)]
pub struct ChatSender {
    pub uuid: uuid::Uuid,
    pub name: Component,
    pub team_name: Option<Component>,
}
