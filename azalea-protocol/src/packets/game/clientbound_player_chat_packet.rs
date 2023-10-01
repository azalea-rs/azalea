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
    use std::backtrace::Backtrace;

    // you can remove or update this test if it breaks because mojang changed the
    // structure of the packet again
    #[test]
    fn test_player_chat_packet() {
        let data: [u8; 295] = [
            47, 247, 69, 164, 160, 108, 63, 217, 178, 34, 4, 161, 47, 115, 192, 126, 0, 0, 11, 72,
            101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 0, 1, 132, 209, 9, 72, 139, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 242, 1, 123, 34, 105, 110, 115, 101, 114, 116, 105, 111,
            110, 34, 58, 34, 98, 111, 116, 48, 34, 44, 34, 99, 108, 105, 99, 107, 69, 118, 101,
            110, 116, 34, 58, 123, 34, 97, 99, 116, 105, 111, 110, 34, 58, 34, 115, 117, 103, 103,
            101, 115, 116, 95, 99, 111, 109, 109, 97, 110, 100, 34, 44, 34, 118, 97, 108, 117, 101,
            34, 58, 34, 47, 116, 101, 108, 108, 32, 98, 111, 116, 48, 32, 34, 125, 44, 34, 104,
            111, 118, 101, 114, 69, 118, 101, 110, 116, 34, 58, 123, 34, 97, 99, 116, 105, 111,
            110, 34, 58, 34, 115, 104, 111, 119, 95, 101, 110, 116, 105, 116, 121, 34, 44, 34, 99,
            111, 110, 116, 101, 110, 116, 115, 34, 58, 123, 34, 116, 121, 112, 101, 34, 58, 34,
            109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 112, 108, 97, 121, 101, 114, 34, 44, 34,
            105, 100, 34, 58, 34, 50, 102, 102, 55, 52, 53, 97, 52, 45, 97, 48, 54, 99, 45, 51,
            102, 100, 57, 45, 98, 50, 50, 50, 45, 48, 52, 97, 49, 50, 102, 55, 51, 99, 48, 55, 101,
            34, 44, 34, 110, 97, 109, 101, 34, 58, 123, 34, 116, 101, 120, 116, 34, 58, 34, 98,
            111, 116, 48, 34, 125, 125, 125, 44, 34, 116, 101, 120, 116, 34, 58, 34, 98, 111, 116,
            48, 34, 125, 0,
        ];
        // just make sure it doesn't panic
        if let Err(e) = ClientboundPlayerChatPacket::read_from(&mut Cursor::new(&data)) {
            let default_backtrace = Backtrace::capture();
            let backtrace = std::error::request_ref::<Backtrace>(&e).unwrap_or(&default_backtrace);
            eprintln!("{e}\n{backtrace}");

            panic!("failed to read player chat packet");
        }
    }
}
