//! An arbitrary identifier or resource location.

use std::{
    fmt::{self, Debug, Display},
    io::{self, Cursor, Write},
    num::NonZeroUsize,
    str::FromStr,
};

use azalea_buf::{AzaleaRead, AzaleaWrite, BufReadError};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use simdnbt::{FromNbtTag, ToNbtTag, owned::NbtTag};

/// An identifier, like `minecraft:stone` or `brigadier:number`.
///
/// All registry variants can be converted to an identifier.
///
/// This was formerly called a `ResourceLocation`.
#[doc(alias = "ResourceLocation")]
#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct Identifier {
    // empty namespaces aren't allowed so NonZero is fine.
    colon_index: Option<NonZeroUsize>,
    inner: Box<str>,
}

static DEFAULT_NAMESPACE: &str = "minecraft";
// static REALMS_NAMESPACE: &str = "realms";

impl Identifier {
    pub fn new(resource_string: impl Into<String>) -> Identifier {
        let mut resource_string = resource_string.into();

        let colon_index = resource_string.find(':');
        let colon_index = if let Some(colon_index) = colon_index {
            if colon_index == 0 {
                resource_string = resource_string.split_off(1);
            }
            NonZeroUsize::new(colon_index)
        } else {
            None
        };

        Self {
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

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.colon_index.is_some() {
            write!(f, "{}", self.inner)
        } else {
            write!(f, "{DEFAULT_NAMESPACE}:{}", self.inner)
        }
    }
}
impl Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
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

#[cfg(feature = "serde")]
impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
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
        assert_eq!(r.namespace(), "abcdef");
        assert_eq!(r.path(), "ghijkl");
    }
    #[test]
    fn no_namespace() {
        let r = Identifier::new("azalea");
        assert_eq!(r.namespace(), "minecraft");
        assert_eq!(r.path(), "azalea");
    }
    #[test]
    fn colon_start() {
        let r = Identifier::new(":azalea");
        assert_eq!(r.namespace(), "minecraft");
        assert_eq!(r.path(), "azalea");
    }
    #[test]
    fn colon_end() {
        let r = Identifier::new("azalea:");
        assert_eq!(r.namespace(), "azalea");
        assert_eq!(r.path(), "");
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
