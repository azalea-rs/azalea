use azalea_world::entity::Entity;
use azalea_world::World;
use uuid::Uuid;

/// Something that has a world associated to it. Usually, this is a `Client`.
pub trait WorldHaver {
    fn world(&self) -> &World;
}

/// A player in the world or tab list.
#[derive(Default, Debug)]
pub struct Player {
    /// The player's uuid.
    pub uuid: Uuid,
    /// The player's entity id.
    pub entity_id: u32,
}

impl Player {
    /// Get a reference to the entity of the player in the world.
    pub fn entity<'d>(&'d self, world: &'d World) -> Option<Entity<&World>> {
        world.entity(self.entity_id)
    }

    /// Get a mutable reference to the entity of the player in the world.
    pub fn entity_mut<'d>(&'d self, world: &'d mut World) -> Option<Entity> {
        world.entity_mut(self.entity_id)
    }

    pub fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }

    pub fn set_entity_id(&mut self, entity_id: u32) {
        self.entity_id = entity_id;
    }
}
