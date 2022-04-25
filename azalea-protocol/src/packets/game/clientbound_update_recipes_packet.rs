use azalea_chat::component::Component;
use azalea_core::resource_location::ResourceLocation;
use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundUpdateRecipesPacket {
    pub recipes: Vec<Recipe>,
}

struct Recipe {
    type_: ResourceLocation,
    identifier: ResourceLocation,
    // data
}
