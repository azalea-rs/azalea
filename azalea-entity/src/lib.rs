use azalea_core::EntityPos;

#[derive(Default, Debug)]
pub struct Entity {
    /// The incrementing numerical id of the entity.
    pub id: u32,
    pub pos: EntityPos,
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
