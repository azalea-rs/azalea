use std::{
    any::Any,
    fmt::{self, Debug},
    io::{self, Cursor, Write},
};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite, BufReadError};
use azalea_inventory::{
    DataComponentPatch, ItemStack, ItemStackData,
    components::{self, DataComponentUnion},
};
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::{DataComponentKind, Item};

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
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

#[derive(Clone, Debug, AzBuf, PartialEq)]
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
#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct ItemCost {
    pub item: Item,
    #[var]
    pub count: i32,
    pub components: DataComponentExactPredicate,
}
impl ItemCost {
    pub fn into_item_stack(self) -> ItemStackData {
        let mut component_patch = DataComponentPatch::default();
        for component in self.components.expected {
            unsafe {
                component_patch.unchecked_insert_component(component.kind, Some(component.value));
            }
        }
        // TODO: add a fast way to iterate over default components, and insert the ones
        // that aren't present as None

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
#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct DataComponentExactPredicate {
    pub expected: Vec<TypedDataComponent>,
}

pub struct TypedDataComponent {
    kind: DataComponentKind,
    value: DataComponentUnion,
}
impl TypedDataComponent {
    pub fn kind(&self) -> &DataComponentKind {
        &self.kind
    }
    pub fn value(&self) -> &DataComponentUnion {
        &self.value
    }
    pub fn as_dyn(&self) -> &dyn components::EncodableDataComponent {
        // SAFETY: the kind is correct because we got it from azalea_read_as, and the
        // kind isn't mutable
        unsafe { self.value.as_kind(self.kind) }
    }
    pub fn get<T: components::DataComponentTrait>(&self) -> Option<&T> {
        let component = self.as_dyn();
        let component_any = component as &dyn Any;
        component_any.downcast_ref::<T>()
    }
}
impl AzaleaRead for TypedDataComponent {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let kind = DataComponentKind::azalea_read(buf)?;
        let value = DataComponentUnion::azalea_read_as(kind, buf)?;
        Ok(Self { kind, value })
    }
}
impl AzaleaWrite for TypedDataComponent {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        self.kind.azalea_write(buf)?;
        unsafe { self.value.azalea_write_as(self.kind, buf) }
    }
}
impl Debug for TypedDataComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypedDataComponent")
            .field("kind", &self.kind)
            .finish()
    }
}
impl Clone for TypedDataComponent {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind,
            value: unsafe { self.value.clone_as(self.kind) },
        }
    }
}
impl PartialEq for TypedDataComponent {
    fn eq(&self, other: &Self) -> bool {
        if self.kind != other.kind {
            return false;
        }
        self.as_dyn().eq(other.as_dyn())
    }
}
