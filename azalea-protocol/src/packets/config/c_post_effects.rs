use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;
use azalea_registry::identifier::Identifier;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundConfigPacket)]
pub struct ClientboundPostEffects {
    pub post_effects: Vec<Identifier>,
}
