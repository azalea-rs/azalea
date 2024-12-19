use std::collections::HashMap;

use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::recipe::{Ingredient, SlotDisplayData};

#[derive(Clone, Debug, PartialEq, AzBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateRecipes {
    pub item_sets: HashMap<ResourceLocation, RecipePropertySet>,
    pub stonecutter_recipes: Vec<SingleInputEntry>,
}

#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct SingleInputEntry {
    pub input: Ingredient,
    pub recipe: SelectableRecipe,
}
#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct SelectableRecipe {
    pub option_display: SlotDisplayData,
}

#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct RecipePropertySet {
    pub items: Vec<azalea_registry::Item>,
}
