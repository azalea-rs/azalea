use azalea_buf::McBuf;
use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize};
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

#[derive(McBuf, Debug, Clone)]
pub struct ProfilePropertyValue {
    pub value: String,
    pub signature: Option<String>,
}

impl Serialize for GameProfile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serializer = serializer.serialize_struct("GameProfile", 3)?;
        serializer.serialize_field("id", &self.uuid)?;
        serializer.serialize_field("name", &self.name)?;
        serializer.serialize_field(
            "properties",
            &SerializedProfilePropertyValue::from_map(self.properties.clone()),
        )?;
        serializer.end()
    }
}

impl<'de> Deserialize<'de> for GameProfile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Id,
            Name,
            Properties,
        }

        struct GameProfileVisitor;
        impl<'de> Visitor<'de> for GameProfileVisitor {
            type Value = GameProfile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct GameProfile")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let id = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let name = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let properties = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                Ok(GameProfile {
                    uuid: id,
                    name,
                    properties: SerializedProfilePropertyValue::into_map(properties),
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;
                let mut properties = None;
                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Properties => {
                            if properties.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            properties = Some(map.next_value()?);
                        }
                    }
                }
                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let properties = SerializedProfilePropertyValue::into_map(
                    properties.ok_or_else(|| serde::de::Error::missing_field("properties"))?,
                );
                Ok(GameProfile {
                    uuid: id,
                    name,
                    properties,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["id", "name", "properties"];
        deserializer.deserialize_struct("GameProfile", FIELDS, GameProfileVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializedProfilePropertyValue {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl SerializedProfilePropertyValue {
    pub fn from_map(
        map: HashMap<String, ProfilePropertyValue>,
    ) -> Vec<SerializedProfilePropertyValue> {
        let mut list: Vec<SerializedProfilePropertyValue> = Vec::new();
        for (key, value) in map {
            list.push(SerializedProfilePropertyValue {
                name: key,
                value: value.value,
                signature: value.signature,
            });
        }
        list
    }

    pub fn into_map(
        list: Vec<SerializedProfilePropertyValue>,
    ) -> HashMap<String, ProfilePropertyValue> {
        let mut map: HashMap<String, ProfilePropertyValue> = HashMap::new();
        for value in list {
            map.insert(
                value.name,
                ProfilePropertyValue {
                    value: value.value,
                    signature: value.signature,
                },
            );
        }
        map
    }
}
