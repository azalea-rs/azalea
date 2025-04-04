use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::recipe::RecipeDisplayData;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundPlaceGhostRecipe {
    #[var]
    pub container_id: i32,
    pub recipe: RecipeDisplayData,
}
