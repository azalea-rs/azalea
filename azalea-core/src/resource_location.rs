//! A resource, like minecraft:stone

use std::{
    fmt::{self, Debug, Display},
    io::{self, Cursor, Write},
    num::NonZeroUsize,
    str::FromStr,
};

use azalea_buf::{AzaleaRead, AzaleaWrite, BufReadError};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use simdnbt::{FromNbtTag, ToNbtTag, owned::NbtTag};

#[derive(Hash, Clone, PartialEq, Eq, Default)]
pub struct ResourceLocation {
    // empty namespaces aren't allowed so NonZero is fine.
    colon_index: Option<NonZeroUsize>,
    inner: Box<str>,
}

static DEFAULT_NAMESPACE: &str = "minecraft";
// static REALMS_NAMESPACE: &str = "realms";

impl ResourceLocation {
    pub fn new(resource_string: impl Into<String>) -> ResourceLocation {
        let resource_string = resource_string.into();

        let colon_index = resource_string.find(':').and_then(|i| NonZeroUsize::new(i));
        ResourceLocation {
            colon_index,
            inner: resource_string.into(),
        }
    }

    pub fn namespace(&self) -> &str {
        if let Some(colon_index) = self.colon_index {
            &self.inner[0..colon_index.get()]
        } else {
            DEFAULT_NAMESPACE
        }
    }
    pub fn path(&self) -> &str {
        if let Some(colon_index) = self.colon_index {
            &self.inner[(colon_index.get() + 1)..]
        } else {
            &self.inner
        }
    }
}

impl Display for ResourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.colon_index.is_some() {
            write!(f, "{}", self.inner)
        } else {
            write!(f, "{DEFAULT_NAMESPACE}:{}", self.inner)
        }
    }
}
impl Debug for ResourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}
impl FromStr for ResourceLocation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ResourceLocation::new(s))
    }
}
impl From<&str> for ResourceLocation {
    fn from(s: &str) -> Self {
        ResourceLocation::new(s)
    }
}

impl AzaleaRead for ResourceLocation {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let location_string = String::azalea_read(buf)?;
        Ok(ResourceLocation::new(&location_string))
    }
}
impl AzaleaWrite for ResourceLocation {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        self.to_string().azalea_write(buf)
    }
}

impl Serialize for ResourceLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ResourceLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.contains(':') {
            Ok(ResourceLocation::new(&s))
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Str(&s),
                &"a valid ResourceLocation",
            ))
        }
    }
}

impl FromNbtTag for ResourceLocation {
    fn from_nbt_tag(tag: simdnbt::borrow::NbtTag) -> Option<Self> {
        tag.string().and_then(|s| s.to_str().parse().ok())
    }
}

impl ToNbtTag for ResourceLocation {
    fn to_nbt_tag(self) -> NbtTag {
        NbtTag::String(self.to_string().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_resource_location() {
        let r = ResourceLocation::new("abcdef:ghijkl");
        assert_eq!(r.namespace(), "abcdef");
        assert_eq!(r.path(), "ghijkl");
    }
    #[test]
    fn no_namespace() {
        let r = ResourceLocation::new("azalea");
        assert_eq!(r.namespace(), "minecraft");
        assert_eq!(r.path(), "azalea");
    }
    #[test]
    fn colon_start() {
        let r = ResourceLocation::new(":azalea");
        assert_eq!(r.namespace(), "minecraft");
        assert_eq!(r.path(), "azalea");
    }
    #[test]
    fn colon_end() {
        let r = ResourceLocation::new("azalea:");
        assert_eq!(r.namespace(), "azalea");
        assert_eq!(r.path(), "");
    }

    #[test]
    fn azbuf_resource_location() {
        let mut buf = Vec::new();
        ResourceLocation::new("minecraft:dirt")
            .azalea_write(&mut buf)
            .unwrap();

        let mut buf = Cursor::new(&buf[..]);

        assert_eq!(
            ResourceLocation::azalea_read(&mut buf).unwrap(),
            ResourceLocation::new("minecraft:dirt")
        );
    }
}
