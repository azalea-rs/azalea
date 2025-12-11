use std::collections::HashMap;

use azalea_buf::AzBuf;
use azalea_registry::identifier::Identifier;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::builtin::ItemKind;

use crate::common::recipe::{Ingredient, SlotDisplayData};

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundUpdateRecipes {
    pub item_sets: HashMap<Identifier, RecipePropertySet>,
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
    pub items: Vec<ItemKind>,
}
