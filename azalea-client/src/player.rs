use azalea_world::entity::Entity;
use azalea_world::Dimension;
use uuid::Uuid;

/// Something that has a dimension associated to it. Usually, this is a `Client`.
pub trait DimensionHaver {
    fn dimension(&self) -> &Dimension;
}

/// A player in the dimension or tab list.
#[derive(Default, Debug)]
pub struct Player {
    /// The player's uuid.
    pub uuid: Uuid,
    /// The player's entity id.
    pub entity_id: u32,
}

impl Player {
    /// Get a reference to the entity of the player in the world.
    pub fn entity<'a>(&self, dimension: &'a Dimension) -> Option<&'a Entity> {
        dimension.entity_by_id(self.entity_id)
    }

    /// Get a mutable reference to the entity of the player in the world.
    pub fn mut_entity<'a>(&self, dimension: &'a mut Dimension) -> Option<&'a mut Entity> {
        dimension.mut_entity_by_id(self.entity_id)
    }

    pub fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }

    pub fn set_entity_id(&mut self, entity_id: u32) {
        self.entity_id = entity_id;
    }
}
