#![doc = include_str!("../README.md")]

mod behavior;
pub mod block_state;
pub mod fluid_state;
mod generated;
mod range;

use core::fmt::Debug;
use std::{any::Any, collections::HashMap, str::FromStr};

use azalea_registry::builtin::BlockKind;
pub use behavior::BlockBehavior;
// re-exported for convenience
pub use block_state::BlockState;
pub use generated::{blocks, properties};
pub use range::BlockStates;

pub trait BlockTrait: Debug + Any {
    fn behavior(&self) -> BlockBehavior;
    /// Get the Minecraft string ID for this block.
    ///
    /// For example, `stone` or `grass_block`.
    fn id(&self) -> &'static str;
    /// Convert the block struct to a [`BlockState`].
    ///
    /// This is a lossless conversion, as [`BlockState`] also contains state
    /// data.
    fn as_block_state(&self) -> BlockState;
    /// Convert the block struct to a [`BlockKind`].
    ///
    /// This is a lossy conversion, as [`BlockKind`] doesn't contain any state
    /// data.
    fn as_registry_block(&self) -> BlockKind;

    /// Returns a map of property names on this block to their values as
    /// strings.
    ///
    /// Consider using [`Self::get_property`] if you only need a single
    /// property.
    fn property_map(&self) -> HashMap<&'static str, &'static str>;
    /// Get a property's value as a string by its name, or `None` if the block
    /// has no property with that name.
    ///
    /// To get all properties, you may use [`Self::property_map`].
    ///
    /// To set a property, use [`Self::set_property`].
    fn get_property(&self, name: &str) -> Option<&'static str>;
    /// Update a property on this block, with the name and value being strings.
    ///
    /// Returns `Ok(())`, if the property name and value are valid, otherwise it
    /// returns `Err(InvalidPropertyError)`.
    ///
    /// To get a property, use [`Self::get_property`].
    fn set_property(&mut self, name: &str, new_value: &str) -> Result<(), InvalidPropertyError>;
}

#[derive(Debug)]
pub struct InvalidPropertyError;

impl dyn BlockTrait {
    pub fn downcast_ref<T: BlockTrait>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref::<T>()
    }
}

pub trait Property: FromStr {
    type Value;

    fn try_from_block_state(state: BlockState) -> Option<Self::Value>;

    /// Convert the value of the property to a string, like "x" or "true".
    fn to_static_str(&self) -> &'static str;
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
        let block_from_state = *block_from_state
            .downcast_ref::<crate::blocks::OakTrapdoor>()
            .unwrap();
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
        assert_eq!(property_map.get("facing"), Some(&"east"));
        assert_eq!(property_map.get("half"), Some(&"bottom"));
        assert_eq!(property_map.get("open"), Some(&"true"));
        assert_eq!(property_map.get("powered"), Some(&"false"));
        assert_eq!(property_map.get("waterlogged"), Some(&"false"));
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
        assert_eq!(properties_0.get("stage"), Some(&"0"));
        assert_eq!(sapling_stage_0.get_property("stage"), Some("0"));

        // Test stage 1
        let properties_1 = sapling_stage_1.property_map();
        assert_eq!(properties_1.len(), 1);
        assert_eq!(properties_1.get("stage"), Some(&"1"));
        assert_eq!(sapling_stage_1.get_property("stage"), Some("1"));

        // Test non-existent property
        assert_eq!(sapling_stage_0.get_property("nonexistent"), None);
    }
}
