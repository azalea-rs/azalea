use azalea_buf::AzBuf;
use azalea_chat::{numbers::NumberFormat, FormattedText};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundSetScore {
    pub owner: String,
    pub objective_name: String,
    #[var]
    pub score: u32,
    pub display: Option<FormattedText>,
    pub number_format: Option<NumberFormat>,
}
