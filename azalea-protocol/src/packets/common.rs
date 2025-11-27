use azalea_buf::AzBuf;
use azalea_core::{
    data_registry::ResolvableDataRegistry,
    game_type::{GameMode, OptionalGameType},
    identifier::Identifier,
    position::GlobalPos,
    registry_holder::{RegistryHolder, dimension_type::DimensionTypeElement},
    resource_location::ResourceLocation,
};
use tracing::error;

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct CommonPlayerSpawnInfo {
    pub dimension_type: azalea_registry::DimensionType,
    pub dimension: Identifier,
    pub seed: i64,
    pub game_type: GameMode,
    pub previous_game_type: OptionalGameType,
    pub is_debug: bool,
    pub is_flat: bool,
    pub last_death_location: Option<GlobalPos>,
    #[var]
    pub portal_cooldown: u32,
    #[var]
    pub sea_level: i32,
}
impl CommonPlayerSpawnInfo {
    pub fn dimension_type<'a>(
        &self,
        registry_holder: &'a RegistryHolder,
    ) -> Option<(&'a Identifier, &'a DimensionTypeElement)> {
        let dimension_res = self.dimension_type.resolve(registry_holder);
        let Some((dimension_type, dimension_data)) = dimension_res else {
            error!("Couldn't resolve dimension_type {:?}", self.dimension_type);
            return None;
        };

        Some((dimension_type, dimension_data))
    }
}
