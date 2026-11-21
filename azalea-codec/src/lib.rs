#![feature(associated_type_defaults)]
// needed for StringLiteral
#![allow(incomplete_features)]
#![feature(adt_const_params, unsized_const_params)]
// thiserror
#![feature(error_generic_member_access)]

pub use azalea_codec_macros::Codec;

use crate::{
    error::{ReadError, ReadResult, WriteError, WriteResult},
    number::Number,
};

pub mod error;
#[cfg(feature = "serde_json")]
pub mod json;
#[cfg(feature = "simdnbt")]
pub mod nbt;
pub mod number;
pub mod rust;
pub mod types;

#[diagnostic::on_unimplemented(message = "type `{Self}` does not implement `azalea_codec::Codec`.")]
pub trait Codec<O: Ops>: Sized {
    type Incomplete = ReadResult<Self>;

    /// Convert from a **data format** to a **data structure**.
    fn codec_read(ops: &O) -> ReadResult<Self>;
    /// Convert from a **data structure** to a **data format**.
    fn codec_write(&self) -> WriteResult<O>;
}

#[diagnostic::on_unimplemented(message = "type `{Self}` is not a map that can be flattened.")]
pub trait MapCodec<O: Ops>: Sized + Codec<O> {
    // this doesn't return a ReadResult because the errors are stored in the
    // Incomplete struct
    fn codec_read_map_item(incomplete: &mut <Self as Codec<O>>::Incomplete, k: &O, v: &O);
    fn codec_write_to_map(&self, map: &mut Vec<(O, O)>) -> WriteResult<()>;

    /// For normal structs, this sets `incomplete.__result = Ok(())`.
    fn get_and_reset_result(incomplete: &mut <Self as Codec<O>>::Incomplete) -> ReadResult<()>;
}

// https://github.com/Mojang/DataFixerUpper/blob/master/src/main/java/com/mojang/serialization/DynamicOps.java
#[diagnostic::on_unimplemented(
    message = "type `{Self}` is not a data format; did you mean to use `{Self}::codec_read` instead?"
)]
pub trait Ops: Sized + Clone {
    type List: IntoIterator<Item = Self>;
    type Map: MapLike<Self>;

    fn empty() -> WriteResult<Self> {
        Self::create_map(&[])
    }
    fn empty_list() -> WriteResult<Self> {
        Self::create_list(&[])
    }

    fn get_number(&self) -> ReadResult<Number> {
        Err(ReadError::unimplemented())
    }
    fn create_number(_n: Number) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_boolean(&self) -> ReadResult<bool> {
        Err(ReadError::unimplemented())
    }
    fn create_boolean(_v: bool) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_list(&self) -> ReadResult<Self::List> {
        Err(ReadError::unimplemented())
    }
    fn create_list(_list: &[Self]) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_map(&self) -> ReadResult<Self::Map> {
        Err(ReadError::unimplemented())
    }
    fn create_map(_map: &[(Self, Self)]) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_string(&self) -> ReadResult<String> {
        Err(ReadError::unimplemented())
    }
    fn create_string(_v: &str) -> WriteResult<Self> {
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

    fn into_values(&self) -> Box<[(T, T)]> {
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
        Err(error::ReadError::missing_key(name))
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

pub trait Defaultable<O: Ops>: Codec<O> {
    fn is_default(&self) -> bool;
}
