use crate::block_state::{BlockState, BlockStateIntegerRepr};

#[derive(Clone, Debug)]
pub struct FluidState {
    pub kind: FluidKind,
    /// 0 = empty, 8 = full, 9 = max.
    ///
    /// 9 is meant to be used when there's another fluid block of the same type
    /// above it, but it's usually unused by this struct.
    ///
    /// This is different from [`crate::blocks::Water::level`], which is
    /// basically the opposite (0 = full, 8 = empty). You can convert between
    /// the two representations with [`to_or_from_legacy_fluid_level`].
    pub amount: u8,

    /// Whether this fluid is at the max level and there's another fluid of the
    /// same type above it.
    ///
    /// TODO: this is currently unused (always false), make this actually get
    /// set (see FlowingFluid.getFlowing)
    pub falling: bool,
}
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FluidKind {
    #[default]
    Empty,
    Water,
    Lava,
}
impl FluidState {
    pub fn new_source_block(kind: FluidKind, falling: bool) -> Self {
        Self {
            kind,
            amount: 8,
            falling,
        }
    }

    /// A floating point number in between 0 and 1 representing the height (as a
    /// percentage of a full block) of the fluid.
    pub fn height(&self) -> f32 {
        self.amount as f32 / 9.
    }
    pub fn is_empty(&self) -> bool {
        self.amount == 0
    }

    pub fn affects_flow(&self, other: &FluidState) -> bool {
        other.amount == 0 || self.is_same_kind(other)
    }

    pub fn is_same_kind(&self, other: &FluidState) -> bool {
        (other.kind == self.kind) || (self.amount == 0 && other.amount == 0)
    }
}

impl Default for FluidState {
    fn default() -> Self {
        Self {
            kind: FluidKind::Empty,
            amount: 0,
            falling: false,
        }
    }
}

impl From<BlockState> for FluidState {
    fn from(state: BlockState) -> Self {
        // note that 8 here might be treated as 9 in some cases if there's another fluid
        // block of the same type above it

        if state
            .property::<crate::properties::Waterlogged>()
            .unwrap_or_default()
        {
            return Self {
                kind: FluidKind::Water,
                amount: 8,
                falling: false,
            };
        }

        let registry_block = azalea_registry::Block::from(state);
        match registry_block {
            azalea_registry::Block::Water => {
                let level = state
                    .property::<crate::properties::WaterLevel>()
                    .expect("water block should always have WaterLevel");
                return Self {
                    kind: FluidKind::Water,
                    amount: to_or_from_legacy_fluid_level(level as u8),
                    falling: false,
                };
            }
            azalea_registry::Block::Lava => {
                let level = state
                    .property::<crate::properties::LavaLevel>()
                    .expect("lava block should always have LavaLevel");
                return Self {
                    kind: FluidKind::Lava,
                    amount: to_or_from_legacy_fluid_level(level as u8),
                    falling: false,
                };
            }
            azalea_registry::Block::BubbleColumn => {
                return Self::new_source_block(FluidKind::Water, false);
            }
            _ => {}
        }

        Self::default()
    }
}

/// Convert between Minecraft's two fluid level representations.
///
/// This exists because sometimes Minecraft represents fluids with 0 being empty
/// and 8 being full, and sometimes it's the opposite.
///
/// You usually don't need to call this yourself, see [`FluidState`].
pub fn to_or_from_legacy_fluid_level(level: u8) -> u8 {
    // see FlowingFluid.getLegacyLevel
    8_u8.saturating_sub(level)
}

impl From<FluidState> for BlockState {
    fn from(state: FluidState) -> Self {
        match state.kind {
            FluidKind::Empty => BlockState::AIR,
            FluidKind::Water => BlockState::from(crate::blocks::Water {
                level: crate::properties::WaterLevel::from(state.amount as BlockStateIntegerRepr),
            }),
            FluidKind::Lava => BlockState::from(crate::blocks::Lava {
                level: crate::properties::LavaLevel::from(state.amount as BlockStateIntegerRepr),
            }),
        }
    }
}
