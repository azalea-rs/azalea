use azalea_buf::McBuf;

use crate::Menu;

#[derive(Debug, Clone)]
pub enum ClickOperation {
    Pickup(PickupClick),
    QuickMove(QuickMoveClick),
    Swap(SwapClick),
    Clone(CloneClick),
    Throw(ThrowClick),
    QuickCraft(QuickCraftClick),
    PickupAll(PickupAllClick),
}

#[derive(Debug, Clone)]
pub enum PickupClick {
    /// Left mouse click
    Left { slot: u16 },
    /// Right mouse click
    Right { slot: u16 },
    /// Drop cursor stack
    LeftOutside,
    /// Drop cursor single item
    RightOutside,
}
impl From<PickupClick> for ClickOperation {
    fn from(click: PickupClick) -> Self {
        ClickOperation::Pickup(click)
    }
}
/// Shift click
#[derive(Debug, Clone)]
pub enum QuickMoveClick {
    /// Shift + left mouse click
    Left { slot: u16 },
    /// Shift + right mouse click (identical behavior)
    Right { slot: u16 },
}
impl From<QuickMoveClick> for ClickOperation {
    fn from(click: QuickMoveClick) -> Self {
        ClickOperation::QuickMove(click)
    }
}
#[derive(Debug, Clone)]
pub enum SwapClick {
    /// Like number keys 1-9 (but 0 indexed so actually 0-8)
    Hotbar { slot: u16, index: u8 },
    /// Offhand swap key F
    Offhand { slot: u16 },
}
impl From<SwapClick> for ClickOperation {
    fn from(click: SwapClick) -> Self {
        ClickOperation::Swap(click)
    }
}
/// Middle click, only defined for creative players in non-player
/// inventories.
#[derive(Debug, Clone)]
pub struct CloneClick {
    pub slot: u16,
}
impl From<CloneClick> for ClickOperation {
    fn from(click: CloneClick) -> Self {
        ClickOperation::Clone(click)
    }
}
#[derive(Debug, Clone)]
pub enum ThrowClick {
    /// Drop key (Q)
    Single { slot: u16 },
    /// Ctrl + drop key (Q)
    All { slot: u16 },
}
impl From<ThrowClick> for ClickOperation {
    fn from(click: ThrowClick) -> Self {
        ClickOperation::Throw(click)
    }
}
#[derive(Debug, Clone)]
pub enum QuickCraftClick {
    /// Starting left mouse drag
    StartLeft,
    /// Starting right mouse drag
    StartRight,
    /// Starting middle mouse drag, only defined for creative players in
    /// non-player inventories.
    StartMiddle,
    /// Add slot for left-mouse drag
    AddLeft { slot: u16 },
    /// Add slot for right-mouse drag
    AddRight { slot: u16 },
    /// Add slot for middle-mouse drag, only defined for creative
    /// players in
    AddMiddle { slot: u16 },
    /// Ending left mouse drag
    EndLeft,
    /// Ending right mouse drag
    EndRight,
    /// Ending middle mouse drag, only defined for creative players in
    /// non-player inventories.
    EndMiddle,
}
/// Double click
#[derive(Debug, Clone)]
pub struct PickupAllClick {
    pub slot: u16,
}
impl From<PickupAllClick> for ClickOperation {
    fn from(click: PickupAllClick) -> Self {
        ClickOperation::PickupAll(click)
    }
}

impl Menu {
    /// Modify the inventory as if the given operation was performed on it.
    pub fn click(&mut self, operation: &ClickOperation) {
        // TODO
    }
}

impl ClickOperation {
    /// Return the slot number that this operation is acting on, if any.
    ///
    /// Note that in the protocol, "None" is represented as -999.
    pub fn slot_num(&self) -> Option<u16> {
        match self {
            ClickOperation::Pickup(pickup) => match pickup {
                PickupClick::Left { slot } => Some(*slot),
                PickupClick::Right { slot } => Some(*slot),
                PickupClick::LeftOutside => None,
                PickupClick::RightOutside => None,
            },
            ClickOperation::QuickMove(quick_move) => match quick_move {
                QuickMoveClick::Left { slot } => Some(*slot),
                QuickMoveClick::Right { slot } => Some(*slot),
            },
            ClickOperation::Swap(swap) => match swap {
                SwapClick::Hotbar { slot, .. } => Some(*slot),
                SwapClick::Offhand { slot } => Some(*slot),
            },
            ClickOperation::Clone(clone) => Some(clone.slot),
            ClickOperation::Throw(throw) => match throw {
                ThrowClick::Single { slot } => Some(*slot),
                ThrowClick::All { slot } => Some(*slot),
            },
            ClickOperation::QuickCraft(quick_craft) => match quick_craft {
                QuickCraftClick::StartLeft => None,
                QuickCraftClick::StartRight => None,
                QuickCraftClick::StartMiddle => None,
                QuickCraftClick::AddLeft { slot } => Some(*slot),
                QuickCraftClick::AddRight { slot } => Some(*slot),
                QuickCraftClick::AddMiddle { slot } => Some(*slot),
                QuickCraftClick::EndLeft => None,
                QuickCraftClick::EndRight => None,
                QuickCraftClick::EndMiddle => None,
            },
            ClickOperation::PickupAll(pickup_all) => Some(pickup_all.slot),
        }
    }

    pub fn button_num(&self) -> u8 {
        match self {
            ClickOperation::Pickup(pickup) => match pickup {
                PickupClick::Left { .. } => 0,
                PickupClick::Right { .. } => 1,
                PickupClick::LeftOutside => 0,
                PickupClick::RightOutside => 1,
            },
            ClickOperation::QuickMove(quick_move) => match quick_move {
                QuickMoveClick::Left { .. } => 0,
                QuickMoveClick::Right { .. } => 1,
            },
            ClickOperation::Swap(swap) => match swap {
                SwapClick::Hotbar { index, .. } => *index,
                SwapClick::Offhand { .. } => 40,
            },
            ClickOperation::Clone(_) => 2,
            ClickOperation::Throw(throw) => match throw {
                ThrowClick::Single { .. } => 0,
                ThrowClick::All { .. } => 1,
            },
            ClickOperation::QuickCraft(quick_craft) => match quick_craft {
                QuickCraftClick::StartLeft => 0,
                QuickCraftClick::StartRight => 4,
                QuickCraftClick::StartMiddle => 8,
                QuickCraftClick::AddLeft { .. } => 1,
                QuickCraftClick::AddRight { .. } => 5,
                QuickCraftClick::AddMiddle { .. } => 9,
                QuickCraftClick::EndLeft => 2,
                QuickCraftClick::EndRight => 6,
                QuickCraftClick::EndMiddle => 10,
            },
            ClickOperation::PickupAll(_) => 0,
        }
    }

    pub fn click_type(&self) -> ClickType {
        match self {
            ClickOperation::Pickup(_) => ClickType::Pickup,
            ClickOperation::QuickMove(_) => ClickType::QuickMove,
            ClickOperation::Swap(_) => ClickType::Swap,
            ClickOperation::Clone(_) => ClickType::Clone,
            ClickOperation::Throw(_) => ClickType::Throw,
            ClickOperation::QuickCraft(_) => ClickType::QuickCraft,
            ClickOperation::PickupAll(_) => ClickType::PickupAll,
        }
    }
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum ClickType {
    Pickup = 0,
    QuickMove = 1,
    Swap = 2,
    Clone = 3,
    Throw = 4,
    QuickCraft = 5,
    PickupAll = 6,
}
