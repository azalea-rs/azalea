//! A resource, like minecraft:stone

use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use std::io::{Cursor, Write};

#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

// TODO: make a `resourcelocation!("minecraft:overworld")` macro that checks if
// it's correct at compile-time.

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct ResourceLocation {
    pub namespace: String,
    pub path: String,
}

static DEFAULT_NAMESPACE: &str = "minecraft";
// static REALMS_NAMESPACE: &str = "realms";

impl ResourceLocation {
    pub fn new(resource_string: &str) -> Result<ResourceLocation, BufReadError> {
        let sep_byte_position_option = resource_string.chars().position(|c| c == ':');
        let (namespace, path) = if let Some(sep_byte_position) = sep_byte_position_option {
            if sep_byte_position == 0 {
                (DEFAULT_NAMESPACE, &resource_string[1..])
            } else {
                (
                    &resource_string[..sep_byte_position],
                    &resource_string[sep_byte_position + 1..],
                )
            }
        } else {
            (DEFAULT_NAMESPACE, resource_string)
        };
        Ok(ResourceLocation {
            namespace: namespace.to_string(),
            path: path.to_string(),
        })
    }
}

impl std::fmt::Display for ResourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}
impl std::fmt::Debug for ResourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl McBufReadable for ResourceLocation {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let location_string = String::read_from(buf)?;
        ResourceLocation::new(&location_string)
    }
}
impl McBufWritable for ResourceLocation {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.to_string().write_into(buf)
    }
}

#[cfg(feature = "serde")]
impl Serialize for ResourceLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for ResourceLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.contains(':') {
            match ResourceLocation::new(&s) {
                Ok(r) => Ok(r),
                Err(e) => Err(de::Error::custom(e)),
            }
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Str(&s),
                &"a valid ResourceLocation",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_resource_location() {
        let r = ResourceLocation::new("abcdef:ghijkl").unwrap();
        assert_eq!(r.namespace, "abcdef");
        assert_eq!(r.path, "ghijkl");
    }
    #[test]
    fn no_namespace() {
        let r = ResourceLocation::new("azalea").unwrap();
        assert_eq!(r.namespace, "minecraft");
        assert_eq!(r.path, "azalea");
    }
    #[test]
    fn colon_start() {
        let r = ResourceLocation::new(":azalea").unwrap();
        assert_eq!(r.namespace, "minecraft");
        assert_eq!(r.path, "azalea");
    }
    #[test]
    fn colon_end() {
        let r = ResourceLocation::new("azalea:").unwrap();
        assert_eq!(r.namespace, "azalea");
        assert_eq!(r.path, "");
    }

    #[test]
    fn mcbuf_resource_location() {
        let mut buf = Vec::new();
        ResourceLocation::new("minecraft:dirt")
            .unwrap()
            .write_into(&mut buf)
            .unwrap();

        let mut buf = Cursor::new(&buf[..]);

        assert_eq!(
            ResourceLocation::read_from(&mut buf).unwrap(),
            ResourceLocation::new("minecraft:dirt").unwrap()
        );
    }
}
