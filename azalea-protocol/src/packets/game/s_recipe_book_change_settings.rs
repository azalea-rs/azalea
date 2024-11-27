use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundRecipeBookChangeSettings {
    pub book_type: RecipeBookType,
    pub is_open: bool,
    pub is_filtering: bool,
}

#[derive(AzBuf, Clone, Copy, Debug)]
pub enum RecipeBookType {
    Crafting = 0,
    Furnace = 1,
    BlastFurnace = 2,
    Smoker = 3,
}
