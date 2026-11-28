use azalea_buf::AzBuf;
use azalea_chat::{
    FormattedText,
    component::translatable::{PrimitiveOrComponent, TranslatableComponent},
};
use azalea_protocol_macros::ClientboundGamePacket;

use super::c_player_chat::ChatTypeBound;

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
    #[must_use]
    pub fn message(&self) -> FormattedText {
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

        let translation_key = self.chat_type.translation_key();
        let component = TranslatableComponent::new(translation_key.to_owned(), args);

        FormattedText::Translatable(component)
    }
}
