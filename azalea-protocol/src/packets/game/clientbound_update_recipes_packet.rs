use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use azalea_core::{ResourceLocation, Slot};
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
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
    width: usize,
    height: usize,
    group: String,
    ingredients: Vec<Ingredient>,
    result: Slot,
}

impl McBufWritable for ShapedRecipe {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (self.width as u32).var_write_into(buf)?;
        (self.height as u32).var_write_into(buf)?;
        self.group.write_into(buf)?;
        for ingredient in &self.ingredients {
            ingredient.write_into(buf)?;
        }
        self.result.write_into(buf)?;

        Ok(())
    }
}
impl McBufReadable for ShapedRecipe {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let width = u32::var_read_from(buf)?.try_into().unwrap();
        let height = u32::var_read_from(buf)?.try_into().unwrap();
        let group = String::read_from(buf)?;
        let mut ingredients = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            ingredients.push(Ingredient::read_from(buf)?);
        }
        let result = Slot::read_from(buf)?;

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
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match &self.data {
            RecipeData::CraftingShapeless(recipe) => {
                ResourceLocation::new("minecraft:crafting_shapeless")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
                recipe.write_into(buf)?;
            }
            RecipeData::CraftingShaped(recipe) => {
                ResourceLocation::new("minecraft:crafting_shaped")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
                recipe.write_into(buf)?;
            }
            RecipeData::CraftingSpecialArmorDye => {
                ResourceLocation::new("minecraft:crafting_special_armordye")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
            }
            RecipeData::CraftingSpecialBookCloning => {
                ResourceLocation::new("minecraft:crafting_special_bookcloning")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
            }
            RecipeData::CraftingSpecialMapCloning => {
                ResourceLocation::new("minecraft:crafting_special_mapcloning")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
            }
            RecipeData::CraftingSpecialMapExtending => {
                ResourceLocation::new("minecraft:crafting_special_mapextending")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
            }
            RecipeData::CraftingSpecialFireworkRocket => {
                ResourceLocation::new("minecraft:crafting_special_firework_rocket")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
            }
            RecipeData::CraftingSpecialFireworkStar => {
                ResourceLocation::new("minecraft:crafting_special_firework_star")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
            }
            RecipeData::CraftingSpecialFireworkStarFade => {
                ResourceLocation::new("minecraft:crafting_special_firework_star_fade")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
            }
            RecipeData::CraftingSpecialRepairItem => {
                ResourceLocation::new("minecraft:crafting_special_repairitem")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
            }
            RecipeData::CraftingSpecialTippedArrow => {
                ResourceLocation::new("minecraft:crafting_special_tippedarrow")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
            }
            RecipeData::CraftingSpecialBannerDuplicate => {
                ResourceLocation::new("minecraft:crafting_special_bannerduplicate")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
            }
            RecipeData::CraftingSpecialBannerAddPattern => {
                ResourceLocation::new("minecraft:crafting_special_banneraddpattern")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
            }
            RecipeData::CraftingSpecialShieldDecoration => {
                ResourceLocation::new("minecraft:crafting_special_shielddecoration")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
            }
            RecipeData::CraftingSpecialShulkerBoxColoring => {
                ResourceLocation::new("minecraft:crafting_special_shulkerboxcoloring")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
            }
            RecipeData::CraftingSpecialSuspiciousStew => {
                ResourceLocation::new("minecraft:crafting_special_suspiciousstew")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
            }
            RecipeData::Smelting(recipe) => {
                ResourceLocation::new("minecraft:smelting")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
                recipe.write_into(buf)?;
            }
            RecipeData::Blasting(recipe) => {
                ResourceLocation::new("minecraft:blasting")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
                recipe.write_into(buf)?;
            }
            RecipeData::Smoking(recipe) => {
                ResourceLocation::new("minecraft:smoking")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
                recipe.write_into(buf)?;
            }
            RecipeData::CampfireCooking(recipe) => {
                ResourceLocation::new("minecraft:campfire_cooking")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
                recipe.write_into(buf)?;
            }
            RecipeData::Stonecutting(recipe) => {
                ResourceLocation::new("minecraft:stonecutting")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
                recipe.write_into(buf)?;
            }
            RecipeData::Smithing(recipe) => {
                ResourceLocation::new("minecraft:smithing")
                    .unwrap()
                    .write_into(buf)?;
                self.identifier.write_into(buf)?;
                recipe.write_into(buf)?;
            }
        };
        Ok(())
    }
}

impl McBufReadable for Recipe {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let recipe_type = ResourceLocation::read_from(buf)?;
        let identifier = ResourceLocation::read_from(buf)?;

        // rust doesn't let us match ResourceLocation so we have to do a big
        // if-else chain :(
        let data = if recipe_type == ResourceLocation::new("minecraft:crafting_shapeless").unwrap()
        {
            RecipeData::CraftingShapeless(ShapelessRecipe::read_from(buf)?)
        } else if recipe_type == ResourceLocation::new("minecraft:crafting_shaped").unwrap() {
            RecipeData::CraftingShaped(ShapedRecipe::read_from(buf)?)
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
            RecipeData::Smelting(CookingRecipe::read_from(buf)?)
        } else if recipe_type == ResourceLocation::new("minecraft:blasting").unwrap() {
            RecipeData::Blasting(CookingRecipe::read_from(buf)?)
        } else if recipe_type == ResourceLocation::new("minecraft:smoking").unwrap() {
            RecipeData::Smoking(CookingRecipe::read_from(buf)?)
        } else if recipe_type == ResourceLocation::new("minecraft:campfire_cooking").unwrap() {
            RecipeData::CampfireCooking(CookingRecipe::read_from(buf)?)
        } else if recipe_type == ResourceLocation::new("minecraft:stonecutting").unwrap() {
            RecipeData::Stonecutting(StoneCuttingRecipe::read_from(buf)?)
        } else if recipe_type == ResourceLocation::new("minecraft:smithing").unwrap() {
            RecipeData::Smithing(SmithingRecipe::read_from(buf)?)
        } else {
            return Err(BufReadError::UnexpectedStringEnumVariant {
                id: recipe_type.to_string(),
            });
        };

        let recipe = Recipe { identifier, data };

        Ok(recipe)
    }
}
