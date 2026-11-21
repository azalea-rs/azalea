use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::number_formatting::NumberFormat;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundSetScore {
    pub owner: String,
    pub objective_name: String,
    #[var]
    pub score: u32,
    pub display: Option<FormattedText>,
    pub number_format: Option<NumberFormat>,
}
