use azalea_buf::AzBuf;
use azalea_core::{checksum::Checksum, registry_holder::RegistryHolder};
use azalea_inventory::{ItemStack, operations::ClickType};
use azalea_protocol_macros::ServerboundGamePacket;
use indexmap::IndexMap;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundContainerClick {
    #[var]
    pub container_id: i32,
    #[var]
    pub state_id: u32,
    pub slot_num: i16,
    pub button_num: u8,
    pub click_type: ClickType,
    pub changed_slots: IndexMap<u16, HashedStack>,
    pub carried_item: HashedStack,
}

/// Similar to an [`ItemStack`] but only carrying a CRC32 hash of the value of
/// added data components instead of their entire contents.
#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct HashedStack(pub Option<HashedActualItem>);

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct HashedActualItem {
    pub kind: azalea_registry::Item,
    #[var]
    pub count: i32,
    pub components: HashedPatchMap,
}

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct HashedPatchMap {
    #[limit(256)]
    pub added_components: Vec<(azalea_registry::DataComponentKind, Checksum)>,
    #[limit(256)]
    pub removed_components: Vec<azalea_registry::DataComponentKind>,
}

impl HashedStack {
    /// Convert your [`ItemStack`] into a [`HashedStack`] by hashing the data
    /// components.
    ///
    /// Minecraft uses this whenever the client sends data components to the
    /// server.
    ///
    /// The [`RegistryHolder`] is required as some components will hash
    /// differently based on the server's registries.
    pub fn from_item_stack(item: &ItemStack, registries: &RegistryHolder) -> Self {
        let ItemStack::Present(item) = item else {
            return HashedStack(None);
        };

        let mut added_components = Vec::new();
        let mut removed_components = Vec::new();

        for (kind, data) in item.component_patch.iter() {
            if let Some(data) = data {
                added_components.push((kind, data.crc_hash(registries)));
            } else {
                removed_components.push(kind);
            }
        }

        let components = HashedPatchMap {
            added_components,
            removed_components,
        };
        let item = HashedActualItem {
            kind: item.kind,
            count: item.count,
            components,
        };
        Self(Some(item))
    }
}
