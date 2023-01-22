use azalea_buf::McBuf;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(McBuf, Debug, Clone, Default)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub name: String,
    pub properties: HashMap<String, ProfilePropertyValue>,
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

impl From<SerializableGameProfile> for GameProfile {
    fn from(value: SerializableGameProfile) -> Self {
        let mut properties = HashMap::new();
        for value in value.properties {
            properties.insert(
                value.name,
                ProfilePropertyValue {
                    value: value.value,
                    signature: value.signature,
                },
            );
        }
        Self {
            uuid: value.id,
            name: value.name,
            properties,
        }
    }
}

#[derive(McBuf, Debug, Clone)]
pub struct ProfilePropertyValue {
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableGameProfile {
    pub id: Uuid,
    pub name: String,
    pub properties: Vec<SerializableProfilePropertyValue>,
}

impl From<GameProfile> for SerializableGameProfile {
    fn from(value: GameProfile) -> Self {
        let mut properties = Vec::new();
        for (key, value) in value.properties {
            properties.push(SerializableProfilePropertyValue {
                name: key,
                value: value.value,
                signature: value.signature,
            });
        }
        Self {
            id: value.uuid,
            name: value.name,
            properties,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableProfilePropertyValue {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}
