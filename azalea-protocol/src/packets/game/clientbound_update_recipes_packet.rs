use std::collections::HashMap;

use azalea_buf::McBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_inventory::ItemSlot;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::HolderSet;

#[derive(Clone, Debug, PartialEq, McBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateRecipesPacket {
    pub item_sets: HashMap<ResourceLocation, RecipePropertySet>,
    pub stonecutter_recipes: Vec<SingleInputEntry>,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct SingleInputEntry {
    pub input: Ingredient,
    pub recipe: SelectableRecipe,
}
#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct SelectableRecipe {
    pub option_display: SlotDisplayData,
}

/// [`azalea_registry::SlotDisplay`]
#[derive(Clone, Debug, PartialEq, McBuf)]
pub enum SlotDisplayData {
    Empty,
    AnyFuel,
    Item(ItemSlotDisplay),
    ItemStack(ItemStackSlotDisplay),
    Tag(ResourceLocation),
    SmithingTrim(Box<SmithingTrimDemoSlotDisplay>),
    WithRemainder(Box<WithRemainderSlotDisplay>),
    Composite(CompositeSlotDisplay),
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct ItemSlotDisplay {
    pub item: azalea_registry::Item,
}
#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct ItemStackSlotDisplay {
    pub stack: ItemSlot,
}
#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct TagSlotDisplay {
    pub tag: azalea_registry::Item,
}
#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct SmithingTrimDemoSlotDisplay {
    pub base: SlotDisplayData,
    pub material: SlotDisplayData,
    pub pattern: SlotDisplayData,
}
#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct WithRemainderSlotDisplay {
    pub input: SlotDisplayData,
    pub remainder: SlotDisplayData,
}
#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct CompositeSlotDisplay {
    pub contents: Vec<SlotDisplayData>,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct RecipePropertySet {
    pub items: Vec<azalea_registry::Item>,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct Ingredient {
    pub allowed: HolderSet<azalea_registry::Item, ResourceLocation>,
}
