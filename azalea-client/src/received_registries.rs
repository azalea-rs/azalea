use std::collections::HashMap;

use azalea_core::ResourceLocation;
use azalea_nbt::Nbt;
use azalea_protocol::packets::configuration::clientbound_registry_data_packet::registry::{
    DimensionTypeElement, RegistryType,
};
use bevy_ecs::prelude::*;
use serde::de::DeserializeOwned;

/// The registries that were sent to us during the configuration state.
#[derive(Default, Component)]
pub struct ReceivedRegistries {
    pub registries: HashMap<ResourceLocation, Nbt>,
}

impl ReceivedRegistries {
    fn get<T: DeserializeOwned>(&self, name: &ResourceLocation) -> Option<T> {
        let nbt = self.registries.get(&name)?;
        serde_json::from_value(serde_json::to_value(nbt).ok()?).ok()
    }

    /// Get the dimension type registry, or `None` if it doesn't exist. You
    /// should do some type of error handling if this returns `None`.
    pub fn dimension_type(&self) -> Option<RegistryType<DimensionTypeElement>> {
        self.get(&ResourceLocation::new("minecraft:dimension_type"))
    }
}
