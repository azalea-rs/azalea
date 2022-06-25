use azalea_entity::Entity;
use azalea_world::World;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct Player {
    /// The player's uuid.
    pub uuid: Uuid,
    /// The player's entity id.
    pub entity_id: u32,
}

impl Player {
    /// Get the entity of the player in the world.
    pub fn entity<'a>(&self, world: &'a World) -> Option<&'a Entity> {
        // world.entity_by_uuid(&self.uuid)
        world.entity_by_id(self.entity_id)
    }

    pub fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }

    pub fn set_entity_id(&mut self, entity_id: u32) {
        self.entity_id = entity_id;
    }
}
