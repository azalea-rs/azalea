use super::clientbound_player_chat_packet::ChatTypeBound;
use azalea_buf::McBuf;
use azalea_chat::{
    translatable_component::{StringOrComponent, TranslatableComponent},
    FormattedText,
};
use azalea_protocol_macros::ClientboundGamePacket;

// A disguised chat packet is basically the same as a normal
// [`ClientboundPlayerChatPacket`], except that it doesn't have any of the chat
// signing things. Vanilla servers use this when messages are sent from the
// console.
#[derive(Clone, Debug, McBuf, ClientboundGamePacket, PartialEq)]
pub struct ClientboundDisguisedChatPacket {
    pub message: FormattedText,
    pub chat_type: ChatTypeBound,
}

impl ClientboundDisguisedChatPacket {
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
