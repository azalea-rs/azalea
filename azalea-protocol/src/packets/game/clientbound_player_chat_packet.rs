use azalea_buf::McBuf;
use azalea_chat::component::Component;
use azalea_crypto::{MessageSignature, SignedMessageHeader};
use packet_macros::GamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundPlayerChatPacket {
    pub message: PlayerChatMessage,
    pub chat_type: ChatTypeBound,
}

#[derive(Copy, Clone, Debug, McBuf)]
pub enum ChatType {
    Chat = 0,
    SayCommand = 1,
    MsgCommandIncoming = 2,
    MsgCommandOutgoing = 3,
    TeamMsgCommandIncoming = 4,
    EmoteCommand = 5,
}

#[derive(Clone, Debug, McBuf)]
pub struct ChatTypeBound {
    pub chat_type: ChatType,
    pub name: Component,
    pub target_name: Component,
}

#[derive(Clone, Debug, McBuf)]
pub struct PlayerChatMessage {
    pub signed_header: SignedMessageHeader,
    pub header_signature: MessageSignature,
    pub signed_body: SignedMessageBody,
    pub unsigned_content: Option<Component>,
}

#[derive(Clone, Debug, McBuf)]
pub struct SignedMessageBody {
    pub content: Component,
    pub timestamp: u64,
    pub salt: u64,
    pub last_seen: Vec<LastSeen>,
}

#[derive(Clone, Debug, McBuf)]
pub struct LastSeen {
    pub profile_id: Uuid,
    pub last_signature: MessageSignature,
}
