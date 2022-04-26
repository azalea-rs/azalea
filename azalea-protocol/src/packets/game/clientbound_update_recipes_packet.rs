use async_trait::async_trait;
use azalea_chat::component::Component;
use azalea_core::{resource_location::ResourceLocation, Slot};
use packet_macros::GamePacket;
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

#[derive(Clone, Debug)]
pub enum RecipeData {
    CraftingShapeless {
        /// Used to group similar recipes together in the recipe book.
        /// Tag is present in recipe JSON
        group: String,
        // ingredients
        ingredients: Vec<Ingredient>,
        result: Slot,
    },
}

#[derive(Clone, Debug)]
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

            RecipeData::CraftingShapeless {
                group,
                ingredients,
                result,
            }
        } else {
            panic!();
        };

        let recipe = Recipe { identifier, data };

        Ok(recipe)
    }
}

impl McBufWritable for Ingredient {
    fn write_into(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        todo!()
    }
}
#[async_trait]
impl McBufReadable for Ingredient {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        let ingredient = Ingredient {
            allowed: Vec::<Slot>::read_into(buf).await?,
        };
        Ok(ingredient)
    }
}
