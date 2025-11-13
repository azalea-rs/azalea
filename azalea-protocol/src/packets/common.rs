use azalea_buf::AzBuf;
use azalea_core::{
    data_registry::ResolvableDataRegistry,
    game_type::{GameMode, OptionalGameType},
    identifier::Identifier,
    position::GlobalPos,
    registry_holder::{DimensionTypeElement, RegistryHolder},
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
    pub fn dimension_type(
        &self,
        registry_holder: &RegistryHolder,
    ) -> Option<(Identifier, DimensionTypeElement)> {
        let dimension_res = self
            .dimension_type
            .resolve_and_deserialize::<DimensionTypeElement>(registry_holder);
        let Some(dimension_res) = dimension_res else {
            error!("Couldn't resolve dimension_type {:?}", self.dimension_type);
            return None;
        };
        let (dimension_type, dimension_data) = match dimension_res {
            Ok(d) => d,
            Err(err) => {
                error!(
                    "Couldn't deserialize dimension_type {:?}: {err:?}",
                    self.dimension_type
                );
                return None;
            }
        };

        Some((dimension_type, dimension_data))
    }
}
