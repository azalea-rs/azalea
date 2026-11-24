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
#[derive(Clone, Eq)]
pub enum Identifier {
    Owned {
        // empty namespaces aren't allowed, so NonZero is fine
        colon_index: Option<NonZeroUsize>,
        inner: Box<str>,
    },
    Static(IdentifierRef<'static>),
}
const _: () = assert!(size_of::<Identifier>() == 24);
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct IdentifierRef<'a>(
    // this string is required to be include a namespace
    &'a str,
);

impl Default for Identifier {
    fn default() -> Self {
        Self::Static(Default::default())
    }
}
impl Default for IdentifierRef<'static> {
    fn default() -> Self {
        Self("minecraft:")
    }
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

        Self::Owned {
            colon_index,
            inner: resource_string.into(),
        }
    }
    pub fn new_from_parts(namespace: &str, path: &str) -> Self {
        Self::new(format!("{namespace}:{path}"))
    }
    pub const fn const_new(s: &'static str) -> Self {
        Self::Static(IdentifierRef::new(s))
    }

    pub fn namespace(&self) -> &str {
        match self {
            Self::Owned {
                colon_index: Some(colon_index),
                inner,
            } => &inner[..colon_index.get()],
            Self::Owned {
                colon_index: None, ..
            } => DEFAULT_NAMESPACE,
            Self::Static(s) => s.namespace(),
        }
    }
    pub fn path(&self) -> &str {
        match self {
            Self::Owned {
                colon_index: Some(colon_index),
                inner,
            } => &inner[(colon_index.get() + 1)..],
            Self::Owned {
                colon_index: None,
                inner,
            } => inner,
            Self::Static(s) => s.path(),
        }
    }

    /// Returns the identifier as it is stored, with a boolean indicating
    /// whether or not there's a namespace present.
    pub fn as_ref(&self) -> (bool, &str) {
        match self {
            Identifier::Owned { colon_index, inner } => (colon_index.is_some(), inner),
            Identifier::Static(identifier_ref) => (true, identifier_ref.0),
        }
    }
}
impl<'a> IdentifierRef<'a> {
    pub const fn new(s: &'a str) -> Self {
        let s = Self(s);
        // validate that it contains a colon
        s.colon_index();
        s
    }
    pub fn namespace(self) -> &'a str {
        str::from_utf8(&self.0.as_bytes()[..self.colon_index()]).unwrap()
    }
    pub fn path(self) -> &'a str {
        str::from_utf8(&self.0.as_bytes()[(self.colon_index() + 1)..]).unwrap()
    }
    pub const fn colon_index(self) -> usize {
        let bytes = self.0.as_bytes();
        assert!(bytes.is_ascii(), "static identifiers must be valid ascii");

        // namespace isn't allowed to be the first character
        let mut idx = 1;
        while idx < bytes.len() {
            if bytes[idx] == b':' {
                return idx;
            }
            idx += 1;
        }
        panic!("static identifiers must contain a namespace.");
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
        match self {
            Self::Owned {
                colon_index: None,
                inner,
            } => {
                write!(f, "{DEFAULT_NAMESPACE}:{inner}")
            }
            Self::Owned { inner, .. } => {
                write!(f, "{inner}")
            }
            Self::Static(s) => Display::fmt(s, f),
        }
    }
}
impl Display for IdentifierRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self.0, f)
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
    fn azalea_read(ctx: azalea_buf::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let location_string = String::azalea_read(ctx, buf)?;
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
            Identifier::azalea_read(Default::default(), &mut buf).unwrap(),
            Identifier::new("minecraft:dirt")
        );
    }
}
