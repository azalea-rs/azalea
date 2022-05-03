use crate::Entity;
use azalea_world::World;

#[derive(Default)]
pub struct Player {
    /// The entity attached to the player. There's some useful fields here.
    pub entity: Entity,
}
