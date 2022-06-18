use azalea_core::EntityPos;

#[derive(Default, Debug)]
pub struct Entity {
    /// The incrementing numerical id of the entity.
    pub id: u32,
    pos: EntityPos,
}

impl Entity {
    pub fn pos(&self) -> &EntityPos {
        &self.pos
    }

    pub fn set_pos(&mut self, pos: EntityPos) {
        // TODO: check if it moved to another chunk
        self.pos = pos;
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
