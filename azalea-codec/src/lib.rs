#![doc = include_str!("../README.md")]
//! ## Example
//!
//! ```rust
#![doc = include_str!("../examples/showcase.rs")]
//! ```

// used for Codec::Incomplete
#![feature(associated_type_defaults)]
// needed for StringLiteral
#![allow(incomplete_features)]
#![feature(adt_const_params, unsized_const_params)]
// thiserror
#![feature(error_generic_member_access)]
// specialized lists
#![feature(min_specialization)]
// ConstFromStr, used for compile-time validation of `#[codec(tag_as = ...)]` tags
#![feature(const_trait_impl)]

pub use azalea_codec_macros::Codec;

use crate::{
    error::{ReadError, ReadResult, WriteError, WriteResult},
    number::Number,
};

pub mod error;
mod integrations;
pub mod number;
pub mod rust;
pub mod types;

// for macros
#[cfg(feature = "serde_json")]
#[doc(hidden)]
pub use serde;
#[cfg(feature = "serde_json")]
#[doc(hidden)]
pub use serde_json;
#[cfg(feature = "simd-json")]
#[doc(hidden)]
pub use simd_json;

/// A data structure that can be written and read.
///
/// When possible, you should prefer to use the
/// [`Codec`](azalea_codec_macros::Codec) derive macro.
#[diagnostic::on_unimplemented(message = "type `{Self}` does not implement `azalea_codec::Codec`.")]
pub trait Codec<O: Ops>: Sized {
    type Incomplete = ReadResult<Self>;

    /// Convert from a **data format** to a **data structure**.
    fn codec_read(ops: &O) -> ReadResult<Self>;
    // ^ this takes in a reference for user ergonomics and performance. many
    // implementations will be calling read on the same object many times (to
    // try different ways of deserializing it), and being required to clone the
    // entire thing every time would be a shame.

    /// Convert from a **data structure** to a **data format**.
    fn codec_write(&self) -> WriteResult<O>;
}
/// Implement [`Codec`] for a type that's `Into<T>` and `From<T>` for another
/// type that's already a `Codec`.
///
/// If the type is a map, consider also using [`impl_mapcodec_from`].
///
/// ```ignore
/// azalea_codec::impl_codec_from!(GameProfile, SerializableGameProfile, struct IncompleteGameProfile);
/// azalea_codec::impl_mapcodec_from!(GameProfile, SerializableGameProfile);
/// ```
#[macro_export]
macro_rules! impl_codec_from {
    ($(<$($generic:ident$(: $constraint:ident $(+ $constraint2:ident)*  )?),*>)? $this:ident $(<$($this_generic:ident),+>)?, $other:ty, struct $incomplete:ident) => {
        impl<O: $crate::Ops $($(, $generic: $crate::Codec<O> $(+ $constraint $(+$constraint2)*)*)*)?> $crate::Codec<O> for $this $(<$($this_generic),*>)? {
            type Incomplete = $incomplete<O $($(, $this_generic)*)?>;
            fn codec_read(ops: &O) -> $crate::error::ReadResult<Self> {
                <$other>::codec_read(ops).map(Into::into)
            }
            fn codec_write(&self) -> $crate::error::WriteResult<O> {
                <$other>::from(self.clone()).codec_write()
            }
        }
        #[doc(hidden)]
        #[allow(nonstandard_style)]
        pub struct $incomplete<O: $crate::Ops $($(, $generic: $crate::Codec<O> $(+ $constraint $(+$constraint2)*)*)*)?>(
            pub <$other as $crate::Codec<O>>::Incomplete
        );
        impl<O: $crate::Ops $($(, $generic: $crate::Codec<O> $(+ $constraint $(+$constraint2)*)*)*)?> $crate::Incomplete for $incomplete<O $($(, $this_generic)*)?> {
            type Complete = $this $(<$($this_generic),*>)?;
            fn new(name: &'static str) -> Self {
                Self(<_>::new(name))
            }
            fn build(self) -> $crate::error::ReadResult<Self::Complete> {
                self.0.build().map(Into::into)
            }
        }

        impl<O: $crate::Ops, $($($generic: Codec<O> $(+ $constraint $(+ $constraint2)*)?),*)?> $crate::NewOk<O> for $incomplete<O $($(, $this_generic)*)?>
            where Self: $crate::Incomplete,
            <$other as $crate::Codec<O>>::Incomplete: $crate::NewOk<O>,
        {
            fn new_ok() -> Self {
                Self($crate::NewOk::new_ok())
            }
        }
    };
}
/// See [`impl_codec_from`].
#[macro_export]
macro_rules! impl_mapcodec_from {
    ($(<$($generic:ident$(: $constraint:ident $(+ $constraint2:ident)*  )?),*>)? $this:ident $(<$($this_generic:ident),+>)?, $other:ty) => {
        impl<O: $crate::Ops $($(, $generic: $crate::Codec<O> $(+ $constraint $(+$constraint2)*)*)*)?> $crate::MapCodec<O> for $this $(<$($this_generic),*>)? {
            fn codec_read_map_item(
                incomplete: &mut <Self as $crate::Codec<O>>::Incomplete,

                k: &O,
                v: &O,
            ) {
                <$other>::codec_read_map_item(&mut incomplete.0, k, v);
            }
            fn codec_write_to_map(
                &self,

                map: &mut Vec<(O, O)>,
            ) -> $crate::error::WriteResult<()> {
                <$other>::from(self.clone()).codec_write_to_map( map)
            }
            fn get_and_reset_result(
                incomplete: &mut <Self as $crate::Codec<O>>::Incomplete,
            ) -> $crate::error::ReadResult<()> {
                <$other>::get_and_reset_result(&mut incomplete.0)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mapcodec {
    ($t:ty, $incomplete:ident $(<$($generic:ident$(: $constraint:ident $(+ $constraint2:ident)*  )?),*>)?, $k:ty : $v:ty) => {
        pub struct $incomplete $(<$($generic),*>)? {
            result: $crate::error::ReadResult<()>,
            map: $t,
        }
        impl<O: Ops, $($($generic: Codec<O> + Clone $(+ $constraint $(+ $constraint2)*)?),*)?> $crate::MapCodec<O> for $t {
            fn codec_read_map_item(incomplete: &mut <Self as Codec<O>>::Incomplete, k: &O, v: &O) {
                let k = match <$k>::codec_read(k) {
                    Ok(r) => r,
                    Err(e) => {
                        incomplete.result = Err(e);
                        return;
                    }
                };
                let v = match <$v>::codec_read(v) {
                    Ok(r) => r,
                    Err(e) => {
                        incomplete.result = Err(e);
                        return;
                    }
                };
                incomplete.map.insert(k, v);
            }
            fn codec_write_to_map(
                &self,

                map: &mut Vec<(O, O)>,
            ) -> $crate::error::WriteResult<()> {
                for (k, v) in self.iter() {
                    map.push((k.to_owned().codec_write()?, v.codec_write()?))
                }
                Ok(())
            }
            fn get_and_reset_result(
                incomplete: &mut <Self as Codec<O>>::Incomplete,
            ) -> $crate::error::ReadResult<()> {
                let mut result = Ok(());
                ::std::mem::swap(&mut incomplete.result, &mut result);
                result
            }
        }
        impl<$($($generic $(: $constraint $(+ $constraint2)*)?),*)?> Incomplete for $incomplete $(<$($generic),*>)? {
            type Complete = $t;
            fn new(name: &'static str) -> Self {
                Self {
                    result: Err(ReadError::missing_key(name)),
                    map: Default::default(),
                }
            }
            fn build(self) -> ReadResult<Self::Complete> {
                self.result.map(|_| self.map)
            }
        }
        impl<O: Ops, $($($generic: Codec<O> $(+ $constraint $(+ $constraint2)*)?),*)?> NewOk<O> for $incomplete $(<$($generic),*>)?
            where Self: Incomplete
        {
            fn new_ok() -> Self {
                Self {
                    result: Ok(()),
                    map: Default::default(),
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! internal_impl_serde_with {
    ($t:ident, $value_ty:ty) => {
        impl $crate::serde::Serialize for $t {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer,
            {
                let json: $value_ty = self
                    .codec_write()
                    .map_err($crate::serde::ser::Error::custom)?;
                json.serialize(serializer)
            }
        }
        impl<'de> $crate::serde::Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $crate::serde::Deserializer<'de>,
            {
                let json = <$value_ty>::deserialize(deserializer)?;
                Self::codec_read(&json).map_err($crate::serde::de::Error::custom)
            }
        }
    };
}

#[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
#[macro_export]
macro_rules! impl_serde {
    ($t:ident) => {
        azalea_codec::internal_impl_serde_with!($t, $crate::serde_json::Value);
    };
}
#[cfg(feature = "simd-json")]
#[macro_export]
macro_rules! impl_serde {
    ($t:ident) => {
        azalea_codec::internal_impl_serde_with!($t, $crate::simd_json::owned::Value);
    };
}

#[diagnostic::on_unimplemented(message = "type `{Self}` is not a map that can be flattened.")]
pub trait MapCodec<O: Ops>: Sized + Codec<O> {
    // this doesn't return a ReadResult because the errors are stored in the
    // Incomplete struct
    fn codec_read_map_item(incomplete: &mut <Self as Codec<O>>::Incomplete, k: &O, v: &O);
    fn codec_write_to_map(&self, map: &mut Vec<(O, O)>) -> WriteResult<()>;

    /// For normal structs, this sets `incomplete.__result = Ok(())` and returns
    /// the previous value of `__result`.
    fn get_and_reset_result(incomplete: &mut <Self as Codec<O>>::Incomplete) -> ReadResult<()>;
}

/// A data serialization format.
#[diagnostic::on_unimplemented(
    message = "type `{Self}` is not a data format; did you mean to use `{Self}::codec_read` instead?"
)]
#[allow(unused_variables)]
pub trait Ops: Sized + Clone + Codec<Self> + 'static {
    // https://github.com/Mojang/DataFixerUpper/blob/master/src/main/java/com/mojang/serialization/DynamicOps.java

    type List: IntoIterator<Item = Self>;
    type Map: MapLike<Self>;

    fn empty() -> WriteResult<Self> {
        Self::empty_map()
    }
    fn empty_map() -> WriteResult<Self> {
        Self::create_map(&[])
    }
    fn empty_list() -> WriteResult<Self> {
        Self::create_list(&[])
    }

    fn get_number(&self) -> ReadResult<Number> {
        Err(ReadError::unimplemented())
    }
    fn create_number(n: Number) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_boolean(&self) -> ReadResult<bool> {
        Err(ReadError::unimplemented())
    }
    fn create_boolean(v: bool) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_list(&self) -> ReadResult<Self::List> {
        Err(ReadError::unimplemented())
    }
    fn create_list(list: &[Self]) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_map(&self) -> ReadResult<Self::Map> {
        Err(ReadError::unimplemented())
    }
    fn create_map(map: &[(Self, Self)]) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_string(&self) -> ReadResult<String> {
        Err(ReadError::unimplemented())
    }
    fn create_string(v: &str) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_byte_buffer(&self) -> ReadResult<Box<[u8]>> {
        fallback_get_list(self)
    }
    fn create_byte_list(v: &[u8]) -> WriteResult<Self> {
        fallback_create_list(v)
    }

    fn get_int_list(&self) -> ReadResult<Box<[i32]>> {
        fallback_get_list(self)
    }
    fn create_int_list(v: &[i32]) -> WriteResult<Self> {
        fallback_create_list(v)
    }

    fn get_long_list(&self) -> ReadResult<Box<[i64]>> {
        fallback_get_list(self)
    }
    fn create_long_list(v: &[i64]) -> WriteResult<Self> {
        fallback_create_list(v)
    }
}

fn fallback_get_list<O: Ops, N: TryFrom<Number, Error = ReadError>>(o: &O) -> ReadResult<Box<[N]>> {
    o.get_list()?
        .into_iter()
        .map(|o| o.get_number().and_then(|n| n.try_into()))
        .collect()
}
fn fallback_create_list<O: Ops, N: Into<Number> + Copy>(v: &[N]) -> WriteResult<O> {
    O::create_list(
        &v.iter()
            .map(|&i| O::create_number(i.into()))
            .collect::<WriteResult<Box<[O]>>>()?,
    )
}

pub trait MapLike<T: Clone>: Sized {
    fn for_each(&self, cb: impl FnMut(&T, &T));

    fn to_values(&self) -> Box<[(T, T)]> {
        let mut values: Vec<(T, T)> = Vec::new();
        self.for_each(|k, v| {
            values.push((k.clone(), v.clone()));
        });
        values.into_boxed_slice()
    }
}

pub trait Incomplete: Sized {
    type Complete;

    fn new(name: &'static str) -> Self;
    fn new_unnamed() -> Self {
        Self::new("")
    }

    fn build(self) -> ReadResult<Self::Complete>;
}
impl<T> Incomplete for ReadResult<T> {
    type Complete = T;

    fn new(name: &'static str) -> Self {
        Err(ReadError::missing_key(name))
    }
    fn build(self) -> ReadResult<T> {
        self
    }
}

pub trait NewOk<T>: Incomplete {
    fn new_ok() -> Self;
}
impl<T: Default> NewOk<T> for ReadResult<T> {
    fn new_ok() -> ReadResult<T> {
        Ok(T::default())
    }
}

/// A trait that represents a subset of types with `Default`.
///
/// This is required for a `#[codec(optional)]` to work on a type. Also see
/// [`impl_defaultable`].
#[diagnostic::on_unimplemented(
    message = "type `{Self}` does not support `#[codec(optional)]`.",
    note = "consider using `azalea_codec::impl_defaultable!({Self})`."
)]
pub trait Defaultable<O: Ops>: Codec<O> {
    fn is_default(&self) -> bool;
}
/// Automatically implements [`Defaultable`] on the given type, allowing it to
/// be supported by `#[codec(optional)]` fields.
#[macro_export]
macro_rules! impl_defaultable {
    (<$($generic_name:ident$(: $generic_constraint:path)?)*> for $t:ty, $self:ident, $($check:tt)*) => {
        impl<O: $crate::Ops, $($generic_name$(: $generic_constraint)?)*> $crate::Defaultable<O> for $t {
            fn is_default(&$self) -> bool {
                $($check)*
            }
        }
    };

    ($t:ty) => {
        $crate::impl_defaultable!($t, self, self == &Self::default());
    };
    ($t:ty, $self:ident, $($check:tt)*) => {
        impl<O: $crate::Ops> $crate::Defaultable<O> for $t {
            fn is_default(&$self) -> bool {
                $($check)*
            }
        }
    };
}

pub trait ResolvableItem<O: Ops> {
    fn name(&self) -> &'static str;
    fn to_resolvable_id(&self) -> u32;
}
pub trait IntoResolvableItem<O: Ops>: ResolvableItem<O> + 'static {
    type Resolvable: Codec<O> + ResolvableItem<O>;
    fn codec_read_resolvable_item(ops: &O) -> ReadResult<Self::Resolvable>;
    fn from_resolvable_id(id: u32) -> Self;
}

// this trait exists because often Rust won't let us impl `From<T> + Into<T>`
// for certain wrapper types.
pub trait Interchangeable<T> {
    fn interchangeable_into(self) -> T;
    fn interchangeable_from(other: T) -> Self;
}
impl<A, B> Interchangeable<A> for B
where
    A: From<B>,
    B: From<A>,
{
    fn interchangeable_into(self) -> A {
        self.into()
    }
    fn interchangeable_from(other: A) -> Self {
        other.into()
    }
}
