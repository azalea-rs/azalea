use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use azalea_chat::{
    translatable_component::{StringOrComponent, TranslatableComponent},
    FormattedText,
};
use azalea_core::bitset::BitSet;
use azalea_crypto::MessageSignature;
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket, PartialEq)]
pub struct ClientboundPlayerChatPacket {
    pub sender: Uuid,
    #[var]
    pub index: u32,
    pub signature: Option<MessageSignature>,
    pub body: PackedSignedMessageBody,
    pub unsigned_content: Option<FormattedText>,
    pub filter_mask: FilterMask,
    pub chat_type: ChatTypeBound,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct PackedSignedMessageBody {
    // the error is here, for some reason it skipped a byte earlier and here
    // it's reading `0` when it should be `11`
    pub content: String,
    pub timestamp: u64,
    pub salt: u64,
    pub last_seen: PackedLastSeenMessages,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct PackedLastSeenMessages {
    pub entries: Vec<PackedMessageSignature>,
}

/// Messages can be deleted by either their signature or message id.
#[derive(Clone, Debug, PartialEq)]
pub enum PackedMessageSignature {
    Signature(Box<MessageSignature>),
    Id(u32),
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub enum FilterMask {
    PassThrough,
    FullyFiltered,
    PartiallyFiltered(BitSet),
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

// must be in Client
#[derive(Clone, Debug, PartialEq)]
pub struct MessageSignatureCache {
    pub entries: Vec<Option<MessageSignature>>,
}

// impl MessageSignatureCache {
//     pub fn unpacker(&self) -> impl Fn(u32) -> Option<SignedMessageBody> {

//     }
// }

// impl PackedSignedMessageBody {
//     pub fn unpack(&self, unpacker: impl Fn(u32) -> Option<SignedMessageBody>)
// {} }

impl ClientboundPlayerChatPacket {
    /// Returns the content of the message. If you want to get the FormattedText
    /// for the whole message including the sender part, use
    /// [`ClientboundPlayerChatPacket::message`].
    #[must_use]
    pub fn content(&self) -> FormattedText {
        self.unsigned_content
            .clone()
            .unwrap_or_else(|| FormattedText::from(self.body.content.clone()))
    }

    /// Get the full message, including the sender part.
    #[must_use]
    pub fn message(&self) -> FormattedText {
        let sender = self.chat_type.name.clone();
        let content = self.content();
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

impl McBufReadable for PackedMessageSignature {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = u32::var_read_from(buf)?;
        if id == 0 {
            let full_signature = MessageSignature::read_from(buf)?;

            Ok(PackedMessageSignature::Signature(Box::new(full_signature)))
        } else {
            Ok(PackedMessageSignature::Id(id - 1))
        }
    }
}
impl McBufWritable for PackedMessageSignature {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            PackedMessageSignature::Signature(full_signature) => {
                0u32.var_write_into(buf)?;
                full_signature.write_into(buf)?;
            }
            PackedMessageSignature::Id(id) => {
                (id + 1).var_write_into(buf)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_player_chat_packet() {
        let mut bytes = Cursor::new(
            &[
                55, 186, 28, 76, 92, 167, 177, 75, 188, 158, 200, 179, 191, 227, 16, 171, 145, 0,
                0, 4, 116, 101, 115, 116, 0, 0, 1, 140, 178, 225, 89, 103, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 10, 10, 0, 10, 104, 111, 118, 101, 114, 69, 118, 101, 110, 116, 10, 0,
                8, 99, 111, 110, 116, 101, 110, 116, 115, 8, 0, 4, 110, 97, 109, 101, 0, 12, 75,
                97, 115, 117, 109, 105, 77, 97, 114, 105, 115, 97, 11, 0, 2, 105, 100, 0, 0, 0, 4,
                186, 28, 76, 92, 167, 177, 75, 188, 158, 200, 179, 191, 227, 16, 171, 145, 8, 0, 4,
                116, 121, 112, 101, 0, 16, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 112, 108,
                97, 121, 101, 114, 0, 8, 0, 6, 97, 99, 116, 105, 111, 110, 0, 11, 115, 104, 111,
                119, 95, 101, 110, 116, 105, 116, 121, 0, 10, 0, 10, 99, 108, 105, 99, 107, 69,
                118, 101, 110, 116, 8, 0, 6, 97, 99, 116, 105, 111, 110, 0, 15, 115, 117, 103, 103,
                101, 115, 116, 95, 99, 111, 109, 109, 97, 110, 100, 8, 0, 5, 118, 97, 108, 117,
                101, 0, 19, 47, 116, 101, 108, 108, 32, 75, 97, 115, 117, 109, 105, 77, 97, 114,
                105, 115, 97, 32, 0, 9, 0, 5, 101, 120, 116, 114, 97, 8, 0, 0, 0, 3, 0, 0, 0, 12,
                75, 97, 115, 117, 109, 105, 77, 97, 114, 105, 115, 97, 0, 0, 8, 0, 9, 105, 110,
                115, 101, 114, 116, 105, 111, 110, 0, 12, 75, 97, 115, 117, 109, 105, 77, 97, 114,
                105, 115, 97, 8, 0, 4, 116, 101, 120, 116, 0, 0, 0, 0,
            ][..],
        );
        let _packet = ClientboundPlayerChatPacket::read_from(&mut bytes).unwrap();
    }
}
