use azalea_buf::AzBuf;
use azalea_core::identifier::Identifier;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundPostEffects {
    pub post_effects: Vec<Identifier>,
}
