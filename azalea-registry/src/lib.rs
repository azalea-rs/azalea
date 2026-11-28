#![doc = include_str!("../README.md")]
// used by default_for_item in components
#![feature(min_specialization)]
// used by the `registry!` macro
#![feature(const_trait_impl, const_cmp)]

pub use azalea_registry_builtin::*;
pub mod data;
pub mod data_registry;
pub mod item;
pub mod registry_holder;
pub mod tags;

use std::{
    fmt::{self, Debug},
    hash::Hash,
    io::{self, Cursor, Write},
};

use azalea_buf::{AzBuf, AzBufVar, BufReadError, BufReadErrorRepr};
use azalea_codec::Codec;
use azalea_core::identifier::Identifier;
pub use azalea_registry_macros as macros;

pub trait Registry:
    AzBuf + PartialEq + PartialOrd + Ord + Clone + Hash + Into<Identifier> + TryFrom<Identifier> + Clone
where
    Self: Sized,
{
    const NAME: &'static str;

    fn from_id(ctx: azalea_buf::Ctx, id: u32) -> Result<Self, BufReadError>;
    fn to_id(&self, ctx: azalea_buf::Ctx) -> io::Result<u32>;
}

impl<T: BuiltinRegistry> Registry for T {
    const NAME: &'static str = T::NAME;
    fn from_id(_ctx: azalea_buf::Ctx, id: u32) -> Result<Self, BufReadError> {
        T::builtin_from_id(id)
            .ok_or_else(|| BufReadErrorRepr::UnexpectedEnumVariant { id: id as i32 }.into())
    }
    fn to_id(&self, _ctx: azalea_buf::Ctx) -> io::Result<u32> {
        Ok(self.builtin_into_id())
    }
}

/// A registry that might not be present.
///
/// This is transmitted as a single varint in the protocol.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OptionalRegistry<T: Registry>(pub Option<T>);

impl<T: Registry> AzBuf for OptionalRegistry<T> {
    fn azalea_read(ctx: azalea_buf::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(OptionalRegistry(match u32::azalea_read_var(ctx, buf)? {
            0 => None,
            value => Some(T::from_id(ctx, value - 1)?),
        }))
    }
    fn azalea_write(&self, ctx: azalea_buf::Ctx, buf: &mut impl Write) -> io::Result<()> {
        match &self.0 {
            None => 0u32.azalea_write_var(ctx, buf),
            Some(value) => (value.to_id(ctx)? + 1).azalea_write_var(ctx, buf),
        }
    }
}

/// A registry that will either take an ID or a resource location.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CustomRegistry<D: Registry, C: AzBuf> {
    Direct(D),
    Custom(C),
}

impl<D: Registry, C: AzBuf> AzBuf for CustomRegistry<D, C> {
    fn azalea_read(ctx: azalea_buf::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let direct_registry = OptionalRegistry::<D>::azalea_read(ctx, buf)?;
        if let Some(direct_registry) = direct_registry.0 {
            return Ok(CustomRegistry::Direct(direct_registry));
        }
        Ok(CustomRegistry::Custom(C::azalea_read(ctx, buf)?))
    }
    fn azalea_write(&self, ctx: azalea_buf::Ctx, buf: &mut impl Write) -> io::Result<()> {
        match self {
            CustomRegistry::Direct(direct_registry) => {
                // write the id + 1
                (direct_registry.to_id(ctx)? + 1).azalea_write_var(ctx, buf)
            }
            CustomRegistry::Custom(custom_registry) => {
                // write 0, then the custom registry
                0u32.azalea_write_var(ctx, buf)?;
                custom_registry.azalea_write(ctx, buf)
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum HolderSet<D> {
    Direct {
        contents: Vec<D>,
    },
    Named {
        key: Identifier,
        contents: Vec<Identifier>,
    },
}
impl<D: AzBuf> AzBuf for HolderSet<D> {
    fn azalea_read(ctx: azalea_buf::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let size = i32::azalea_read_var(ctx, buf)?.wrapping_sub(1);
        if size == -1 {
            let key = Identifier::azalea_read(ctx, buf)?;
            Ok(Self::Named {
                key,
                contents: Vec::new(),
            })
        } else {
            let mut contents = Vec::new();
            for _ in 0..size {
                contents.push(D::azalea_read(ctx, buf)?);
            }
            Ok(Self::Direct { contents })
        }
    }
    fn azalea_write(&self, ctx: azalea_buf::Ctx, buf: &mut impl Write) -> io::Result<()> {
        match self {
            Self::Direct { contents } => {
                (contents.len() as i32 + 1).azalea_write_var(ctx, buf)?;
                for item in contents {
                    item.azalea_write(ctx, buf)?;
                }
            }
            Self::Named { key, contents: _ } => {
                0i32.azalea_write_var(ctx, buf)?;
                key.azalea_write(ctx, buf)?;
            }
        }
        Ok(())
    }
}
impl<D: Debug> Debug for HolderSet<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Direct { contents } => f.debug_list().entries(contents).finish(),
            Self::Named { key, contents } => f
                .debug_struct("Named")
                .field("key", key)
                .field("contents", contents)
                .finish(),
        }
    }
}
impl<D> From<Vec<D>> for HolderSet<D> {
    fn from(contents: Vec<D>) -> Self {
        Self::Direct { contents }
    }
}
impl<D: Codec<O>, O: azalea_codec::Ops> Codec<O> for HolderSet<D> {
    fn codec_read(ops: &O) -> azalea_codec::error::ReadResult<Self> {
        if let Ok(key) = Identifier::codec_read(ops) {
            return Ok(Self::Named {
                key,
                contents: vec![],
            });
        }
        if let Ok(d) = D::codec_read(ops) {
            return Ok(Self::Direct { contents: vec![d] });
        }
        if let Ok(contents) = Vec::<D>::codec_read(ops) {
            return Ok(Self::Direct { contents });
        }

        Err(azalea_codec::error::ReadError::custom("invalid holderset"))
    }
    fn codec_write(&self) -> azalea_codec::error::WriteResult<O> {
        match self {
            Self::Direct { contents } => {
                if contents.len() == 1 {
                    contents[0].codec_write()
                } else {
                    contents.codec_write()
                }
            }
            Self::Named { key, contents: _ } => key.codec_write(),
        }
    }
}

impl<D: Registry> Default for HolderSet<D> {
    fn default() -> Self {
        Self::Direct {
            contents: Vec::new(),
        }
    }
}

impl<O: azalea_codec::Ops, D: Registry + Codec<O>> azalea_codec::Defaultable<O> for HolderSet<D> {
    fn is_default(&self) -> bool {
        match self {
            Self::Direct { contents } => contents.is_empty(),
            _ => false,
        }
    }
}

impl<D: Registry> HolderSet<D>
where
    Identifier: From<D>,
{
    pub fn contains(&self, value: D) -> bool {
        match self {
            HolderSet::Direct { contents } => contents.contains(&value),
            HolderSet::Named { key: _, contents } => contents.contains(&Identifier::from(value)),
        }
    }
}

/// A reference to either a registry or a custom value (usually something with
/// an `Identifier`).
#[derive(Codec)]
pub enum Holder<R: Registry, Direct: AzBuf> {
    Reference(R),
    Direct(Direct),
}
impl<R: Registry, Direct: AzBuf> AzBuf for Holder<R, Direct> {
    fn azalea_read(ctx: azalea_buf::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = u32::azalea_read_var(ctx, buf)?;
        if id == 0 {
            Ok(Self::Direct(Direct::azalea_read(ctx, buf)?))
        } else {
            let id = id - 1;
            let value = Registry::from_id(ctx, id)?;
            Ok(Self::Reference(value))
        }
    }
    fn azalea_write(&self, ctx: azalea_buf::Ctx, buf: &mut impl Write) -> io::Result<()> {
        match self {
            Self::Reference(value) => (value.to_id(ctx)? + 1).azalea_write_var(ctx, buf),
            Self::Direct(value) => {
                0u32.azalea_write_var(ctx, buf)?;
                value.azalea_write(ctx, buf)
            }
        }
    }
}
impl<R: Registry + Debug, Direct: AzBuf + Debug> Debug for Holder<R, Direct> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reference(value) => f.debug_tuple("Reference").field(value).finish(),
            Self::Direct(value) => f.debug_tuple("Direct").field(value).finish(),
        }
    }
}
impl<R: Registry + Clone, Direct: AzBuf + Clone> Clone for Holder<R, Direct> {
    fn clone(&self) -> Self {
        match self {
            Self::Reference(value) => Self::Reference(value.clone()),
            Self::Direct(value) => Self::Direct(value.clone()),
        }
    }
}
impl<R: Registry + PartialEq, Direct: AzBuf + PartialEq> PartialEq for Holder<R, Direct> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Reference(a), Self::Reference(b)) => a == b,
            (Self::Direct(a), Self::Direct(b)) => a == b,
            _ => false,
        }
    }
}
impl<R: Registry + Default, Direct: AzBuf> Default for Holder<R, Direct> {
    fn default() -> Self {
        Self::Reference(R::default())
    }
}
