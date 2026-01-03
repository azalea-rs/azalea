use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::builtin::MenuKind;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundOpenScreen {
    #[var]
    pub container_id: i32,
    pub menu_type: MenuKind,
    pub title: FormattedText,
}
