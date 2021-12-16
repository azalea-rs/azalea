use std::collections::HashMap;

use uuid::Uuid;

#[derive(Hash, Clone, Debug)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub name: String,
    pub properties: HashMap<String, String>,
}

impl GameProfile {
    pub fn new(uuid: Uuid, name: String) -> Self {
        GameProfile {
            uuid,
            name,
            properties: HashMap::new(),
        }
    }
}
