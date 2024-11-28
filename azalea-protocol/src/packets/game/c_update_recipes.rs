use std::collections::HashMap;

use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_inventory::ItemStack;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::HolderSet;

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

/// [`azalea_registry::SlotDisplay`]
#[derive(Clone, Debug, PartialEq, AzBuf)]
pub enum SlotDisplayData {
    Empty,
    AnyFuel,
    Item(ItemStackDisplay),
    ItemStack(ItemStackSlotDisplay),
    Tag(ResourceLocation),
    SmithingTrim(Box<SmithingTrimDemoSlotDisplay>),
    WithRemainder(Box<WithRemainderSlotDisplay>),
    Composite(CompositeSlotDisplay),
}

#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct ItemStackDisplay {
    pub item: azalea_registry::Item,
}
#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct ItemStackSlotDisplay {
    pub stack: ItemStack,
}
#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct TagSlotDisplay {
    pub tag: azalea_registry::Item,
}
#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct SmithingTrimDemoSlotDisplay {
    pub base: SlotDisplayData,
    pub material: SlotDisplayData,
    pub pattern: SlotDisplayData,
}
#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct WithRemainderSlotDisplay {
    pub input: SlotDisplayData,
    pub remainder: SlotDisplayData,
}
#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct CompositeSlotDisplay {
    pub contents: Vec<SlotDisplayData>,
}

#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct RecipePropertySet {
    pub items: Vec<azalea_registry::Item>,
}

#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct Ingredient {
    pub allowed: HolderSet<azalea_registry::Item, ResourceLocation>,
}
