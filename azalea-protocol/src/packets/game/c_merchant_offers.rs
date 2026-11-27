use std::fmt::Debug;

use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::{
    builtin::ItemKind,
    item::{
        ItemStack, ItemStackData,
        components::{DataComponent, patch::DataComponentPatch},
    },
};

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundMerchantOffers {
    #[var]
    pub container_id: i32,
    pub offers: Vec<MerchantOffer>,
    #[var]
    pub villager_level: u32,
    #[var]
    pub villager_xp: u32,
    pub show_progress: bool,
    pub can_restock: bool,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct MerchantOffer {
    pub base_cost_a: ItemCost,
    pub result: ItemStack,
    pub cost_b: Option<ItemCost>,
    pub out_of_stock: bool,
    pub uses: i32,
    pub max_uses: i32,
    pub xp: i32,
    pub special_price_diff: i32,
    pub price_multiplier: f32,
    pub demand: i32,
}

/// An item that a merchant can buy.
///
/// This can be converted into an [`ItemStackData`] with
/// [`Self::into_item_stack`].
#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct ItemCost {
    pub item: ItemKind,
    #[var]
    pub count: i32,
    pub components: DataComponentExactPredicate,
}
impl ItemCost {
    pub fn into_item_stack(self) -> ItemStackData {
        let mut component_patch = DataComponentPatch::default();

        for component in self.components.expected {
            component_patch.insert_component(component.kind(), Some(component));
        }
        // TODO: add a fast way to iterate over default components, and insert
        // the ones that aren't present as None

        ItemStackData {
            kind: self.item,
            count: self.count,
            component_patch,
        }
    }
}

/// Similar to [`DataComponentPatch`], but it's only additive, meaning that
/// there are no `None` values.
///
/// If you got this from [`ItemCost`], consider using
/// [`ItemCost::into_item_stack`] for a better API instead.
#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct DataComponentExactPredicate {
    pub expected: Vec<DataComponent>,
}
