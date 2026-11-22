#![feature(associated_type_defaults)]
// needed for StringLiteral
#![allow(incomplete_features)]
#![feature(adt_const_params, unsized_const_params)]
// thiserror
#![feature(error_generic_member_access)]
// specialized lists
#![feature(min_specialization)]

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

#[diagnostic::on_unimplemented(message = "type `{Self}` does not implement `azalea_codec::Codec`.")]
pub trait Codec<O: Ops>: Sized {
    type Incomplete = ReadResult<Self>;

    /// Convert from a **data format** to a **data structure**.
    fn codec_read(ops: &O) -> ReadResult<Self>;
    /// Convert from a **data structure** to a **data format**.
    fn codec_write(&self, ctx: O::Context) -> WriteResult<O>;
}
/// Implement [`Codec`] for a type that's `Into<T>` and `From<T>` for another
/// type that's already a `Codec`.
///
/// If the type is a map, consider also using [`impl_mapcodec_from`].
///
/// ```
/// mod game_profile_codec {
///     use super::*;
///     azalea_codec::impl_codec_from!(GameProfile, SerializableGameProfile);
///     azalea_codec::impl_mapcodec_from!(GameProfile, SerializableGameProfile);
/// }
/// ```
#[macro_export]
macro_rules! impl_codec_from {
    ($this:ty, $other:ty) => {
        impl<O: azalea_codec::Ops> azalea_codec::Codec<O> for $this {
            type Incomplete = __Incomplete<O>;
            fn codec_read(ops: &O) -> azalea_codec::error::ReadResult<Self> {
                <$other>::codec_read(ops).map(Into::into)
            }
            fn codec_write(&self, ctx: O::Context) -> azalea_codec::error::WriteResult<O> {
                <$other>::from(self.clone()).codec_write(ctx)
            }
        }
        #[doc(hidden)]
        #[allow(nonstandard_style)]
        pub struct __Incomplete<O: azalea_codec::Ops>(
            pub <$other as azalea_codec::Codec<O>>::Incomplete,
        );
        impl<O: azalea_codec::Ops> azalea_codec::Incomplete for __Incomplete<O> {
            type Complete = $this;
            fn new(name: &'static str) -> Self {
                Self(<_>::new(name))
            }
            fn build(self) -> azalea_codec::error::ReadResult<Self::Complete> {
                self.0.build().map(Into::into)
            }
        }
    };
}
/// See [`impl_codec_from`].
#[macro_export]
macro_rules! impl_mapcodec_from {
    ($this:ty, $other:ty) => {
        impl<O: $crate::Ops> $crate::MapCodec<O> for $this {
            fn codec_read_map_item(
                incomplete: &mut <Self as $crate::Codec<O>>::Incomplete,
                k: &O,
                v: &O,
            ) {
                <$other>::codec_read_map_item(&mut incomplete.0, k, v);
            }
            fn codec_write_to_map(
                &self,
                ctx: O::Context,
                map: &mut Vec<(O, O)>,
            ) -> $crate::error::WriteResult<()> {
                <$other>::from(self.clone()).codec_write_to_map(ctx, map)
            }
            fn get_and_reset_result(
                incomplete: &mut <Self as $crate::Codec<O>>::Incomplete,
            ) -> $crate::error::ReadResult<()> {
                <$other>::get_and_reset_result(&mut incomplete.0)
            }
        }
    };
}

#[diagnostic::on_unimplemented(message = "type `{Self}` is not a map that can be flattened.")]
pub trait MapCodec<O: Ops>: Sized + Codec<O> {
    // this doesn't return a ReadResult because the errors are stored in the
    // Incomplete struct
    fn codec_read_map_item(incomplete: &mut <Self as Codec<O>>::Incomplete, k: &O, v: &O);
    fn codec_write_to_map(&self, ctx: O::Context, map: &mut Vec<(O, O)>) -> WriteResult<()>;

    /// For normal structs, this sets `incomplete.__result = Ok(())` and returns
    /// the previous value of `__result`.
    fn get_and_reset_result(incomplete: &mut <Self as Codec<O>>::Incomplete) -> ReadResult<()>;
}

// https://github.com/Mojang/DataFixerUpper/blob/master/src/main/java/com/mojang/serialization/DynamicOps.java
#[diagnostic::on_unimplemented(
    message = "type `{Self}` is not a data format; did you mean to use `{Self}::codec_read` instead?"
)]
#[allow(unused_variables)]
pub trait Ops: Sized + Clone + Codec<Self> {
    type List: IntoIterator<Item = Self>;
    type Map: MapLike<Self>;

    type Context: Clone + Copy = ();

    fn empty(ctx: Self::Context) -> WriteResult<Self> {
        Self::empty_map(ctx)
    }
    fn empty_map(ctx: Self::Context) -> WriteResult<Self> {
        Self::create_map(ctx, &[])
    }
    fn empty_list(ctx: Self::Context) -> WriteResult<Self> {
        Self::create_list(ctx, &[])
    }

    fn get_number(&self) -> ReadResult<Number> {
        Err(ReadError::unimplemented())
    }
    fn create_number(ctx: Self::Context, n: Number) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_boolean(&self) -> ReadResult<bool> {
        Err(ReadError::unimplemented())
    }
    fn create_boolean(ctx: Self::Context, v: bool) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_list(&self) -> ReadResult<Self::List> {
        Err(ReadError::unimplemented())
    }
    fn create_list(ctx: Self::Context, list: &[Self]) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_map(&self) -> ReadResult<Self::Map> {
        Err(ReadError::unimplemented())
    }
    fn create_map(ctx: Self::Context, map: &[(Self, Self)]) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_string(&self) -> ReadResult<String> {
        Err(ReadError::unimplemented())
    }
    fn create_string(ctx: Self::Context, v: &str) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_byte_buffer(&self) -> ReadResult<Box<[u8]>> {
        fallback_get_list(self)
    }
    fn create_byte_list(ctx: Self::Context, v: &[u8]) -> WriteResult<Self> {
        fallback_create_list(ctx, v)
    }

    fn get_int_list(&self) -> ReadResult<Box<[i32]>> {
        fallback_get_list(self)
    }
    fn create_int_list(ctx: Self::Context, v: &[i32]) -> WriteResult<Self> {
        fallback_create_list(ctx, v)
    }

    fn get_long_list(&self) -> ReadResult<Box<[i64]>> {
        fallback_get_list(self)
    }
    fn create_long_list(ctx: Self::Context, v: &[i64]) -> WriteResult<Self> {
        fallback_create_list(ctx, v)
    }

    fn get_resolvable_item<I: ResolvableItem>(&self) -> ReadResult<I> {
        Err(ReadError::unimplemented())
    }
    fn create_resolvable_item<I: ResolvableItem>(ctx: Self::Context, _i: I) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }
}

fn fallback_get_list<O: Ops, N: TryFrom<Number, Error = ReadError>>(o: &O) -> ReadResult<Box<[N]>> {
    o.get_list()?
        .into_iter()
        .map(|o| o.get_number().and_then(|n| n.try_into()))
        .collect()
}
fn fallback_create_list<O: Ops, N: Into<Number> + Copy>(
    ctx: O::Context,
    v: &[N],
) -> WriteResult<O> {
    O::create_list(
        ctx,
        &v.iter()
            .map(|&i| O::create_number(ctx, i.into()))
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

pub struct NoMap;
impl<T: Clone> MapLike<T> for NoMap {
    fn for_each(&self, _: impl FnMut(&T, &T)) {
        unimplemented!()
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

pub trait ResolvableItem {
    const NAME: &'static str;
    fn to_resolvable_id(&self) -> u32;
    fn from_resolvable_id(id: u32) -> Self;
}
