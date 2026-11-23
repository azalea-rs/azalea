//! An arbitrary identifier or resource location.

use std::{
    fmt::{self, Debug, Display},
    hash::{Hash, Hasher},
    io::{self, Cursor, Write},
    num::NonZeroUsize,
    str::FromStr,
};

use azalea_buf::{AzBuf, BufReadError};
use azalea_codec::{Codec, error::ReadError};

/// An identifier, like `minecraft:stone` or `brigadier:number`.
///
/// All registry variants can be converted to an identifier.
///
/// This was formerly called a `ResourceLocation`.
#[doc(alias = "ResourceLocation")]
#[derive(Clone, Default, Eq)]
pub struct Identifier {
    // empty namespaces aren't allowed so NonZero is fine.
    colon_index: Option<NonZeroUsize>,
    inner: Box<str>,
}

const _: () = assert!(size_of::<Identifier>() == 24);

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
    pub fn new_from_parts(namespace: &str, path: &str) -> Self {
        Self::new(format!("{namespace}:{path}"))
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
impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.namespace() == other.namespace() && self.path() == other.path()
    }
}
impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let namespace = self.namespace();
        if namespace != DEFAULT_NAMESPACE {
            namespace.hash(state);
        }
        self.path().hash(state);
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

impl AzBuf for Identifier {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let location_string = String::azalea_read(buf)?;
        Ok(Identifier::new(&location_string))
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        self.to_string().azalea_write(buf)
    }
}

impl<O: azalea_codec::Ops> Codec<O> for Identifier {
    fn codec_read(ops: &O, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::ReadResult<Self> {
        ops.get_string(ctx)?
            .parse()
            .map_err(|_| ReadError::custom("invalid identifier"))
    }
    fn codec_write(&self, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::WriteResult<O> {
        self.to_string().codec_write(ctx)
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
