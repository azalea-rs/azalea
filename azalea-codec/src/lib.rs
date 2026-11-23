#![feature(associated_type_defaults)]
// needed for StringLiteral
#![allow(incomplete_features)]
#![feature(adt_const_params, unsized_const_params)]
// thiserror
#![feature(error_generic_member_access)]
// specialized lists
#![feature(min_specialization)]

use std::any::{Any, TypeId};

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
    fn codec_read(ops: &O, ctx: Ctx<O>) -> ReadResult<Self>;
    // ^ this takes in a reference for user ergonomics and performance. many
    // implementations will be calling read on the same object many times (to
    // try different ways of deserializing it), and being required to clone the
    // entire thing every time would be a shame.

    /// Convert from a **data structure** to a **data format**.
    fn codec_write(&self, ctx: Ctx<O>) -> WriteResult<O>;
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
        impl<O: $crate::Ops> $crate::Codec<O> for $this {
            type Incomplete = __Incomplete<O>;
            fn codec_read(ops: &O, ctx: $crate::Ctx<O>) -> $crate::error::ReadResult<Self> {
                <$other>::codec_read(ops, ctx).map(Into::into)
            }
            fn codec_write(&self, ctx: $crate::Ctx<O>) -> $crate::error::WriteResult<O> {
                <$other>::from(self.clone()).codec_write(ctx)
            }
        }
        #[doc(hidden)]
        #[allow(nonstandard_style)]
        pub struct __Incomplete<O: $crate::Ops>(pub <$other as $crate::Codec<O>>::Incomplete);
        impl<O: $crate::Ops> $crate::Incomplete for __Incomplete<O> {
            type Complete = $this;
            fn new(name: &'static str) -> Self {
                Self(<_>::new(name))
            }
            fn build(self) -> $crate::error::ReadResult<Self::Complete> {
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
                ctx: $crate::Ctx<O>,
                k: &O,
                v: &O,
            ) {
                <$other>::codec_read_map_item(&mut incomplete.0, ctx, k, v);
            }
            fn codec_write_to_map(
                &self,
                ctx: $crate::Ctx<O>,
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

#[macro_export]
macro_rules! impl_mapcodec {
    ($t:ty, $incomplete:ident $(<$($generic:ident$(: $constraint:ident $(+ $constraint2:ident)*  )?),*>)?, $k:ty : $v:ty) => {
        pub struct $incomplete $(<$($generic),*>)? {
            result: $crate::error::ReadResult<()>,
            map: $t,
        }
        impl<O: Ops, $($($generic: Codec<O> $(+ $constraint $(+ $constraint2)*)?),*)?> $crate::MapCodec<O> for $t {
            fn codec_read_map_item(incomplete: &mut <Self as Codec<O>>::Incomplete, ctx: $crate::Ctx<O>, k: &O, v: &O) {
                let k = match <$k>::codec_read(k, ctx) {
                    Ok(r) => r,
                    Err(e) => {
                        incomplete.result = Err(e);
                        return;
                    }
                };
                let v = match <$v>::codec_read(v, ctx) {
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
                ctx: $crate::Ctx<O>,
                map: &mut Vec<(O, O)>,
            ) -> $crate::error::WriteResult<()> {
                for (k, v) in self.iter() {
                    map.push((k.to_owned().codec_write(ctx.clone())?, v.codec_write(ctx.clone())?))
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

#[diagnostic::on_unimplemented(message = "type `{Self}` is not a map that can be flattened.")]
pub trait MapCodec<O: Ops>: Sized + Codec<O> {
    // this doesn't return a ReadResult because the errors are stored in the
    // Incomplete struct
    fn codec_read_map_item(
        incomplete: &mut <Self as Codec<O>>::Incomplete,
        ctx: Ctx<O>,
        k: &O,
        v: &O,
    );
    fn codec_write_to_map(&self, ctx: Ctx<O>, map: &mut Vec<(O, O)>) -> WriteResult<()>;

    /// For normal structs, this sets `incomplete.__result = Ok(())` and returns
    /// the previous value of `__result`.
    fn get_and_reset_result(incomplete: &mut <Self as Codec<O>>::Incomplete) -> ReadResult<()>;
}

// https://github.com/Mojang/DataFixerUpper/blob/master/src/main/java/com/mojang/serialization/DynamicOps.java
#[diagnostic::on_unimplemented(
    message = "type `{Self}` is not a data format; did you mean to use `{Self}::codec_read` instead?"
)]
#[allow(unused_variables)]
pub trait Ops: Sized + Clone + Codec<Self> + 'static {
    type List: IntoIterator<Item = Self>;
    type Map: MapLike<Self>;

    fn empty(ctx: Ctx<Self>) -> WriteResult<Self> {
        Self::empty_map(ctx)
    }
    fn empty_map(ctx: Ctx<Self>) -> WriteResult<Self> {
        Self::create_map(ctx, &[])
    }
    fn empty_list(ctx: Ctx<Self>) -> WriteResult<Self> {
        Self::create_list(ctx, &[])
    }

    fn get_number(&self, ctx: Ctx<Self>) -> ReadResult<Number> {
        Err(ReadError::unimplemented())
    }
    fn create_number(ctx: Ctx<Self>, n: Number) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_boolean(&self, ctx: Ctx<Self>) -> ReadResult<bool> {
        Err(ReadError::unimplemented())
    }
    fn create_boolean(ctx: Ctx<Self>, v: bool) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_list(&self, ctx: Ctx<Self>) -> ReadResult<Self::List> {
        Err(ReadError::unimplemented())
    }
    fn create_list(ctx: Ctx<Self>, list: &[Self]) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_map(&self, ctx: Ctx<Self>) -> ReadResult<Self::Map> {
        Err(ReadError::unimplemented())
    }
    fn create_map(ctx: Ctx<Self>, map: &[(Self, Self)]) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_string(&self, ctx: Ctx<Self>) -> ReadResult<String> {
        Err(ReadError::unimplemented())
    }
    fn create_string(ctx: Ctx<Self>, v: &str) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_byte_buffer(&self, ctx: Ctx<Self>) -> ReadResult<Box<[u8]>> {
        fallback_get_list(self, ctx)
    }
    fn create_byte_list(ctx: Ctx<Self>, v: &[u8]) -> WriteResult<Self> {
        fallback_create_list(ctx, v)
    }

    fn get_int_list(&self, ctx: Ctx<Self>) -> ReadResult<Box<[i32]>> {
        fallback_get_list(self, ctx)
    }
    fn create_int_list(ctx: Ctx<Self>, v: &[i32]) -> WriteResult<Self> {
        fallback_create_list(ctx, v)
    }

    fn get_long_list(&self, ctx: Ctx<Self>) -> ReadResult<Box<[i64]>> {
        fallback_get_list(self, ctx)
    }
    fn create_long_list(ctx: Ctx<Self>, v: &[i64]) -> WriteResult<Self> {
        fallback_create_list(ctx, v)
    }

    fn get_resolvable_item<
        I: IntoResolvableItem<Self, W>,
        W: Codec<Self> + ResolvableItem<Self>,
    >(
        &self,
        ctx: Ctx<Self>,
    ) -> ReadResult<W> {
        I::codec_read_resolvable_item(self, ctx)
    }
    fn create_resolvable_item(ctx: Ctx<Self>, i: &dyn ResolvableItem<Self>) -> WriteResult<Self> {
        ctx.0.resolver_create_item(i)
    }
}

/// A type that contains information which allows certain types, like
/// server-side registries, to be resolved.
pub struct Ctx<'a, O>(pub &'a dyn ResolverTrait<O>);
impl<'a, O> Clone for Ctx<'a, O> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<'a, O> Copy for Ctx<'a, O> {}
impl<O: Ops> Ctx<'static, O> {
    /// Create a new empty resolver context.
    ///
    /// This function is provided for convenience in tests, but is not meant to
    /// be used in actual code, as it may result in codecs unexpectedly failing
    /// to parse.
    pub fn new() -> Self {
        Self(&())
    }
}

pub trait ResolverTrait<O: Ops>: Any {
    // fn resolver_get_item(&self, item: &dyn Any) -> ReadResult<u32>;
    fn resolver_create_item(&self, item: &dyn ResolvableItem<O>) -> WriteResult<O>;
}
// example implementation
impl<O: Ops + Clone, V: Codec<O> + PartialEq + Clone + 'static> ResolverTrait<O> for Vec<(u32, V)> {
    // fn resolver_get_item(&self, item: &dyn Any) -> ReadResult<u32> {
    //     let Some(item) = item.downcast_ref::<V>() else {
    //         return Err(ReadError::custom("resolvable item is of wrong
    // type"));     };
    //     self.iter()
    //         .find(|(_, candidate)| candidate == item)
    //         .ok_or_else(|| ReadError::custom("unknown resolvable item"))
    //         .map(|(i, _)| i.clone())
    // }
    fn resolver_create_item(&self, item: &dyn ResolvableItem<O>) -> WriteResult<O> {
        let id = item.to_resolvable_id();
        let value = self
            .iter()
            .find(|(candidate, _)| candidate == &id)
            .ok_or_else(|| WriteError::custom("unknown resolvable item"))?
            .1
            .clone();
        let r = value.codec_write(Ctx(self));
        r
    }
}
impl<O: Ops> ResolverTrait<O> for () {
    fn resolver_create_item(&self, _id: &dyn ResolvableItem<O>) -> WriteResult<O> {
        Err(WriteError::custom(
            "a codec here has resolvable items, but the default Ctx was passed",
        ))
    }
}

fn fallback_get_list<O: Ops, N: TryFrom<Number, Error = ReadError>>(
    o: &O,
    ctx: Ctx<O>,
) -> ReadResult<Box<[N]>> {
    o.get_list(ctx)?
        .into_iter()
        .map(|o| o.get_number(ctx).and_then(|n| n.try_into()))
        .collect()
}
fn fallback_create_list<O: Ops, N: Into<Number> + Copy>(ctx: Ctx<O>, v: &[N]) -> WriteResult<O> {
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

pub trait ResolvableItem<O: Ops> {
    fn name(&self) -> &'static str;
    fn to_resolvable_id(&self) -> u32;
}
pub trait IntoResolvableItem<O: Ops, W: Codec<O> + ResolvableItem<O>>:
    ResolvableItem<O> + 'static
{
    fn codec_read_resolvable_item(ops: &O, ctx: Ctx<O>) -> ReadResult<W>;
    fn from_resolvable_id(id: u32) -> Self;
}
