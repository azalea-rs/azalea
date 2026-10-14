use azalea_buf::AzBuf;
use crate::common::recipe::RecipeDisplayId;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundPlaceRecipe {
    #[var]
    pub container_id: i32,
    pub recipe: RecipeDisplayId,
    pub shift_down: bool,
}
