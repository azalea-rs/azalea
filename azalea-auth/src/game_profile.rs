use std::{
    io::{self, Write},
    sync::Arc,
};

use azalea_buf::{
    AzBuf, AzaleaRead, AzaleaReadLimited, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;

/// Information about the player that's usually stored on Mojang's servers.
#[derive(Debug, Clone, Default, Eq, PartialEq, AzBuf)]
pub struct GameProfile {
    /// The UUID of the player.
    ///
    /// Typically a UUIDv4 for online-mode players and UUIDv3 for offline-mode
    /// players.
    pub uuid: Uuid,
    /// The username of the player.
    ///
    /// Limited to 16 bytes.
    pub name: String,
    /// The properties of the player, including their in-game skin and cape.
    ///
    /// This is an `Arc` to make it cheaper to clone.
    pub properties: Arc<GameProfileProperties>,
}

impl GameProfile {
    pub fn new(uuid: Uuid, name: String) -> Self {
        GameProfile {
            uuid,
            name,
            properties: Arc::new(GameProfileProperties::default()),
        }
    }
}

impl From<SerializableGameProfile> for GameProfile {
    fn from(value: SerializableGameProfile) -> Self {
        Self {
            uuid: value.id.unwrap_or_default(),
            name: value.name.unwrap_or_default(),
            properties: Arc::new(value.properties.into()),
        }
    }
}

/// The properties of the player, including their in-game skin and cape.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct GameProfileProperties {
    pub map: IndexMap<String, ProfilePropertyValue>,
}
impl AzaleaRead for GameProfileProperties {
    fn azalea_read(buf: &mut io::Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut properties = IndexMap::new();
        let properties_len = u32::azalea_read_var(buf)?;
        if properties_len > 16 {
            return Err(BufReadError::VecLengthTooLong {
                length: properties_len,
                max_length: 16,
            });
        }
        for _ in 0..properties_len {
            let key = String::azalea_read_limited(buf, 16)?;
            let value = ProfilePropertyValue::azalea_read(buf)?;
            properties.insert(key, value);
        }
        Ok(GameProfileProperties { map: properties })
    }
}
impl AzaleaWrite for GameProfileProperties {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        (self.map.len() as u64).azalea_write_var(buf)?;
        for (key, value) in &self.map {
            key.azalea_write(buf)?;
            value.azalea_write(buf)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProfilePropertyValue {
    pub value: String,
    pub signature: Option<String>,
}
impl AzaleaRead for ProfilePropertyValue {
    fn azalea_read(buf: &mut io::Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let value = String::azalea_read_limited(buf, 32767)?;
        let signature = Option::<String>::azalea_read_limited(buf, 1024)?;
        Ok(ProfilePropertyValue { value, signature })
    }
}
impl AzaleaWrite for ProfilePropertyValue {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        self.value.azalea_write(buf)?;
        self.signature.azalea_write(buf)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableGameProfile {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "SerializableProfileProperties::is_empty")]
    pub properties: SerializableProfileProperties,
}

impl From<GameProfile> for SerializableGameProfile {
    fn from(value: GameProfile) -> Self {
        Self {
            id: Some(value.uuid),
            name: Some(value.name),
            properties: (*value.properties).clone().into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct SerializableProfileProperties {
    pub list: Vec<SerializableProfilePropertyValue>,
}
impl SerializableProfileProperties {
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableProfilePropertyValue {
    pub name: String,
    pub value: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl From<GameProfileProperties> for SerializableProfileProperties {
    fn from(value: GameProfileProperties) -> Self {
        let mut list = Vec::new();
        for (name, entry) in value.map {
            list.push(SerializableProfilePropertyValue {
                name,
                value: entry.value,
                signature: entry.signature,
            });
        }
        Self { list }
    }
}
impl From<SerializableProfileProperties> for GameProfileProperties {
    fn from(value: SerializableProfileProperties) -> Self {
        let mut map = IndexMap::new();
        for entry in value.list {
            map.insert(
                entry.name,
                ProfilePropertyValue {
                    value: entry.value,
                    signature: entry.signature,
                },
            );
        }
        Self { map }
    }
}
impl Serialize for GameProfile {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let serializable = SerializableGameProfile::from(self.clone());
        serializable.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_game_profile() {
        let json = r#"{
            "id": "f1a2b3c4-d5e6-f7a8-b9c0-d1e2f3a4b5c6",
            "name": "Notch",
            "properties": [
                {
                    "name": "qwer",
                    "value": "asdf",
                    "signature": "zxcv"
                }
            ]
        }"#;
        let profile =
            GameProfile::from(serde_json::from_str::<SerializableGameProfile>(json).unwrap());
        assert_eq!(
            profile,
            GameProfile {
                uuid: Uuid::parse_str("f1a2b3c4-d5e6-f7a8-b9c0-d1e2f3a4b5c6").unwrap(),
                name: "Notch".to_string(),
                properties: {
                    let mut map = IndexMap::new();
                    map.insert(
                        "qwer".to_string(),
                        ProfilePropertyValue {
                            value: "asdf".to_string(),
                            signature: Some("zxcv".to_string()),
                        },
                    );
                    GameProfileProperties { map }.into()
                },
            }
        );
    }
}
