enum Block {
    Air = 0,
    Stone,
}

impl Block {
    /// Transmutes a u32 to a Block.
    ///
    /// # Safety
    /// The `id` should be less than {}.
    #[inline]
    pub unsafe fn from_u32_unchecked(id: u32) -> Self {
        mem::transmute::<u32, Block>(id)
    }

    #[inline]
    pub fn is_valid_id(id: u32) -> bool {
        id <= 100
    }
}

impl TryFrom<u32> for Block {
    type Error = ();

    /// Safely converts a state id to a block state.
    fn try_from(id: u32) -> Result<Self, Self::Error> {
        if Self::is_valid_state(state_id) {
            Ok(unsafe { Self::from_u32_unsafe(state_id) })
        } else {
            Err(())
        }
    }
}
