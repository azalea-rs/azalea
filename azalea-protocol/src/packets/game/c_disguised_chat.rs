use azalea_buf::AzBuf;
use azalea_chat::{
    FormattedText,
    translatable_component::{PrimitiveOrComponent, TranslatableComponent},
};
use azalea_core::registry_holder::RegistryHolder;
use azalea_protocol_macros::ClientboundGamePacket;

use super::c_player_chat::ChatTypeBound;
use crate::packets::game::c_player_chat::GUESSED_DEFAULT_REGISTRIES_FOR_CHAT;

/// Similar to a [`ClientboundPlayerChat`](super::ClientboundPlayerChat), but
/// without chat signing.
///
/// Vanilla servers use this packet when messages are sent from the console.
#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundDisguisedChat {
    pub message: FormattedText,
    pub chat_type: ChatTypeBound,
}

impl ClientboundDisguisedChat {
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
        let content = self.message.clone();
        let target = self.chat_type.target_name.clone();

        let mut args = vec![
            PrimitiveOrComponent::FormattedText(sender),
            PrimitiveOrComponent::FormattedText(content),
        ];
        if let Some(target) = target {
            args.push(PrimitiveOrComponent::FormattedText(target));
        }

        let translation_key = self.chat_type.translation_key(registries);
        let component = TranslatableComponent::new(translation_key.to_owned(), args);

        FormattedText::Translatable(component)
    }
}
