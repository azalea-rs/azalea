use azalea_core::Slot;

/// This inventory has a normal "grid" inventory you can put items in.
pub trait HasMainInventory {
    /// Get the width of the main inventory.
    fn width(&self) -> usize;
    /// Get the height of the main inventory.
    fn height(&self) -> usize;
    /// Get the slot at the given x and y position in the main inventory.
    fn slot(&self, x: usize, y: usize) -> Option<&Slot>;
}

/// A basic representation of a Minecraft inventory.
struct Inventory<const SIZE: usize> {
    /// Every slot in the inventory. This will always include the player's
    /// inventory, so you probably don't want this.
    slots: [Slot; SIZE],
}

pub struct Generic9x1 {
    inventory: Inventory<9>,
}
pub struct Generic9x2 {
    inventory: Inventory<18>,
}
