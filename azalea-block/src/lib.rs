#![doc = include_str!("../README.md")]

mod behavior;
pub mod block_state;
pub mod fluid_state;
mod generated;
mod range;

use core::fmt::Debug;
use std::{any::Any, collections::HashMap};

pub use behavior::BlockBehavior;
// re-exported for convenience
pub use block_state::BlockState;
pub use generated::{blocks, properties};
pub use range::BlockStates;

pub trait BlockTrait: Debug + Any {
    fn behavior(&self) -> BlockBehavior;
    /// Get the Minecraft ID for this block. For example `stone` or
    /// `grass_block`.
    fn id(&self) -> &'static str;
    /// Convert the block to a block state. This is lossless, as the block
    /// contains all the state data.
    fn as_block_state(&self) -> BlockState;
    /// Convert the block to an [`azalea_registry::Block`]. This is lossy, as
    /// `azalea_registry::Block` doesn't contain any state data.
    fn as_registry_block(&self) -> azalea_registry::Block;

    fn property_map(&self) -> HashMap<String, String>;

    fn get_property(&self, name: &str) -> Option<String>;
}

impl dyn BlockTrait {
    pub fn downcast_ref<T: BlockTrait>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref::<T>()
    }
}

pub trait Property {
    type Value;

    fn try_from_block_state(state: BlockState) -> Option<Self::Value>;
}

#[cfg(test)]
mod tests {
    use crate::BlockTrait;

    #[test]
    pub fn roundtrip_block_state() {
        let block = crate::blocks::OakTrapdoor {
            facing: crate::properties::FacingCardinal::East,
            half: crate::properties::TopBottom::Bottom,
            open: true,
            powered: false,
            waterlogged: false,
        };
        let block_state = block.as_block_state();
        let block_from_state = Box::<dyn BlockTrait>::from(block_state);
        let block_from_state = block_from_state
            .downcast_ref::<crate::blocks::OakTrapdoor>()
            .unwrap()
            .clone();
        assert_eq!(block, block_from_state);
    }

    #[test]
    pub fn test_property_map() {
        let block = crate::blocks::OakTrapdoor {
            facing: crate::properties::FacingCardinal::East,
            half: crate::properties::TopBottom::Bottom,
            open: true,
            powered: false,
            waterlogged: false,
        };

        let property_map = block.property_map();

        assert_eq!(property_map.len(), 5);
        assert_eq!(property_map.get("facing"), Some(&"east".to_string()));
        assert_eq!(property_map.get("half"), Some(&"bottom".to_string()));
        assert_eq!(property_map.get("open"), Some(&"true".to_string()));
        assert_eq!(property_map.get("powered"), Some(&"false".to_string()));
        assert_eq!(property_map.get("waterlogged"), Some(&"false".to_string()));
    }

    #[test]
    pub fn test_integer_properties() {
        // Test with oak sapling that has an integer-like stage property
        let sapling_stage_0 = crate::blocks::OakSapling {
            stage: crate::properties::OakSaplingStage::_0,
        };

        let sapling_stage_1 = crate::blocks::OakSapling {
            stage: crate::properties::OakSaplingStage::_1,
        };

        // Test stage 0
        let properties_0 = sapling_stage_0.property_map();
        assert_eq!(properties_0.len(), 1);
        assert_eq!(properties_0.get("stage"), Some(&"0".to_string()));
        assert_eq!(sapling_stage_0.get_property("stage"), Some("0".to_string()));

        // Test stage 1
        let properties_1 = sapling_stage_1.property_map();
        assert_eq!(properties_1.len(), 1);
        assert_eq!(properties_1.get("stage"), Some(&"1".to_string()));
        assert_eq!(sapling_stage_1.get_property("stage"), Some("1".to_string()));

        // Test non-existent property
        assert_eq!(sapling_stage_0.get_property("nonexistent"), None);
    }
}
