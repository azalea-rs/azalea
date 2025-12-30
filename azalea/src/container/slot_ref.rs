use azalea_inventory::ItemStack;

use crate::container::ContainerHandleRef;

pub struct SlotRef {
    container: ContainerHandleRef,
    index: u16,
}

impl SlotRef {
    pub fn new(container: ContainerHandleRef, index: u16) -> Self {
        Self { container, index }
    }
    /// Returns the index of this slot.
    pub fn index(&self) -> u16 {
        self.index
    }

    /// Returns the [`ItemStack`] at the slot if the container is still open, or
    /// returns `None` if the container is closed.
    ///
    /// Consider using [`Self::map_item`] for performance if you want to avoid
    /// cloning the `ItemStack`.
    pub fn item(&self) -> Option<ItemStack> {
        self.container
            .menu()
            .and_then(|menu| menu.slot(self.index).cloned())
    }

    pub fn map_item<R>(&self, f: impl FnOnce(&ItemStack) -> R) -> Option<R> {
        self.container
            .menu()
            .and_then(|menu| menu.slot(self.index).map(f))
    }

    pub fn left_click(&self) {
        self.container.left_click(self.index);
    }
    pub fn right_click(&self) {
        self.container.right_click(self.index);
    }
    pub fn shift_click(&self) {
        self.container.shift_click(self.index);
    }
}
