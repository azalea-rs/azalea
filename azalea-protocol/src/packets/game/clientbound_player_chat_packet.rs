use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use azalea_chat::component::Component;
use azalea_core::BitSet;
use azalea_crypto::{MessageSignature, SignedMessageHeader};
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Read, Write};
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerChatPacket {
    pub message: PlayerChatMessage,
    pub chat_type: ChatTypeBound,
}

#[derive(Copy, Clone, Debug, McBuf, PartialEq, Eq)]
pub enum ChatType {
    Chat = 0,
    SayCommand = 1,
    MsgCommandIncoming = 2,
    MsgCommandOutgoing = 3,
    TeamMsgCommandIncoming = 4,
    TeamMsgCommandOutgoing = 5,
    EmoteCommand = 6,
}

#[derive(Clone, Debug, McBuf)]
pub struct ChatTypeBound {
    pub chat_type: ChatType,
    pub name: Component,
    pub target_name: Option<Component>,
}

#[derive(Clone, Debug, McBuf)]
pub struct PlayerChatMessage {
    pub signed_header: SignedMessageHeader,
    pub header_signature: MessageSignature,
    pub signed_body: SignedMessageBody,
    pub unsigned_content: Option<Component>,
    pub filter_mask: FilterMask,
}

#[derive(Clone, Debug, McBuf)]
pub struct SignedMessageBody {
    pub content: ChatMessageContent,
    pub timestamp: u64,
    pub salt: u64,
    pub last_seen: Vec<LastSeenMessagesEntry>,
}

impl PlayerChatMessage {
    pub fn message(&self, only_secure_chat: bool) -> Component {
        if only_secure_chat {
            return self
                .signed_body
                .content
                .decorated
                .clone()
                .unwrap_or(Component::from(self.signed_body.content.plain.clone()));
        }
        self.unsigned_content.clone().unwrap_or(self.message(true))
    }
}

#[derive(Clone, Debug, McBuf)]
pub struct LastSeenMessagesEntry {
    pub profile_id: Uuid,
    pub last_signature: MessageSignature,
}

#[derive(Clone, Debug, McBuf)]
pub struct LastSeenMessagesUpdate {
    pub last_seen: Vec<LastSeenMessagesEntry>,
    pub last_received: Option<LastSeenMessagesEntry>,
}

#[derive(Clone, Debug, McBuf)]
pub struct ChatMessageContent {
    pub plain: String,
    /// Only sent if the decorated message is different than the plain.
    pub decorated: Option<Component>,
}

#[derive(Clone, Debug)]
pub enum FilterMask {
    PassThrough,
    FullyFiltered,
    PartiallyFiltered(BitSet),
}

impl McBufReadable for FilterMask {
    fn read_from(buf: &mut impl Read) -> Result<Self, BufReadError> {
        let filter_mask = u32::var_read_from(buf)?;
        match filter_mask {
            0 => Ok(FilterMask::PassThrough),
            1 => Ok(FilterMask::FullyFiltered),
            2 => Ok(FilterMask::PartiallyFiltered(BitSet::read_from(buf)?)),
            _ => Err(BufReadError::UnexpectedEnumVariant {
                id: filter_mask as i32,
            }),
        }
    }
}
impl McBufWritable for FilterMask {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            FilterMask::PassThrough => 0u32.var_write_into(buf)?,
            FilterMask::FullyFiltered => 1u32.var_write_into(buf)?,
            FilterMask::PartiallyFiltered(bits) => {
                2u32.var_write_into(buf)?;
                bits.write_into(buf)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_type() {
        let chat_type_enum = ChatType::read_from(&mut &[0x06][..]).unwrap();
        assert_eq!(chat_type_enum, ChatType::EmoteCommand);
        assert!(ChatType::read_from(&mut &[0x07][..]).is_err());
    }
}
