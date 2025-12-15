use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::builtin::RecipeBookCategory;

use crate::common::recipe::{Ingredient, RecipeDisplayData};

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundRecipeBookAdd {
    pub entries: Vec<Entry>,
    pub replace: bool,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct Entry {
    pub contents: RecipeDisplayEntry,
    pub flags: u8,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct RecipeDisplayEntry {
    #[var]
    pub id: u32,
    pub display: RecipeDisplayData,
    // optional varint
    #[var]
    pub group: u32,
    pub category: RecipeBookCategory,
    pub crafting_requirements: Option<Vec<Ingredient>>,
}
