use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundRecipeBookRemove {
    #[var]
    pub recipes: Vec<u32>,
}
