//! An arbitrary identifier or resource location.

use std::{
    fmt,
    io::{self, Cursor, Write},
    str::FromStr,
};

use azalea_buf::{AzaleaRead, AzaleaWrite, BufReadError};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use simdnbt::{FromNbtTag, ToNbtTag, owned::NbtTag};

/// An identifier, like `minecraft:stone` or `brigadier:number`.
///
/// This was formerly called a `ResourceLocation`.
#[doc(alias = "ResourceLocation")]
#[derive(Hash, Clone, PartialEq, Eq, Default)]
pub struct Identifier {
    pub namespace: String,
    pub path: String,
}

static DEFAULT_NAMESPACE: &str = "minecraft";
// static REALMS_NAMESPACE: &str = "realms";

impl Identifier {
    pub fn new(resource_string: &str) -> Identifier {
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
        Identifier {
            namespace: namespace.to_string(),
            path: path.to_string(),
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}
impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}
impl FromStr for Identifier {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Identifier::new(s))
    }
}
impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Identifier::new(s)
    }
}

impl AzaleaRead for Identifier {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let location_string = String::azalea_read(buf)?;
        Ok(Identifier::new(&location_string))
    }
}
impl AzaleaWrite for Identifier {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        self.to_string().azalea_write(buf)
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.contains(':') {
            Ok(Identifier::new(&s))
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Str(&s),
                &"a valid Identifier",
            ))
        }
    }
}

impl FromNbtTag for Identifier {
    fn from_nbt_tag(tag: simdnbt::borrow::NbtTag) -> Option<Self> {
        tag.string().and_then(|s| s.to_str().parse().ok())
    }
}

impl ToNbtTag for Identifier {
    fn to_nbt_tag(self) -> NbtTag {
        NbtTag::String(self.to_string().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_identifier() {
        let r = Identifier::new("abcdef:ghijkl");
        assert_eq!(r.namespace, "abcdef");
        assert_eq!(r.path, "ghijkl");
    }
    #[test]
    fn no_namespace() {
        let r = Identifier::new("azalea");
        assert_eq!(r.namespace, "minecraft");
        assert_eq!(r.path, "azalea");
    }
    #[test]
    fn colon_start() {
        let r = Identifier::new(":azalea");
        assert_eq!(r.namespace, "minecraft");
        assert_eq!(r.path, "azalea");
    }
    #[test]
    fn colon_end() {
        let r = Identifier::new("azalea:");
        assert_eq!(r.namespace, "azalea");
        assert_eq!(r.path, "");
    }

    #[test]
    fn azbuf_identifier() {
        let mut buf = Vec::new();
        Identifier::new("minecraft:dirt")
            .azalea_write(&mut buf)
            .unwrap();

        let mut buf = Cursor::new(&buf[..]);

        assert_eq!(
            Identifier::azalea_read(&mut buf).unwrap(),
            Identifier::new("minecraft:dirt")
        );
    }
}
