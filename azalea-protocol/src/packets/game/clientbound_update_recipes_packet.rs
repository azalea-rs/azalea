use std::io::{self, Cursor};

use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufWritable};
use azalea_core::resource_location::ResourceLocation;
use azalea_inventory::ItemSlot;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::HolderSet;

#[derive(Clone, Debug, PartialEq, ClientboundGamePacket)]
pub struct ClientboundUpdateRecipesPacket {
    pub recipes: Vec<RecipeHolder>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecipeHolder {
    pub id: ResourceLocation,
    pub data: RecipeData,
}

// custom reader for additional debug info

impl McBufReadable for ClientboundUpdateRecipesPacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let length = u32::var_read_from(buf)? as usize;
        // we limit the capacity to not get exploited into allocating a bunch
        let mut recipes = Vec::with_capacity(usize::min(length, 65536));

        #[cfg(debug_assertions)]
        {
            let mut last_id = None;
            for _ in 0..length {
                let recipe = RecipeHolder::read_from(buf).map_err(|e| {
                    BufReadError::Custom(format!(
                        "Failed to read RecipeHolder for recipe right after id {:?}: {e}",
                        last_id
                    ))
                })?;
                last_id = Some(recipe.id.clone());
                recipes.push(recipe);
            }
        }
        #[cfg(not(debug_assertions))]
        {
            for _ in 0..length {
                recipes.push(RecipeHolder::read_from(buf)?);
            }
        }

        Ok(ClientboundUpdateRecipesPacket { recipes })
    }
}
impl McBufWritable for ClientboundUpdateRecipesPacket {
    fn write_into(&self, buf: &mut impl io::Write) -> Result<(), io::Error> {
        self.recipes.write_into(buf)
    }
}

impl McBufReadable for RecipeHolder {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = ResourceLocation::read_from(buf)?;
        let data = RecipeData::read_from(buf).map_err(|e| {
            BufReadError::Custom(format!(
                "Failed to read RecipeData for recipe with id {id}: {e}"
            ))
        })?;
        Ok(RecipeHolder { id, data })
    }
}
impl McBufWritable for RecipeHolder {
    fn write_into(&self, buf: &mut impl io::Write) -> Result<(), io::Error> {
        self.id.write_into(buf)?;
        self.data.write_into(buf)
    }
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct ShapelessRecipe {
    /// Used to group similar recipes together in the recipe book.
    /// Nbt is present in recipe JSON
    pub group: String,
    pub category: CraftingBookCategory,
    pub result: ItemSlot,
    pub ingredients: Vec<Ingredient>,
}
#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct ShapedRecipe {
    pub group: String,
    pub category: CraftingBookCategory,
    pub pattern: ShapedRecipePattern,
    pub result: ItemSlot,
    pub show_notification: bool,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct ShapedRecipePattern {
    #[var]
    pub width: u32,
    #[var]
    pub height: u32,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Clone, Debug, Copy, PartialEq, McBuf)]
pub enum CraftingBookCategory {
    Building = 0,
    Redstone,
    Equipment,
    Misc,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct SimpleCookingRecipe {
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
    pub template: ItemSlot,
    pub base: Ingredient,
    pub addition: Ingredient,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct SimpleCraftingRecipe {
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
pub struct TransmuteRecipe {
    pub group: String,
    pub category: CraftingBookCategory,
    pub input: Ingredient,
    pub material: Ingredient,
    pub result: azalea_registry::Item,
}

// see RecipeSerializer.java
#[derive(Clone, Debug, PartialEq, McBuf)]
pub enum RecipeData {
    CraftingShaped(ShapedRecipe),
    CraftingShapeless(ShapelessRecipe),
    CraftingSpecialArmorDye(SimpleCraftingRecipe),
    CraftingSpecialBookCloning(SimpleCraftingRecipe),
    CraftingSpecialMapCloning(SimpleCraftingRecipe),
    CraftingSpecialMapExtending(SimpleCraftingRecipe),
    CraftingSpecialFireworkRocket(SimpleCraftingRecipe),
    CraftingSpecialFireworkStar(SimpleCraftingRecipe),
    CraftingSpecialFireworkStarFade(SimpleCraftingRecipe),
    CraftingSpecialTippedArrow(SimpleCraftingRecipe),
    CraftingSpecialBannerDuplicate(SimpleCraftingRecipe),
    CraftingSpecialShieldDecoration(SimpleCraftingRecipe),
    Transmute(TransmuteRecipe),
    CraftingSpecialRepairItem(SimpleCraftingRecipe),
    Smelting(SimpleCookingRecipe),
    Blasting(SimpleCookingRecipe),
    Smoking(SimpleCookingRecipe),
    CampfireCooking(SimpleCookingRecipe),
    Stonecutting(StoneCutterRecipe),
    SmithingTransform(SmithingTransformRecipe),
    SmithingTrim(SmithingTrimRecipe),
    CraftingDecoratedPot(SimpleCraftingRecipe),
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct Ingredient {
    pub allowed: HolderSet<azalea_registry::Item, ResourceLocation>,
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
                            allowed: HolderSet::Direct { contents: vec![] },
                        },
                        Ingredient {
                            allowed: HolderSet::Direct { contents: vec![] },
                        },
                        Ingredient {
                            allowed: HolderSet::Direct { contents: vec![] },
                        },
                        Ingredient {
                            allowed: HolderSet::Direct { contents: vec![] },
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
                        allowed: HolderSet::Direct { contents: vec![] },
                    },
                    Ingredient {
                        allowed: HolderSet::Direct { contents: vec![] },
                    },
                    Ingredient {
                        allowed: HolderSet::Direct { contents: vec![] },
                    },
                    Ingredient {
                        allowed: HolderSet::Direct { contents: vec![] },
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
            data: RecipeData::CraftingSpecialArmorDye(SimpleCraftingRecipe {
                category: CraftingBookCategory::Building,
            }),
        };
        recipe.write_into(&mut buf).unwrap();
        let decoded_recipe = RecipeHolder::read_from(&mut Cursor::new(&buf[..])).unwrap();
        assert_eq!(recipe, decoded_recipe);
    }
}
