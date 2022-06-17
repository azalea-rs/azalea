mod behavior;
mod blocks;

pub use behavior::BlockBehavior;
pub use blocks::*;

use std::mem;

impl BlockState {
	/// Transmutes a u32 to a block state. UB if the value is not a valid block
	/// state.
	#[inline]
	pub unsafe fn from_u32_unsafe(state_id: u32) -> Self {
		mem::transmute::<u32, BlockState>(state_id)
	}

	#[inline]
	pub fn is_valid_state(state_id: u32) -> bool {
		state_id <= Self::max_state()
	}
}

impl TryFrom<u32> for BlockState {
	type Error = ();

	/// Safely converts a state id to a block state.
	fn try_from(state_id: u32) -> Result<Self, Self::Error> {
		if Self::is_valid_state(state_id) {
			Ok(unsafe { Self::from_u32_unsafe(state_id) })
		} else {
			Err(())
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_u32() {
		assert_eq!(BlockState::try_from(0).unwrap(), BlockState::Air);

		assert!(BlockState::try_from(BlockState::max_state()).is_ok());
		assert!(BlockState::try_from(BlockState::max_state() + 1).is_err());
	}
}
