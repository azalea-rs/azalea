use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundServerDataPacket {
pub motd: FormattedText,
pub icon_bytes: Option<(u32, Vec<u8>)>,
}