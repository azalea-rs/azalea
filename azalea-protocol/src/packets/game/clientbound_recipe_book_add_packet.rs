use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use super::clientbound_update_recipes_packet::{Ingredient, SlotDisplayData};

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundRecipeBookAddPacket {
    pub entries: Vec<Entry>,
    pub replace: bool,
}

#[derive(Clone, Debug, McBuf)]
pub struct Entry {
    pub contents: RecipeDisplayEntry,
    pub flags: u8,
}

#[derive(Clone, Debug, McBuf)]
pub struct RecipeDisplayEntry {
    #[var]
    pub id: u32,
    pub display: RecipeDisplayData,
    // ByteBufCodecs.OPTIONAL_VAR_INT
    #[var]
    pub group: u32,
    pub category: azalea_registry::RecipeBookCategory,
    pub crafting_requirements: Option<Vec<Ingredient>>,
}

/// [`azalea_registry::RecipeDisplay`]
#[derive(Clone, Debug, McBuf)]
pub enum RecipeDisplayData {
    Shapeless(ShapelessCraftingRecipeDisplay),
    Shaped(ShapedCraftingRecipeDisplay),
    Furnace(FurnaceRecipeDisplay),
    Stonecutter(StonecutterRecipeDisplay),
    Smithing(SmithingRecipeDisplay),
}

#[derive(Clone, Debug, McBuf)]
pub struct ShapelessCraftingRecipeDisplay {
    pub ingredients: Vec<SlotDisplayData>,
    pub result: SlotDisplayData,
    pub crafting_station: SlotDisplayData,
}
#[derive(Clone, Debug, McBuf)]
pub struct ShapedCraftingRecipeDisplay {
    #[var]
    pub width: u32,
    #[var]
    pub height: u32,
    pub ingredients: Vec<SlotDisplayData>,
    pub result: SlotDisplayData,
    pub crafting_station: SlotDisplayData,
}
#[derive(Clone, Debug, McBuf)]
pub struct FurnaceRecipeDisplay {
    pub ingredient: SlotDisplayData,
    pub fuel: SlotDisplayData,
    pub result: SlotDisplayData,
    pub crafting_station: SlotDisplayData,
    #[var]
    pub duration: u32,
    pub experience: f32,
}
#[derive(Clone, Debug, McBuf)]
pub struct StonecutterRecipeDisplay {
    pub input: SlotDisplayData,
    pub result: SlotDisplayData,
    pub crafting_station: SlotDisplayData,
}
#[derive(Clone, Debug, McBuf)]
pub struct SmithingRecipeDisplay {
    pub template: SlotDisplayData,
    pub base: SlotDisplayData,
    pub addition: SlotDisplayData,
    pub result: SlotDisplayData,
    pub crafting_station: SlotDisplayData,
}
