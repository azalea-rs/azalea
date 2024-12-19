use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_inventory::ItemStack;
use azalea_registry::HolderSet;

/// [`azalea_registry::RecipeDisplay`]
#[derive(Clone, Debug, AzBuf)]
pub enum RecipeDisplayData {
    Shapeless(ShapelessCraftingRecipeDisplay),
    Shaped(ShapedCraftingRecipeDisplay),
    Furnace(FurnaceRecipeDisplay),
    Stonecutter(StonecutterRecipeDisplay),
    Smithing(SmithingRecipeDisplay),
}

#[derive(Clone, Debug, AzBuf)]
pub struct ShapelessCraftingRecipeDisplay {
    pub ingredients: Vec<SlotDisplayData>,
    pub result: SlotDisplayData,
    pub crafting_station: SlotDisplayData,
}
#[derive(Clone, Debug, AzBuf)]
pub struct ShapedCraftingRecipeDisplay {
    #[var]
    pub width: u32,
    #[var]
    pub height: u32,
    pub ingredients: Vec<SlotDisplayData>,
    pub result: SlotDisplayData,
    pub crafting_station: SlotDisplayData,
}
#[derive(Clone, Debug, AzBuf)]
pub struct FurnaceRecipeDisplay {
    pub ingredient: SlotDisplayData,
    pub fuel: SlotDisplayData,
    pub result: SlotDisplayData,
    pub crafting_station: SlotDisplayData,
    #[var]
    pub duration: u32,
    pub experience: f32,
}
#[derive(Clone, Debug, AzBuf)]
pub struct StonecutterRecipeDisplay {
    pub input: SlotDisplayData,
    pub result: SlotDisplayData,
    pub crafting_station: SlotDisplayData,
}
#[derive(Clone, Debug, AzBuf)]
pub struct SmithingRecipeDisplay {
    pub template: SlotDisplayData,
    pub base: SlotDisplayData,
    pub addition: SlotDisplayData,
    pub result: SlotDisplayData,
    pub crafting_station: SlotDisplayData,
}

#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct Ingredient {
    pub allowed: HolderSet<azalea_registry::Item, ResourceLocation>,
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
