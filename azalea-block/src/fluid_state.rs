use crate::block_state::{BlockState, BlockStateIntegerRepr};

#[derive(Clone, Debug)]
pub struct FluidState {
    pub fluid: azalea_registry::Fluid,
    /// 0 = empty, 8 = full, 9 = max.
    ///
    /// 9 is meant to be used when there's another fluid block of the same type
    /// above it, but it's usually unused by this struct.
    ///
    /// This is different from [`crate::blocks::Water::level`], which is
    /// basically the opposite (0 = full, 8 = empty). You can convert between
    /// the two representations with [`to_or_from_legacy_fluid_level`].
    pub amount: u8,
}
impl FluidState {
    /// A floating point number in between 0 and 1 representing the height (as a
    /// percentage of a full block) of the fluid.
    pub fn height(&self) -> f32 {
        self.amount as f32 / 9.
    }

    pub fn get_flow(world: &Instance, pos: BlockPos) {
        let _ = world;
        let _ = pos;
    }
}

impl Default for FluidState {
    fn default() -> Self {
        Self {
            fluid: azalea_registry::Fluid::Empty,
            amount: 0,
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
            Self {
                fluid: azalea_registry::Fluid::Water,
                amount: 8,
            }
        } else {
            let block = Box::<dyn Block>::from(state);
            if let Some(water) = block.downcast_ref::<crate::blocks::Water>() {
                Self {
                    fluid: azalea_registry::Fluid::Water,
                    amount: to_or_from_legacy_fluid_level(water.level as u8),
                }
            } else if let Some(lava) = block.downcast_ref::<crate::blocks::Lava>() {
                Self {
                    fluid: azalea_registry::Fluid::Lava,
                    amount: to_or_from_legacy_fluid_level(lava.level as u8),
                }
            } else {
                Self {
                    fluid: azalea_registry::Fluid::Empty,
                    amount: 0,
                }
            }
        }
    }
}

/// Sometimes Minecraft represents fluids with 0 being the empty and 8 being
/// full, and sometimes it's the opposite. You can use this function to convert
/// in between those two representations.
///
/// You usually don't need to call this yourself, see [`FluidState`].
pub fn to_or_from_legacy_fluid_level(level: u8) -> u8 {
    // see FlowingFluid.getLegacyLevel
    8_u8.saturating_sub(level)
}

impl From<FluidState> for BlockState {
    fn from(state: FluidState) -> Self {
        match state.fluid {
            azalea_registry::Fluid::Empty => BlockState::AIR,
            azalea_registry::Fluid::Water | azalea_registry::Fluid::FlowingWater => {
                BlockState::from(crate::blocks::Water {
                    level: crate::properties::WaterLevel::from(
                        state.amount as BlockStateIntegerRepr,
                    ),
                })
            }
            azalea_registry::Fluid::Lava | azalea_registry::Fluid::FlowingLava => {
                BlockState::from(crate::blocks::Lava {
                    level: crate::properties::LavaLevel::from(
                        state.amount as BlockStateIntegerRepr,
                    ),
                })
            }
        }
    }
}
