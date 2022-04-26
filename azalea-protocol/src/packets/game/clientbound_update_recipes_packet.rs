use async_trait::async_trait;
use azalea_chat::component::Component;
use azalea_core::{resource_location::ResourceLocation, Slot};
use packet_macros::{GamePacket, McBufReadable, McBufWritable};
use tokio::io::AsyncRead;

use crate::mc_buf::{McBufReadable, McBufWritable, Readable, Writable};

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundUpdateRecipesPacket {
    pub recipes: Vec<Recipe>,
}

#[derive(Clone, Debug)]
pub struct Recipe {
    pub identifier: ResourceLocation,
    pub data: RecipeData,
}

#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct ShapelessRecipe {
    /// Used to group similar recipes together in the recipe book.
    /// Tag is present in recipe JSON
    group: String,
    ingredients: Vec<Ingredient>,
    result: Slot,
}
#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct ShapedRecipe {
    width: u32,
    height: u32,
    group: String,
    ingredients: Vec<Ingredient>,
    result: Slot,
}
#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct CookingRecipe {
    group: String,
    ingredient: Ingredient,
    result: Slot,
    experience: f32,
    #[varint]
    cooking_time: u32,
}
#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct StoneCuttingRecipe {
    group: String,
    ingredient: Ingredient,
    result: Slot,
}
#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct SmithingRecipe {
    base: Ingredient,
    addition: Ingredient,
    result: Slot,
}

#[derive(Clone, Debug)]
pub enum RecipeData {
    CraftingShapeless(ShapelessRecipe),
    CraftingShaped(ShapedRecipe),
    CraftingSpecialArmorDye,
    CraftingSpecialBookCloning,
    CraftingSpecialMapCloning,
    CraftingSpecialMapExtending,
    CraftingSpecialFireworkRocket,
    CraftingSpecialFireworkStar,
    CraftingSpecialFireworkStarFade,
    CraftingSpecialRepairItem,
    CraftingSpecialTippedArrow,
    CraftingSpecialBannerDuplicate,
    CraftingSpecialBannerAddPattern,
    CraftingSpecialShieldDecoration,
    CraftingSpecialShulkerBoxColoring,
    CraftingSpecialSuspiciousStew,
    Smelting(CookingRecipe),
    Blasting(CookingRecipe),
    Smoking(CookingRecipe),
    CampfireCooking(CookingRecipe),
    Stonecutting(StoneCuttingRecipe),
}

#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct Ingredient {
    pub allowed: Vec<Slot>,
}

impl McBufWritable for Recipe {
    fn write_into(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        todo!()
    }
}
#[async_trait]
impl McBufReadable for Recipe {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        let recipe_type = buf.read_resource_location().await?;
        let identifier = buf.read_resource_location().await?;

        // rust doesn't let us match ResourceLocation so we have to do a big
        // if-else chain :(
        let data = if recipe_type == ResourceLocation::new("minecraft:crafting_shapeless").unwrap()
        {
            let group = buf.read_utf().await?;
            let ingredients = Vec::<Ingredient>::read_into(buf).await?;
            let result = Slot::read_into(buf).await?;

            RecipeData::CraftingShapeless(ShapelessRecipe {
                group,
                ingredients,
                result,
            })
        } else {
            panic!("Unknown recipe type sent by server: {}", recipe_type);
        };

        let recipe = Recipe { identifier, data };

        Ok(recipe)
    }
}
