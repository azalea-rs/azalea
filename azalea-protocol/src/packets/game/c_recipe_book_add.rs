use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::recipe::{Ingredient, RecipeDisplayData};

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundRecipeBookAdd {
    pub entries: Vec<Entry>,
    pub replace: bool,
}

#[derive(Clone, Debug, AzBuf)]
pub struct Entry {
    pub contents: RecipeDisplayEntry,
    pub flags: u8,
}

#[derive(Clone, Debug, AzBuf)]
pub struct RecipeDisplayEntry {
    #[var]
    pub id: u32,
    pub display: RecipeDisplayData,
    // optional varint
    #[var]
    pub group: u32,
    pub category: azalea_registry::RecipeBookCategory,
    pub crafting_requirements: Option<Vec<Ingredient>>,
}
