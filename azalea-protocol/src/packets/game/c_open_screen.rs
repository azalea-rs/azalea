use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundOpenScreen {
    #[var]
    pub container_id: i32,
    pub menu_type: azalea_registry::MenuKind,
    pub title: FormattedText,
}
