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
    pub group: String,
    pub category: CraftingBookCategory,
    pub ingredients: Vec<Ingredient>,
    pub result: Slot,
}
#[derive(Clone, Debug)]
pub struct ShapedRecipe {
    pub width: usize,
    pub height: usize,
    pub group: String,
    pub category: CraftingBookCategory,
    pub ingredients: Vec<Ingredient>,
    pub result: Slot,
}

#[derive(Clone, Debug, Copy, McBuf)]
pub enum CraftingBookCategory {
    Building = 0,
    Redstone,
    Equipment,
    Misc,
}

impl McBufWritable for ShapedRecipe {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (self.width as u32).var_write_into(buf)?;
        (self.height as u32).var_write_into(buf)?;
        self.group.write_into(buf)?;
        self.category.write_into(buf)?;
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
        let category = CraftingBookCategory::read_from(buf)?;
        let mut ingredients = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            ingredients.push(Ingredient::read_from(buf)?);
        }
        let result = Slot::read_from(buf)?;

        Ok(ShapedRecipe {
            width,
            height,
            group,
            category,
            ingredients,
            result,
        })
    }
}

#[derive(Clone, Debug, McBuf)]
pub struct CookingRecipe {
    pub group: String,
    pub category: CraftingBookCategory,
    pub ingredient: Ingredient,
    pub result: Slot,
    pub experience: f32,
    #[var]
    pub cooking_time: u32,
}
#[derive(Clone, Debug, McBuf)]
pub struct StoneCutterRecipe {
    pub group: String,
    pub ingredient: Ingredient,
    pub result: Slot,
}
#[derive(Clone, Debug, McBuf)]
pub struct SmithingRecipe {
    pub base: Ingredient,
    pub addition: Ingredient,
    pub result: Slot,
}

#[derive(Clone, Debug, McBuf)]
pub struct SimpleRecipe {
    pub category: CraftingBookCategory,
}

#[derive(Clone, Debug, McBuf)]
pub enum RecipeData {
    CraftingShapeless(ShapelessRecipe),
    CraftingShaped(ShapedRecipe),
    CraftingSpecialArmorDye(SimpleRecipe),
    CraftingSpecialBookCloning(SimpleRecipe),
    CraftingSpecialMapCloning(SimpleRecipe),
    CraftingSpecialMapExtending(SimpleRecipe),
    CraftingSpecialFireworkRocket(SimpleRecipe),
    CraftingSpecialFireworkStar(SimpleRecipe),
    CraftingSpecialFireworkStarFade(SimpleRecipe),
    CraftingSpecialRepairItem(SimpleRecipe),
    CraftingSpecialTippedArrow(SimpleRecipe),
    CraftingSpecialBannerDuplicate(SimpleRecipe),
    CraftingSpecialBannerAddPattern(SimpleRecipe),
    CraftingSpecialShieldDecoration(SimpleRecipe),
    CraftingSpecialShulkerBoxColoring(SimpleRecipe),
    CraftingSpecialSuspiciousStew(SimpleRecipe),
    Smelting(CookingRecipe),
    Blasting(CookingRecipe),
    Smoking(CookingRecipe),
    CampfireCooking(CookingRecipe),
    Stonecutting(StoneCutterRecipe),
    Smithing(SmithingRecipe),
}

#[derive(Clone, Debug, McBuf)]
pub struct Ingredient {
    pub allowed: Vec<Slot>,
}

impl McBufWritable for Recipe {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let resource_location = match &self.data {
            RecipeData::CraftingShapeless(_) => "minecraft:crafting_shapeless",
            RecipeData::CraftingShaped(_) => "minecraft:crafting_shaped",
            RecipeData::CraftingSpecialArmorDye(_) => "minecraft:crafting_special_armordye",
            RecipeData::CraftingSpecialBookCloning(_) => "minecraft:crafting_special_bookcloning",
            RecipeData::CraftingSpecialMapCloning(_) => "minecraft:crafting_special_mapcloning",
            RecipeData::CraftingSpecialMapExtending(_) => "minecraft:crafting_special_mapextending",
            RecipeData::CraftingSpecialFireworkRocket(_) => {
                "minecraft:crafting_special_firework_rocket"
            }
            RecipeData::CraftingSpecialFireworkStar(_) => {
                "minecraft:crafting_special_firework_star"
            }

            RecipeData::CraftingSpecialFireworkStarFade(_) => {
                "minecraft:crafting_special_firework_star_fade"
            }
            RecipeData::CraftingSpecialRepairItem(_) => "minecraft:crafting_special_repairitem",
            RecipeData::CraftingSpecialTippedArrow(_) => "minecraft:crafting_special_tippedarrow",
            RecipeData::CraftingSpecialBannerDuplicate(_) => {
                "minecraft:crafting_special_bannerduplicate"
            }
            RecipeData::CraftingSpecialBannerAddPattern(_) => {
                "minecraft:crafting_special_banneraddpattern"
            }
            RecipeData::CraftingSpecialShieldDecoration(_) => {
                "minecraft:crafting_special_shielddecoration"
            }
            RecipeData::CraftingSpecialShulkerBoxColoring(_) => {
                "minecraft:crafting_special_shulkerboxcoloring"
            }
            RecipeData::CraftingSpecialSuspiciousStew(_) => {
                "minecraft:crafting_special_suspiciousstew"
            }
            RecipeData::Smelting(_) => "minecraft:smelting",
            RecipeData::Blasting(_) => "minecraft:blasting",
            RecipeData::Smoking(_) => "minecraft:smoking",
            RecipeData::CampfireCooking(_) => "minecraft:campfire_cooking",
            RecipeData::Stonecutting(_) => "minecraft:stonecutting",
            RecipeData::Smithing(_) => "minecraft:smithing",
        };
        ResourceLocation::new(resource_location).write_into(buf)?;
        self.identifier.write_into(buf)?;
        self.data.write_without_id(buf)?;
        Ok(())
    }
}

impl McBufReadable for Recipe {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let recipe_type = ResourceLocation::read_from(buf)?;
        let identifier = ResourceLocation::read_from(buf)?;

        // rust doesn't let us match ResourceLocation so we have to do a big
        // if-else chain :(
        let data = match recipe_type.to_string().as_str() {
            "minecraft:crafting_shapeless" => {
                RecipeData::CraftingShapeless(ShapelessRecipe::read_from(buf)?)
            }
            "minecraft:crafting_shaped" => {
                RecipeData::CraftingShaped(ShapedRecipe::read_from(buf)?)
            }
            "minecraft:crafting_special_armordye" => {
                RecipeData::CraftingSpecialArmorDye(SimpleRecipe::read_from(buf)?)
            }
            "minecraft:crafting_special_bookcloning" => {
                RecipeData::CraftingSpecialBookCloning(SimpleRecipe::read_from(buf)?)
            }
            "minecraft:crafting_special_mapcloning" => {
                RecipeData::CraftingSpecialMapCloning(SimpleRecipe::read_from(buf)?)
            }
            "minecraft:crafting_special_mapextending" => {
                RecipeData::CraftingSpecialMapExtending(SimpleRecipe::read_from(buf)?)
            }
            "minecraft:crafting_special_firework_rocket" => {
                RecipeData::CraftingSpecialFireworkRocket(SimpleRecipe::read_from(buf)?)
            }
            "minecraft:crafting_special_firework_star" => {
                RecipeData::CraftingSpecialFireworkStar(SimpleRecipe::read_from(buf)?)
            }
            "minecraft:crafting_special_firework_star_fade" => {
                RecipeData::CraftingSpecialFireworkStarFade(SimpleRecipe::read_from(buf)?)
            }
            "minecraft:crafting_special_repairitem" => {
                RecipeData::CraftingSpecialRepairItem(SimpleRecipe::read_from(buf)?)
            }
            "minecraft:crafting_special_tippedarrow" => {
                RecipeData::CraftingSpecialTippedArrow(SimpleRecipe::read_from(buf)?)
            }
            "minecraft:crafting_special_bannerduplicate" => {
                RecipeData::CraftingSpecialBannerDuplicate(SimpleRecipe::read_from(buf)?)
            }
            "minecraft:crafting_special_banneraddpattern" => {
                RecipeData::CraftingSpecialBannerAddPattern(SimpleRecipe::read_from(buf)?)
            }
            "minecraft:crafting_special_shielddecoration" => {
                RecipeData::CraftingSpecialShieldDecoration(SimpleRecipe::read_from(buf)?)
            }
            "minecraft:crafting_special_shulkerboxcoloring" => {
                RecipeData::CraftingSpecialShulkerBoxColoring(SimpleRecipe::read_from(buf)?)
            }
            "minecraft:crafting_special_suspiciousstew" => {
                RecipeData::CraftingSpecialSuspiciousStew(SimpleRecipe::read_from(buf)?)
            }
            "minecraft:smelting" => RecipeData::Smelting(CookingRecipe::read_from(buf)?),
            "minecraft:blasting" => RecipeData::Blasting(CookingRecipe::read_from(buf)?),
            "minecraft:smoking" => RecipeData::Smoking(CookingRecipe::read_from(buf)?),
            "minecraft:campfire_cooking" => {
                RecipeData::CampfireCooking(CookingRecipe::read_from(buf)?)
            }
            "minecraft:stonecutting" => {
                RecipeData::Stonecutting(StoneCutterRecipe::read_from(buf)?)
            }
            "minecraft:smithing" => RecipeData::Smithing(SmithingRecipe::read_from(buf)?),
            _ => {
                return Err(BufReadError::UnexpectedStringEnumVariant {
                    id: recipe_type.to_string(),
                });
            }
        };

        let recipe = Recipe { identifier, data };

        Ok(recipe)
    }
}
