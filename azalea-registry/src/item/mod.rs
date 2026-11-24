// this module is contained in azalea-registry (as opposed to being in
// azalea-inventory or its own thing) because Particle needs ItemStack. maybe we
// should move `item` and `registry_holder` to its own crate? idk, it's probably
// fine like this for now.

pub mod checksum;
pub mod components;
pub mod consume_effect;
pub mod default_components;

use std::{
    borrow::Cow,
    fmt::Debug,
    io::{self, Cursor, Write},
};

use azalea_buf::{AzBuf, AzBufVar, BufReadError};
use azalea_codec::Codec;

use crate::{
    builtin::ItemKind,
    item::{
        components::{
            DataComponentUnion, MaxStackSize,
            patch::{DataComponentPatch, EMPTY_DATA_COMPONENT_PATCH},
        },
        default_components::get_default_component,
    },
};

pub trait MaxStackSizeExt {
    /// Get the maximum stack size for this item.
    ///
    /// This is a signed integer to be consistent with the `count` field of
    /// [`ItemStackData`].
    ///
    /// [`ItemStackData`]: crate::ItemStackData
    fn max_stack_size(&self) -> i32;

    /// Whether this item can be stacked with other items.
    ///
    /// This is equivalent to `self.max_stack_size() > 1`.
    fn stackable(&self) -> bool {
        self.max_stack_size() > 1
    }
}

impl MaxStackSizeExt for ItemKind {
    fn max_stack_size(&self) -> i32 {
        get_default_component::<MaxStackSize>(*self).map_or(64, |s| s.count)
    }
}

/// Either an item in an inventory or nothing.
#[derive(Clone, Debug, Default, PartialEq, Codec)]
pub enum ItemStack {
    #[codec(unit)]
    #[default]
    Empty,
    Present(ItemStackData),
}

impl ItemStack {
    /// Create a new [`ItemStack`] with the given number of [`ItemKind`]s.
    ///
    /// If item is air or the count isn't positive, then it'll be set to an
    /// empty `ItemStack`.
    pub fn new(item: ItemKind, count: i32) -> Self {
        let mut i = ItemStack::Present(ItemStackData::new(item, count));
        // set it to Empty if the item is air or if the count isn't positive
        i.update_empty();
        i
    }

    /// Check if the slot is ItemStack::Empty, if the count is <= 0, or if the
    /// item is air.
    ///
    /// This is the opposite of [`ItemStack::is_present`].
    pub fn is_empty(&self) -> bool {
        match self {
            ItemStack::Empty => true,
            ItemStack::Present(item) => item.is_empty(),
        }
    }
    /// Check if the slot is not ItemStack::Empty, if the count is > 0, and if
    /// the item is not air.
    ///
    /// This is the opposite of [`ItemStack::is_empty`].
    pub fn is_present(&self) -> bool {
        !self.is_empty()
    }

    /// Return the amount of the item in the slot, or 0 if the slot is empty.
    ///
    /// Note that it's possible for the count to be zero or negative when the
    /// slot is present.
    pub fn count(&self) -> i32 {
        match self {
            ItemStack::Empty => 0,
            ItemStack::Present(i) => i.count,
        }
    }

    /// Remove `count` items from this slot, returning the removed items.
    pub fn split(&mut self, count: u32) -> ItemStack {
        match self {
            ItemStack::Empty => ItemStack::Empty,
            ItemStack::Present(i) => {
                let returning = i.split(count);
                if i.is_empty() {
                    *self = ItemStack::Empty;
                }
                ItemStack::Present(returning)
            }
        }
    }

    /// Get the `kind` of the item in this slot, or [`ItemKind::Air`]
    pub fn kind(&self) -> ItemKind {
        match self {
            ItemStack::Empty => ItemKind::Air,
            ItemStack::Present(i) => i.kind,
        }
    }

    /// Update whether this slot is empty, based on the count.
    pub fn update_empty(&mut self) {
        if let ItemStack::Present(i) = self
            && i.is_empty()
        {
            *self = ItemStack::Empty;
        }
    }

    /// Convert this slot into an [`ItemStackData`], if it's present.
    pub fn as_present(&self) -> Option<&ItemStackData> {
        match self {
            ItemStack::Empty => None,
            ItemStack::Present(i) => Some(i),
        }
    }

    pub fn as_present_mut(&mut self) -> Option<&mut ItemStackData> {
        match self {
            ItemStack::Empty => None,
            ItemStack::Present(i) => Some(i),
        }
    }

    pub fn component_patch(&self) -> &DataComponentPatch {
        self.as_present()
            .map_or_else(|| &*EMPTY_DATA_COMPONENT_PATCH, |i| &i.component_patch)
    }

    /// Get the value of a data component for this item.
    ///
    /// This is used for things like getting the damage of an item, or seeing
    /// how much food it replenishes.
    pub fn get_component<'a, T: components::DataComponentTrait>(&'a self) -> Option<Cow<'a, T>> {
        self.as_present().and_then(|i| i.get_component::<T>())
    }

    pub fn with_component<
        T: components::EncodableDataComponent + components::DataComponentTrait,
    >(
        mut self,
        component: impl Into<Option<T>>,
    ) -> Self {
        if let ItemStack::Present(i) = &mut self {
            let component: Option<T> = component.into();
            let component: Option<DataComponentUnion> = component.map(|c| c.into());
            unsafe {
                i.component_patch
                    .unchecked_insert_component(T::KIND, component);
            }
        }
        self
    }
}

/// An item in an inventory, with a count and a set of data components.
///
/// Usually you want [`ItemStack`] or
/// [`ItemKind`](azalea_registry::builtin::ItemKind) instead.
#[derive(Clone, Debug, PartialEq, Codec)]
pub struct ItemStackData {
    #[codec(rename = "id")]
    pub kind: ItemKind,
    /// The amount of the item in this slot.
    ///
    /// The count can be zero or negative, but this is rare.
    pub count: i32,
    /// The item's components that the server set to be different from the
    /// defaults.
    #[codec(optional, rename = "components")]
    pub component_patch: DataComponentPatch,
}

impl ItemStackData {
    /// Create a new [`ItemStackData`] with the given number of [`ItemKind`]s.
    pub fn new(item: ItemKind, count: i32) -> Self {
        ItemStackData {
            count,
            kind: item,
            component_patch: Default::default(),
        }
    }

    /// Remove `count` items from this slot, returning the removed items.
    pub fn split(&mut self, count: u32) -> ItemStackData {
        let returning_count = i32::min(count as i32, self.count);
        let mut returning = self.clone();
        returning.count = returning_count;
        self.count -= returning_count;
        returning
    }

    /// Check if the count of the item is <= 0 or if the item is air.
    pub fn is_empty(&self) -> bool {
        self.count <= 0 || self.kind == ItemKind::Air
    }

    /// Whether this item is the same as another item, ignoring the count.
    ///
    /// ```
    /// # use azalea_inventory::ItemStackData;
    /// # use azalea_registry::builtin::ItemKind;
    /// let mut a = ItemStackData::from(ItemKind::Stone);
    /// let mut b = ItemStackData::new(ItemKind::Stone, 2);
    /// assert!(a.is_same_item_and_components(&b));
    ///
    /// b.kind = ItemKind::Dirt;
    /// assert!(!a.is_same_item_and_components(&b));
    /// ```
    pub fn is_same_item_and_components(&self, other: &ItemStackData) -> bool {
        self.kind == other.kind && self.component_patch == other.component_patch
    }

    /// Get the value of a data component for this item.
    ///
    /// This is used for things like getting the damage of an item, or seeing
    /// how much food it replenishes.
    pub fn get_component<'a, T: components::DataComponentTrait>(&'a self) -> Option<Cow<'a, T>> {
        if let Some(c) = self.component_patch.get::<T>() {
            Some(Cow::Borrowed(c))
        } else {
            get_default_component::<T>(self.kind).map(|c| Cow::Owned(c))
        }
    }
}

impl AzBuf for ItemStack {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let count = i32::azalea_read_var(buf)?;
        if count <= 0 {
            Ok(ItemStack::Empty)
        } else {
            let kind = ItemKind::azalea_read(buf)?;
            let component_patch = DataComponentPatch::azalea_read(buf)?;
            Ok(ItemStack::from(ItemStackData {
                count,
                kind,
                component_patch,
            }))
        }
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        match self {
            ItemStack::Empty => 0_i32.azalea_write_var(buf)?,
            ItemStack::Present(i) => {
                i.count.azalea_write_var(buf)?;
                i.kind.azalea_write(buf)?;
                i.component_patch.azalea_write(buf)?;
            }
        };
        Ok(())
    }
}

impl From<ItemStackData> for ItemStack {
    fn from(item: ItemStackData) -> Self {
        if item.is_empty() {
            ItemStack::Empty
        } else {
            ItemStack::Present(item)
        }
    }
}
impl From<ItemKind> for ItemStack {
    fn from(item: ItemKind) -> Self {
        ItemStack::new(item, 1)
    }
}
impl From<(ItemKind, i32)> for ItemStack {
    fn from(item: (ItemKind, i32)) -> Self {
        ItemStack::new(item.0, item.1)
    }
}
impl From<ItemKind> for ItemStackData {
    fn from(item: ItemKind) -> Self {
        ItemStackData::new(item, 1)
    }
}
impl From<(ItemKind, i32)> for ItemStackData {
    fn from(item: (ItemKind, i32)) -> Self {
        ItemStackData::new(item.0, item.1)
    }
}
