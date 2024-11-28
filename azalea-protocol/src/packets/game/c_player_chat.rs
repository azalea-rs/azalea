use std::io::{Cursor, Write};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};
use azalea_chat::{
    translatable_component::{StringOrComponent, TranslatableComponent},
    FormattedText,
};
use azalea_core::bitset::BitSet;
use azalea_crypto::MessageSignature;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::{ChatType, OptionalRegistry};
use uuid::Uuid;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket, PartialEq)]
pub struct ClientboundPlayerChat {
    pub sender: Uuid,
    #[var]
    pub index: u32,
    pub signature: Option<MessageSignature>,
    pub body: PackedSignedMessageBody,
    pub unsigned_content: Option<FormattedText>,
    pub filter_mask: FilterMask,
    pub chat_type: ChatTypeBound,
}

#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct PackedSignedMessageBody {
    // the error is here, for some reason it skipped a byte earlier and here
    // it's reading `0` when it should be `11`
    pub content: String,
    pub timestamp: u64,
    pub salt: u64,
    pub last_seen: PackedLastSeenMessages,
}

#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct PackedLastSeenMessages {
    pub entries: Vec<PackedMessageSignature>,
}

/// Messages can be deleted by either their signature or message id.
#[derive(Clone, Debug, PartialEq)]
pub enum PackedMessageSignature {
    Signature(Box<MessageSignature>),
    Id(u32),
}

#[derive(Clone, Debug, PartialEq, AzBuf)]
pub enum FilterMask {
    PassThrough,
    FullyFiltered,
    PartiallyFiltered(BitSet),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChatTypeBound {
    pub chat_type: ChatType,
    pub name: FormattedText,
    pub target_name: Option<FormattedText>,
}
impl AzaleaRead for ChatTypeBound {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let Some(chat_type) = OptionalRegistry::<ChatType>::azalea_read(buf)?.0 else {
            return Err(BufReadError::Custom("ChatType cannot be None".to_owned()));
        };
        let name = FormattedText::azalea_read(buf)?;
        let target_name = Option::<FormattedText>::azalea_read(buf)?;

        Ok(ChatTypeBound {
            chat_type,
            name,
            target_name,
        })
    }
}
impl AzaleaWrite for ChatTypeBound {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        OptionalRegistry(Some(self.chat_type)).azalea_write(buf)?;
        self.name.azalea_write(buf)?;
        self.target_name.azalea_write(buf)?;
        Ok(())
    }
}

// must be in Client
#[derive(Clone, Debug, PartialEq)]
pub struct MessageSignatureCache {
    pub entries: Vec<Option<MessageSignature>>,
}

impl ClientboundPlayerChat {
    /// Returns the content of the message. If you want to get the FormattedText
    /// for the whole message including the sender part, use
    /// [`ClientboundPlayerChat::message`].
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

impl AzaleaRead for PackedMessageSignature {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = u32::azalea_read_var(buf)?;
        if id == 0 {
            let full_signature = MessageSignature::azalea_read(buf)?;

            Ok(PackedMessageSignature::Signature(Box::new(full_signature)))
        } else {
            Ok(PackedMessageSignature::Id(id - 1))
        }
    }
}
impl AzaleaWrite for PackedMessageSignature {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            PackedMessageSignature::Signature(full_signature) => {
                0u32.azalea_write_var(buf)?;
                full_signature.azalea_write(buf)?;
            }
            PackedMessageSignature::Id(id) => {
                (id + 1).azalea_write_var(buf)?;
            }
        }
        Ok(())
    }
}
