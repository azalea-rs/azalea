use crate::Entity;
use azalea_core::PositionDelta;

pub enum MoverType {
    Own,
    Player,
    Piston,
    ShulkerBox,
    Shulker,
}

impl Entity {
    pub fn move_entity(&mut self, mover_type: &MoverType, movement: &PositionDelta) {}
}
