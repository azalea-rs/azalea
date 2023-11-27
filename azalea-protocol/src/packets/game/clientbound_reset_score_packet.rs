use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundResetScorePacket {
    pub owner: String,
    pub objective_name: Option<String>,
}
