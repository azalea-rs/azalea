use std::io::{Read, Write};

use azalea_core::{resource_location::ResourceLocation, Slot};
use packet_macros::{GamePacket, McBuf};

use crate::mc_buf::{McBufReadable, McBufWritable, Readable, Writable};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundUpdateRecipesPacket {
    pub recipes: Vec<Recipe>,
}

#[derive(Clone, Debug)]
pub struct Recipe {
    pub identifier: ResourceLocation,
    pub data: RecipeData,
}

#[derive(Clone, Debug, McBuf)]
pub struct ShapelessRecipe {
    /// Used to group similar recipes together in the recipe book.
    /// Tag is present in recipe JSON
    group: String,
    ingredients: Vec<Ingredient>,
    result: Slot,
}
#[derive(Clone, Debug)]
pub struct ShapedRecipe {
    // TODO: make own McBufReadable and McBufWritable for this
    width: usize,
    height: usize,
    group: String,
    ingredients: Vec<Ingredient>,
    result: Slot,
}

impl McBufWritable for ShapedRecipe {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_varint(self.width.try_into().unwrap())?;
        buf.write_varint(self.height.try_into().unwrap())?;
        buf.write_utf(&self.group)?;
        for ingredient in &self.ingredients {
            ingredient.write_into(buf)?;
        }
        self.result.write_into(buf)?;

        Ok(())
    }
}
impl McBufReadable for ShapedRecipe {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let width = buf.read_varint()?.try_into().unwrap();
        let height = buf.read_varint()?.try_into().unwrap();
        let group = buf.read_utf()?;
        let mut ingredients = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            ingredients.push(Ingredient::read_into(buf)?);
        }
        let result = Slot::read_into(buf)?;

        Ok(ShapedRecipe {
            width,
            height,
            group,
            ingredients,
            result,
        })
    }
}

#[derive(Clone, Debug, McBuf)]
pub struct CookingRecipe {
    group: String,
    ingredient: Ingredient,
    result: Slot,
    experience: f32,
    #[var]
    cooking_time: u32,
}
#[derive(Clone, Debug, McBuf)]
pub struct StoneCuttingRecipe {
    group: String,
    ingredient: Ingredient,
    result: Slot,
}
#[derive(Clone, Debug, McBuf)]
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
    Smithing(SmithingRecipe),
}

#[derive(Clone, Debug, McBuf)]
pub struct Ingredient {
    pub allowed: Vec<Slot>,
}

impl McBufWritable for Recipe {
    fn write_into(&self, _buf: &mut impl Write) -> Result<(), std::io::Error> {
        todo!()
    }
}

impl McBufReadable for Recipe {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let recipe_type = buf.read_resource_location()?;
        let identifier = buf.read_resource_location()?;

        // rust doesn't let us match ResourceLocation so we have to do a big
        // if-else chain :(
        let data = if recipe_type == ResourceLocation::new("minecraft:crafting_shapeless").unwrap()
        {
            RecipeData::CraftingShapeless(ShapelessRecipe::read_into(buf)?)
        } else if recipe_type == ResourceLocation::new("minecraft:crafting_shaped").unwrap() {
            RecipeData::CraftingShaped(ShapedRecipe::read_into(buf)?)
        } else if recipe_type
            == ResourceLocation::new("minecraft:crafting_special_armordye").unwrap()
        {
            RecipeData::CraftingSpecialArmorDye
        } else if recipe_type
            == ResourceLocation::new("minecraft:crafting_special_bookcloning").unwrap()
        {
            RecipeData::CraftingSpecialBookCloning
        } else if recipe_type
            == ResourceLocation::new("minecraft:crafting_special_mapcloning").unwrap()
        {
            RecipeData::CraftingSpecialMapCloning
        } else if recipe_type
            == ResourceLocation::new("minecraft:crafting_special_mapextending").unwrap()
        {
            RecipeData::CraftingSpecialMapExtending
        } else if recipe_type
            == ResourceLocation::new("minecraft:crafting_special_firework_rocket").unwrap()
        {
            RecipeData::CraftingSpecialFireworkRocket
        } else if recipe_type
            == ResourceLocation::new("minecraft:crafting_special_firework_star").unwrap()
        {
            RecipeData::CraftingSpecialFireworkStar
        } else if recipe_type
            == ResourceLocation::new("minecraft:crafting_special_firework_star_fade").unwrap()
        {
            RecipeData::CraftingSpecialFireworkStarFade
        } else if recipe_type
            == ResourceLocation::new("minecraft:crafting_special_repairitem").unwrap()
        {
            RecipeData::CraftingSpecialRepairItem
        } else if recipe_type
            == ResourceLocation::new("minecraft:crafting_special_tippedarrow").unwrap()
        {
            RecipeData::CraftingSpecialTippedArrow
        } else if recipe_type
            == ResourceLocation::new("minecraft:crafting_special_bannerduplicate").unwrap()
        {
            RecipeData::CraftingSpecialBannerDuplicate
        } else if recipe_type
            == ResourceLocation::new("minecraft:crafting_special_banneraddpattern").unwrap()
        {
            RecipeData::CraftingSpecialBannerAddPattern
        } else if recipe_type
            == ResourceLocation::new("minecraft:crafting_special_shielddecoration").unwrap()
        {
            RecipeData::CraftingSpecialShieldDecoration
        } else if recipe_type
            == ResourceLocation::new("minecraft:crafting_special_shulkerboxcoloring").unwrap()
        {
            RecipeData::CraftingSpecialShulkerBoxColoring
        } else if recipe_type
            == ResourceLocation::new("minecraft:crafting_special_suspiciousstew").unwrap()
        {
            RecipeData::CraftingSpecialSuspiciousStew
        } else if recipe_type == ResourceLocation::new("minecraft:smelting").unwrap() {
            RecipeData::Smelting(CookingRecipe::read_into(buf)?)
        } else if recipe_type == ResourceLocation::new("minecraft:blasting").unwrap() {
            RecipeData::Blasting(CookingRecipe::read_into(buf)?)
        } else if recipe_type == ResourceLocation::new("minecraft:smoking").unwrap() {
            RecipeData::Smoking(CookingRecipe::read_into(buf)?)
        } else if recipe_type == ResourceLocation::new("minecraft:campfire_cooking").unwrap() {
            RecipeData::CampfireCooking(CookingRecipe::read_into(buf)?)
        } else if recipe_type == ResourceLocation::new("minecraft:stonecutting").unwrap() {
            RecipeData::Stonecutting(StoneCuttingRecipe::read_into(buf)?)
        } else if recipe_type == ResourceLocation::new("minecraft:smithing").unwrap() {
            RecipeData::Smithing(SmithingRecipe::read_into(buf)?)
        } else {
            panic!("Unknown recipe type sent by server: {}", recipe_type);
        };

        let recipe = Recipe { identifier, data };

        Ok(recipe)
    }
}
