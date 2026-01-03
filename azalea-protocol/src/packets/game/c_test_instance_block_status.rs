use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_core::position::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundTestInstanceBlockStatus {
    pub status: FormattedText,
    pub size: Option<Vec3>,
}
