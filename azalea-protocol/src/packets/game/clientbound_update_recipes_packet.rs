use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use azalea_core::ResourceLocation;
use azalea_inventory::ItemSlot;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::RecipeSerializer;

use std::io::{Cursor, Write};
use std::str::FromStr;

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
    /// Nbt is present in recipe JSON
    pub group: String,
    pub category: CraftingBookCategory,
    pub ingredients: Vec<Ingredient>,
    pub result: ItemSlot,
}
#[derive(Clone, Debug)]
pub struct ShapedRecipe {
    pub width: usize,
    pub height: usize,
    pub group: String,
    pub category: CraftingBookCategory,
    pub ingredients: Vec<Ingredient>,
    pub result: ItemSlot,
    pub show_notification: bool,
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
        let result = ItemSlot::read_from(buf)?;
        let show_notification = bool::read_from(buf)?;

        Ok(ShapedRecipe {
            width,
            height,
            group,
            category,
            ingredients,
            result,
            show_notification,
        })
    }
}

#[derive(Clone, Debug, McBuf)]
pub struct CookingRecipe {
    pub group: String,
    pub category: CraftingBookCategory,
    pub ingredient: Ingredient,
    pub result: ItemSlot,
    pub experience: f32,
    #[var]
    pub cooking_time: u32,
}
#[derive(Clone, Debug, McBuf)]
pub struct StoneCutterRecipe {
    pub group: String,
    pub ingredient: Ingredient,
    pub result: ItemSlot,
}
#[derive(Clone, Debug, McBuf)]
pub struct SmithingRecipe {
    pub base: Ingredient,
    pub addition: Ingredient,
    pub result: ItemSlot,
}

#[derive(Clone, Debug, McBuf)]
pub struct SimpleRecipe {
    pub category: CraftingBookCategory,
}

#[derive(Clone, Debug, McBuf)]
pub struct SmithingTransformRecipe {
    pub template: Ingredient,
    pub base: Ingredient,
    pub addition: Ingredient,
    pub result: ItemSlot,
}

#[derive(Clone, Debug, McBuf)]
pub struct SmithingTrimRecipe {
    pub template: Ingredient,
    pub base: Ingredient,
    pub addition: Ingredient,
}

#[derive(Clone, Debug, McBuf)]
pub enum RecipeData {
    CraftingShaped(ShapedRecipe),
    CraftingShapeless(ShapelessRecipe),
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
    CraftingSpecialShieldDecoration(SimpleRecipe),
    CraftingSpecialShulkerBoxColoring(SimpleRecipe),
    CraftingSpecialSuspiciousStew(SimpleRecipe),
    Smelting(CookingRecipe),
    Blasting(CookingRecipe),
    Smoking(CookingRecipe),
    CampfireCooking(CookingRecipe),
    Stonecutting(StoneCutterRecipe),
    SmithingTransform(SmithingTransformRecipe),
    SmithingTrim(SmithingTrimRecipe),
    CraftingDecoratedPot(SimpleRecipe),
}

#[derive(Clone, Debug, McBuf)]
pub struct Ingredient {
    pub allowed: Vec<ItemSlot>,
}

impl McBufWritable for Recipe {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let recipe_serializer = match &self.data {
            RecipeData::CraftingShapeless(_) => RecipeSerializer::CraftingShapeless,
            RecipeData::CraftingShaped(_) => RecipeSerializer::CraftingShaped,
            RecipeData::CraftingSpecialArmorDye(_) => RecipeSerializer::CraftingSpecialArmordye,
            RecipeData::CraftingSpecialBookCloning(_) => {
                RecipeSerializer::CraftingSpecialBookcloning
            }
            RecipeData::CraftingSpecialMapCloning(_) => RecipeSerializer::CraftingSpecialMapcloning,
            RecipeData::CraftingSpecialMapExtending(_) => {
                RecipeSerializer::CraftingSpecialMapextending
            }
            RecipeData::CraftingSpecialFireworkRocket(_) => {
                RecipeSerializer::CraftingSpecialFireworkRocket
            }
            RecipeData::CraftingSpecialFireworkStar(_) => {
                RecipeSerializer::CraftingSpecialFireworkStar
            }

            RecipeData::CraftingSpecialFireworkStarFade(_) => {
                RecipeSerializer::CraftingSpecialFireworkStarFade
            }
            RecipeData::CraftingSpecialRepairItem(_) => RecipeSerializer::CraftingSpecialRepairitem,
            RecipeData::CraftingSpecialTippedArrow(_) => {
                RecipeSerializer::CraftingSpecialTippedarrow
            }
            RecipeData::CraftingSpecialBannerDuplicate(_) => {
                RecipeSerializer::CraftingSpecialBannerduplicate
            }
            RecipeData::CraftingSpecialShieldDecoration(_) => {
                RecipeSerializer::CraftingSpecialShielddecoration
            }
            RecipeData::CraftingSpecialShulkerBoxColoring(_) => {
                RecipeSerializer::CraftingSpecialShulkerboxcoloring
            }
            RecipeData::CraftingSpecialSuspiciousStew(_) => {
                RecipeSerializer::CraftingSpecialSuspiciousstew
            }
            RecipeData::Smelting(_) => RecipeSerializer::Smelting,
            RecipeData::Blasting(_) => RecipeSerializer::Blasting,
            RecipeData::Smoking(_) => RecipeSerializer::Smoking,
            RecipeData::CampfireCooking(_) => RecipeSerializer::CampfireCooking,
            RecipeData::Stonecutting(_) => RecipeSerializer::Stonecutting,
            RecipeData::SmithingTransform(_) => RecipeSerializer::SmithingTransform,
            RecipeData::SmithingTrim(_) => RecipeSerializer::SmithingTrim,
            RecipeData::CraftingDecoratedPot(_) => RecipeSerializer::CraftingDecoratedPot,
        };
        let resource_location = ResourceLocation::new(&recipe_serializer.to_string());
        resource_location.write_into(buf)?;
        self.identifier.write_into(buf)?;
        self.data.write_without_id(buf)?;
        Ok(())
    }
}

impl McBufReadable for Recipe {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let recipe_serializer_name = ResourceLocation::read_from(buf)?;
        let Ok(recipe_serializer) = RecipeSerializer::from_str(&recipe_serializer_name.to_string())
        else {
            return Err(BufReadError::UnexpectedStringEnumVariant {
                id: recipe_serializer_name.to_string(),
            });
        };
        let identifier = ResourceLocation::read_from(buf)?;

        // rust doesn't let us match ResourceLocation so we have to do a big
        // if-else chain :(
        let data = match recipe_serializer {
            RecipeSerializer::CraftingShaped => {
                RecipeData::CraftingShaped(ShapedRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingShapeless => {
                RecipeData::CraftingShapeless(ShapelessRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingSpecialArmordye => {
                RecipeData::CraftingSpecialArmorDye(SimpleRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingSpecialBookcloning => {
                RecipeData::CraftingSpecialBookCloning(SimpleRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingSpecialMapcloning => {
                RecipeData::CraftingSpecialMapCloning(SimpleRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingSpecialMapextending => {
                RecipeData::CraftingSpecialMapExtending(SimpleRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingSpecialFireworkRocket => {
                RecipeData::CraftingSpecialFireworkRocket(SimpleRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingSpecialFireworkStar => {
                RecipeData::CraftingSpecialFireworkStar(SimpleRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingSpecialFireworkStarFade => {
                RecipeData::CraftingSpecialFireworkStarFade(SimpleRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingSpecialRepairitem => {
                RecipeData::CraftingSpecialRepairItem(SimpleRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingSpecialTippedarrow => {
                RecipeData::CraftingSpecialTippedArrow(SimpleRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingSpecialBannerduplicate => {
                RecipeData::CraftingSpecialBannerDuplicate(SimpleRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingSpecialShielddecoration => {
                RecipeData::CraftingSpecialShieldDecoration(SimpleRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingSpecialShulkerboxcoloring => {
                RecipeData::CraftingSpecialShulkerBoxColoring(SimpleRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingSpecialSuspiciousstew => {
                RecipeData::CraftingSpecialSuspiciousStew(SimpleRecipe::read_from(buf)?)
            }
            RecipeSerializer::Smelting => RecipeData::Smelting(CookingRecipe::read_from(buf)?),
            RecipeSerializer::Blasting => RecipeData::Blasting(CookingRecipe::read_from(buf)?),
            RecipeSerializer::Smoking => RecipeData::Smoking(CookingRecipe::read_from(buf)?),
            RecipeSerializer::CampfireCooking => {
                RecipeData::CampfireCooking(CookingRecipe::read_from(buf)?)
            }
            RecipeSerializer::Stonecutting => {
                RecipeData::Stonecutting(StoneCutterRecipe::read_from(buf)?)
            }
            RecipeSerializer::SmithingTransform => {
                RecipeData::SmithingTransform(SmithingTransformRecipe::read_from(buf)?)
            }
            RecipeSerializer::SmithingTrim => {
                RecipeData::SmithingTrim(SmithingTrimRecipe::read_from(buf)?)
            }
            RecipeSerializer::CraftingDecoratedPot => {
                RecipeData::CraftingDecoratedPot(SimpleRecipe::read_from(buf)?)
            }
        };

        let recipe = Recipe { identifier, data };

        Ok(recipe)
    }
}
