use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetScorePacket {
pub owner: String,
pub objective_name: String,
#[var]
pub score: u32,
pub display: Option<FormattedText>,
}