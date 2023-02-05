use azalea_buf::McBuf;
use azalea_chat::{
    translatable_component::{StringOrComponent, TranslatableComponent},
    FormattedText,
};
use azalea_core::BitSet;
use azalea_crypto::{MessageSignature, SignedMessageHeader};
use azalea_protocol_macros::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket, PartialEq)]
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

#[derive(Clone, Debug, McBuf, PartialEq)]
pub struct ChatTypeBound {
    pub chat_type: ChatType,
    pub name: FormattedText,
    pub target_name: Option<FormattedText>,
}

#[derive(Clone, Debug, McBuf, PartialEq)]
pub struct PlayerChatMessage {
    pub signed_header: SignedMessageHeader,
    pub header_signature: MessageSignature,
    pub signed_body: SignedMessageBody,
    pub unsigned_content: Option<FormattedText>,
    pub filter_mask: FilterMask,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct SignedMessageBody {
    pub content: ChatMessageContent,
    pub timestamp: u64,
    pub salt: u64,
    pub last_seen: Vec<LastSeenMessagesEntry>,
}

impl PlayerChatMessage {
    /// Returns the content of the message. If you want to get the
    /// 'FormattedText' for the whole message including the sender part, use
    /// [`ClientboundPlayerChatPacket::message`].
    pub fn content(&self, only_secure_chat: bool) -> FormattedText {
        if only_secure_chat {
            return self
                .signed_body
                .content
                .decorated
                .clone()
                .unwrap_or_else(|| FormattedText::from(self.signed_body.content.plain.clone()));
        }
        self.unsigned_content
            .clone()
            .unwrap_or_else(|| self.content(true))
    }
}

impl ClientboundPlayerChatPacket {
    /// Get the full message, including the sender part.
    pub fn message(&self, only_secure_chat: bool) -> FormattedText {
        let sender = self.chat_type.name.clone();
        let content = self.message.content(only_secure_chat);
        let target = self.chat_type.target_name.clone();

        let translation_key = self.chat_type.chat_type.chat_translation_key();

        let mut args = vec![
            StringOrComponent::FormattedText(sender),
            StringOrComponent::FormattedText(content),
        ];
        if let Some(target) = target {
            args.push(StringOrComponent::FormattedText(target));
        }

        let component = TranslatableComponent::new(translation_key.to_string(), args);

        FormattedText::Translatable(component)
    }
}

impl ChatType {
    #[must_use]
    pub fn chat_translation_key(&self) -> &'static str {
        match self {
            ChatType::Chat => "chat.type.text",
            ChatType::SayCommand => "chat.type.announcement",
            ChatType::MsgCommandIncoming => "commands.message.display.incoming",
            ChatType::MsgCommandOutgoing => "commands.message.display.outgoing",
            ChatType::TeamMsgCommandIncoming => "chat.type.team.text",
            ChatType::TeamMsgCommandOutgoing => "chat.type.team.sent",
            ChatType::EmoteCommand => "chat.type.emote",
        }
    }

    #[must_use]
    pub fn narrator_translation_key(&self) -> &'static str {
        match self {
            ChatType::EmoteCommand => "chat.type.emote",
            _ => "chat.type.text.narrate",
        }
    }
}

#[derive(Clone, Debug, McBuf, PartialEq)]
pub struct LastSeenMessagesEntry {
    pub profile_id: Uuid,
    pub last_signature: MessageSignature,
}

#[derive(Clone, Debug, McBuf, Default)]
pub struct LastSeenMessagesUpdate {
    pub last_seen: Vec<LastSeenMessagesEntry>,
    pub last_received: Option<LastSeenMessagesEntry>,
}

#[derive(Clone, Debug, McBuf, PartialEq)]
pub struct ChatMessageContent {
    pub plain: String,
    /// Only sent if the decorated message is different than the plain.
    pub decorated: Option<FormattedText>,
}

#[derive(Clone, Debug, McBuf, PartialEq)]
pub enum FilterMask {
    PassThrough,
    FullyFiltered,
    PartiallyFiltered(BitSet),
}

#[cfg(test)]
mod tests {
    use super::*;
    use azalea_buf::McBufReadable;
    use std::io::Cursor;

    #[test]
    fn test_chat_type() {
        let chat_type_enum = ChatType::read_from(&mut Cursor::new(&[0x06])).unwrap();
        assert_eq!(chat_type_enum, ChatType::EmoteCommand);
        assert_eq!(
            ChatType::read_from(&mut Cursor::new(&[0x07])).unwrap(),
            ChatType::Chat
        );
    }
}
