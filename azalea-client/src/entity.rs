use azalea_core::EntityPos;

#[derive(Default, Debug)]
pub struct Entity {
    /// The incremental numerical id of the entity.
    pub id: u32,
    pub pos: EntityPos,
}
