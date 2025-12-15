use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::recipe::RecipeDisplayData;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundPlaceGhostRecipe {
    #[var]
    pub container_id: i32,
    pub recipe: RecipeDisplayData,
}
