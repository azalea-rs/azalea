use azalea_buf::McBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundOpenScreen {
    #[var]
    pub container_id: u32,
    pub menu_type: azalea_registry::MenuKind,
    pub title: FormattedText,
}
