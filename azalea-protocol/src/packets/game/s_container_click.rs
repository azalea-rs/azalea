use std::collections::HashMap;

use azalea_buf::{AzBuf, AzaleaWrite};
use azalea_inventory::{ItemStack, operations::ClickType};
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundContainerClick {
    #[var]
    pub container_id: i32,
    #[var]
    pub state_id: u32,
    pub slot_num: i16,
    pub button_num: u8,
    pub click_type: ClickType,
    pub changed_slots: HashMap<u16, HashedStack>,
    pub carried_item: HashedStack,
}

/// Similar to an [`ItemStack`] but only carrying a CRC32 hash of the value of
/// added data components instead of their entire contents.
#[derive(Clone, Debug, AzBuf)]
pub struct HashedStack(pub Option<HashedActualItem>);

#[derive(Clone, Debug, AzBuf)]
pub struct HashedActualItem {
    pub kind: azalea_registry::Item,
    #[var]
    pub count: i32,
    pub components: HashedPatchMap,
}

#[derive(Clone, Debug, AzBuf)]
pub struct HashedPatchMap {
    /// The value is a CRC32 hash of the data component's network serialization.
    /// (kind + data)
    #[limit(256)]
    pub added_components: Vec<(azalea_registry::DataComponentKind, u32)>,
    #[limit(256)]
    pub removed_components: Vec<azalea_registry::DataComponentKind>,
}

/// Convert your [`ItemStack`] into a [`HashedStack`] by hashing the data
/// components.
///
/// This will be necessary if you're writing a client or server, but if you're
/// just making a proxy then you can remove the `crc32` dependency by disabling
/// the `crc32` feature on `azalea-protocol`.
#[cfg(feature = "crc32")]
impl From<&ItemStack> for HashedStack {
    fn from(item: &ItemStack) -> Self {
        let ItemStack::Present(item) = item else {
            return Self(None);
        };

        let mut added_components = Vec::new();
        let mut removed_components = Vec::new();

        for (&kind, data) in &item.component_patch.components {
            if let Some(data) = data {
                // encodeCap in TypedDataComponent.java
                let mut buf = Vec::new();
                kind.azalea_write(&mut buf).unwrap();
                data.encode(&mut buf).unwrap();
                added_components.push((kind, crc32fast::hash(&buf)));
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
