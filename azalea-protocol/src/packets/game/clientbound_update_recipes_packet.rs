use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use azalea_core::resource_location::ResourceLocation;
use azalea_inventory::ItemSlot;
use azalea_protocol_macros::ClientboundGamePacket;

use std::io::{Cursor, Write};

#[derive(Clone, Debug, McBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundUpdateRecipesPacket {
    pub recipes: Vec<RecipeHolder>,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct RecipeHolder {
    pub id: ResourceLocation,
    pub data: RecipeData,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct ShapelessRecipe {
    /// Used to group similar recipes together in the recipe book.
    /// Nbt is present in recipe JSON
    pub group: String,
    pub category: CraftingBookCategory,
    pub ingredients: Vec<Ingredient>,
    pub result: ItemSlot,
}
#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct ShapedRecipe {
    pub group: String,
    pub category: CraftingBookCategory,
    pub pattern: ShapedRecipePattern,
    pub result: ItemSlot,
    pub show_notification: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapedRecipePattern {
    pub width: usize,
    pub height: usize,
    pub ingredients: Vec<Ingredient>,
}

impl McBufWritable for ShapedRecipePattern {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (self.width as u32).var_write_into(buf)?;
        (self.height as u32).var_write_into(buf)?;
        debug_assert_eq!(self.width * self.height, self.ingredients.len());
        for ingredient in &self.ingredients {
            ingredient.write_into(buf)?;
        }
        Ok(())
    }
}

impl McBufReadable for ShapedRecipePattern {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let width = u32::var_read_from(buf)? as usize;
        let height = u32::var_read_from(buf)? as usize;
        let mut ingredients = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            ingredients.push(Ingredient::read_from(buf)?);
        }
        Ok(ShapedRecipePattern {
            width,
            height,
            ingredients,
        })
    }
}

#[derive(Clone, Debug, Copy, PartialEq, McBuf)]
pub enum CraftingBookCategory {
    Building = 0,
    Redstone,
    Equipment,
    Misc,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct CookingRecipe {
    pub group: String,
    pub category: CraftingBookCategory,
    pub ingredient: Ingredient,
    pub result: ItemSlot,
    pub experience: f32,
    #[var]
    pub cooking_time: u32,
}
#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct StoneCutterRecipe {
    pub group: String,
    pub ingredient: Ingredient,
    pub result: ItemSlot,
}
#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct SmithingRecipe {
    pub base: Ingredient,
    pub addition: Ingredient,
    pub result: ItemSlot,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct SimpleRecipe {
    pub category: CraftingBookCategory,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct SmithingTransformRecipe {
    pub template: Ingredient,
    pub base: Ingredient,
    pub addition: Ingredient,
    pub result: ItemSlot,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct SmithingTrimRecipe {
    pub template: Ingredient,
    pub base: Ingredient,
    pub addition: Ingredient,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
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

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct Ingredient {
    pub allowed: Vec<ItemSlot>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crafting_shaped() {
        let mut buf = Vec::new();
        let recipe = RecipeHolder {
            id: ResourceLocation::new("minecraft:crafting_shaped"),
            data: RecipeData::CraftingShaped(ShapedRecipe {
                group: String::new(),
                category: CraftingBookCategory::Building,
                pattern: ShapedRecipePattern {
                    width: 2,
                    height: 2,
                    ingredients: vec![
                        Ingredient {
                            allowed: vec![ItemSlot::Empty],
                        },
                        Ingredient {
                            allowed: vec![ItemSlot::Empty],
                        },
                        Ingredient {
                            allowed: vec![ItemSlot::Empty],
                        },
                        Ingredient {
                            allowed: vec![ItemSlot::Empty],
                        },
                    ],
                },
                result: ItemSlot::Empty,
                show_notification: false,
            }),
        };
        recipe.write_into(&mut buf).unwrap();
        let decoded_recipe = RecipeHolder::read_from(&mut Cursor::new(&buf[..])).unwrap();
        assert_eq!(recipe, decoded_recipe);
    }

    #[test]
    fn test_crafting_shapeless() {
        let mut buf = Vec::new();
        let recipe = RecipeHolder {
            id: ResourceLocation::new("minecraft:crafting_shapeless"),
            data: RecipeData::CraftingShapeless(ShapelessRecipe {
                group: String::new(),
                category: CraftingBookCategory::Building,
                ingredients: vec![
                    Ingredient {
                        allowed: vec![ItemSlot::Empty],
                    },
                    Ingredient {
                        allowed: vec![ItemSlot::Empty],
                    },
                    Ingredient {
                        allowed: vec![ItemSlot::Empty],
                    },
                    Ingredient {
                        allowed: vec![ItemSlot::Empty],
                    },
                ],
                result: ItemSlot::Empty,
            }),
        };
        recipe.write_into(&mut buf).unwrap();
        let decoded_recipe = RecipeHolder::read_from(&mut Cursor::new(&buf[..])).unwrap();
        assert_eq!(recipe, decoded_recipe);
    }

    #[test]
    fn test_crafting_special_armordye() {
        let mut buf = Vec::new();
        let recipe = RecipeHolder {
            id: ResourceLocation::new("minecraft:crafting_special_armordye"),
            data: RecipeData::CraftingSpecialArmorDye(SimpleRecipe {
                category: CraftingBookCategory::Building,
            }),
        };
        recipe.write_into(&mut buf).unwrap();
        let decoded_recipe = RecipeHolder::read_from(&mut Cursor::new(&buf[..])).unwrap();
        assert_eq!(recipe, decoded_recipe);
    }
}
