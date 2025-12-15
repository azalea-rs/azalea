use std::{
    io::{self, Cursor, Write},
    sync::LazyLock,
};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};
use azalea_chat::{
    FormattedText,
    translatable_component::{PrimitiveOrComponent, TranslatableComponent},
};
use azalea_core::{
    bitset::BitSet,
    data_registry::DataRegistryWithKey,
    registry_holder::{RegistryHolder, RegistryType},
};
use azalea_crypto::signing::MessageSignature;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::{
    DataRegistryKey, Holder,
    data::{ChatKind, ChatKindKey},
    identifier::Identifier,
};
use simdnbt::owned::NbtCompound;
use uuid::Uuid;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundPlayerChat {
    #[var]
    pub global_index: u32,
    pub sender: Uuid,
    #[var]
    pub index: u32,
    pub signature: Option<MessageSignature>,
    pub body: PackedSignedMessageBody,
    pub unsigned_content: Option<FormattedText>,
    pub filter_mask: FilterMask,
    pub chat_type: ChatTypeBound,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct PackedSignedMessageBody {
    // the error is here, for some reason it skipped a byte earlier and here
    // it's reading `0` when it should be `11`
    pub content: String,
    pub timestamp: u64,
    pub salt: u64,
    pub last_seen: PackedLastSeenMessages,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct PackedLastSeenMessages {
    pub entries: Vec<PackedMessageSignature>,
}

/// Messages can be deleted by either their signature or message ID.
#[derive(Clone, Debug, PartialEq)]
pub enum PackedMessageSignature {
    Signature(Box<MessageSignature>),
    Id(u32),
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub enum FilterMask {
    PassThrough,
    FullyFiltered,
    PartiallyFiltered(BitSet),
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct ChatTypeBound {
    pub chat_type: Holder<ChatKind, DirectChatType>,
    pub name: FormattedText,
    pub target_name: Option<FormattedText>,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct DirectChatType {
    pub chat: ChatTypeDecoration,
    pub narration: ChatTypeDecoration,
}
#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct ChatTypeDecoration {
    pub translation_key: String,
    pub parameters: Vec<ChatTypeDecorationParameter>,
    pub style: NbtCompound,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq)]
pub enum ChatTypeDecorationParameter {
    Sender = 0,
    Target = 1,
    Content = 2,
}

// must be in Client
#[derive(Clone, Debug, PartialEq)]
pub struct MessageSignatureCache {
    pub entries: Vec<Option<MessageSignature>>,
}

/// A `RegistryHolder` that only has the `chat_type` registry (without values),
/// with the keys being in the default order for vanilla servers.
///
/// This is used when we call [`ClientboundPlayerChat::message`] without also
/// passing registries.
pub static GUESSED_DEFAULT_REGISTRIES_FOR_CHAT: LazyLock<RegistryHolder> =
    LazyLock::new(|| RegistryHolder {
        extra: [(
            Identifier::new("chat_type"),
            RegistryType {
                map: ChatKindKey::ALL
                    .iter()
                    .map(|k| (k.clone().into_ident(), NbtCompound::new()))
                    .collect(),
            },
        )]
        .into_iter()
        .collect(),
        ..Default::default()
    });

impl ClientboundPlayerChat {
    /// Returns the content of the message.
    ///
    /// If you want to get the [`FormattedText`] for the whole message including
    /// the sender part, use [`ClientboundPlayerChat::message`].
    #[must_use]
    pub fn content(&self) -> FormattedText {
        self.unsigned_content
            .clone()
            .unwrap_or_else(|| FormattedText::from(self.body.content.clone()))
    }

    /// Get the full message, including the sender part.
    ///
    /// Note that the returned message may be incorrect on servers that
    /// customize the chat type registry. Consider using
    /// [`Self::message_using_registries`] if you'd like to avoid that
    /// problem.
    #[must_use]
    pub fn message(&self) -> FormattedText {
        self.message_using_registries(&GUESSED_DEFAULT_REGISTRIES_FOR_CHAT)
    }

    /// Get the full message, including the sender part, while ensuring that the
    /// message chat type is correct based on the server's registries.
    ///
    /// Also see [`Self::message`].
    #[must_use]
    pub fn message_using_registries(&self, registries: &RegistryHolder) -> FormattedText {
        let sender = self.chat_type.name.clone();
        let content = self.content();
        let target = self.chat_type.target_name.clone();

        let mut args = vec![
            PrimitiveOrComponent::FormattedText(sender),
            PrimitiveOrComponent::FormattedText(content),
        ];
        if let Some(target) = target {
            args.push(PrimitiveOrComponent::FormattedText(target));
        }

        // TODO: implement chat type registry and apply the styles from it here
        let translation_key = self.chat_type.translation_key(registries);
        let component = TranslatableComponent::new(translation_key.to_owned(), args);

        FormattedText::Translatable(component)
    }
}

impl ChatTypeBound {
    pub fn translation_key(&self, registries: &RegistryHolder) -> &str {
        match &self.chat_type {
            Holder::Reference(r) => r
                .key(registries)
                .map(|r| r.chat_translation_key())
                .unwrap_or("chat.type.text"),
            Holder::Direct(d) => d.chat.translation_key.as_str(),
        }
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
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
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
