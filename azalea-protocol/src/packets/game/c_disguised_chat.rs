use azalea_buf::AzBuf;
use azalea_chat::{
    translatable_component::{StringOrComponent, TranslatableComponent},
    FormattedText,
};
use azalea_protocol_macros::ClientboundGamePacket;

use super::c_player_chat::ChatTypeBound;

// A disguised chat packet is basically the same as a normal
// [`ClientboundPlayerChat`], except that it doesn't have any of the chat
// signing things. Vanilla servers use this when messages are sent from the
// console.
#[derive(Clone, Debug, AzBuf, ClientboundGamePacket, PartialEq)]
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
