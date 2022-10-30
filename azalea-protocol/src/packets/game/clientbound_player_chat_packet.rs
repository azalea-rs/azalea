use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use azalea_chat::{
    translatable_component::{StringOrComponent, TranslatableComponent},
    Component,
};
use azalea_core::BitSet;
use azalea_crypto::MessageSignature;
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerChatPacket {
    pub sender: Uuid,
    #[var]
    pub index: u32,
    pub signature: Option<MessageSignature>,
    pub body: PackedSignedMessageBody,
    pub unsigned_content: Option<Component>,
    pub filter_mask: FilterMask,
    pub chat_type: ChatTypeBound,
}

#[derive(Clone, Debug, McBuf)]
pub struct PackedSignedMessageBody {
    pub content: String,
    pub timestamp: u64,
    pub salt: u64,
    pub last_seen: PackedLastSeenMessages,
}

#[derive(Clone, Debug, McBuf)]
pub struct PackedLastSeenMessages {
    pub entries: PackedMessageSignature,
}

/// Messages can be deleted by either their signature or message id.
#[derive(Clone, Debug)]
pub enum PackedMessageSignature {
    Signature(MessageSignature),
    Id(u32),
}

#[derive(Clone, Debug, McBuf)]
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

#[derive(Clone, Debug, McBuf)]
pub struct ChatTypeBound {
    pub chat_type: ChatType,
    pub name: Component,
    pub target_name: Option<Component>,
}

// must be in Client
pub struct MessageSignatureCache {
    pub entries: Vec<Option<MessageSignature>>,
}

// impl MessageSignatureCache {
//     pub fn unpacker(&self) -> impl Fn(u32) -> Option<SignedMessageBody> {

//     }
// }

// impl PackedSignedMessageBody {
//     pub fn unpack(&self, unpacker: impl Fn(u32) -> Option<SignedMessageBody>) {}
// }

impl ClientboundPlayerChatPacket {
    /// Returns the content of the message. If you want to get the Component
    /// for the whole message including the sender part, use
    /// [`ClientboundPlayerChatPacket::message`].
    pub fn content(&self) -> Component {
        self.unsigned_content
            .clone()
            .unwrap_or_else(|| Component::from(self.body.content.clone()))
    }

    /// Get the full message, including the sender part.
    pub fn message(&self) -> Component {
        let sender = self.chat_type.name.clone();
        let content = self.content();
        let target = self.chat_type.target_name.clone();

        let translation_key = self.chat_type.chat_type.chat_translation_key();

        let mut args = vec![
            StringOrComponent::Component(sender),
            StringOrComponent::Component(content),
        ];
        if let Some(target) = target {
            args.push(StringOrComponent::Component(target));
        }

        let component = TranslatableComponent::new(translation_key.to_string(), args);

        Component::Translatable(component)
    }
}

impl ChatType {
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

    pub fn narrator_translation_key(&self) -> &'static str {
        match self {
            ChatType::Chat => "chat.type.text.narrate",
            ChatType::SayCommand => "chat.type.text.narrate",
            ChatType::MsgCommandIncoming => "chat.type.text.narrate",
            ChatType::MsgCommandOutgoing => "chat.type.text.narrate",
            ChatType::TeamMsgCommandIncoming => "chat.type.text.narrate",
            ChatType::TeamMsgCommandOutgoing => "chat.type.text.narrate",
            ChatType::EmoteCommand => "chat.type.emote",
        }
    }
}

impl McBufReadable for PackedMessageSignature {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = u32::var_read_from(buf)?;
        if id == 0 {
            let full_signature = MessageSignature::read_from(buf)?;
            Ok(PackedMessageSignature::Signature(full_signature))
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
