#![feature(associated_type_defaults)]

use azalea_codec_macros::Codec;

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

pub trait Codec<O: Ops>: Sized {
    type Incomplete = ReadResult<Self>;

    fn codec_read(ops: &O) -> ReadResult<Self>;
    fn codec_write(self, ops: &O) -> WriteResult<O>;
}

pub trait MapCodec<O: Ops>: Sized + Codec<O> {
    fn codec_read_map_item(incomplete: &mut <Self as Codec<O>>::Incomplete, k: &O, v: &O);
    fn codec_write_to_map(self, ops: &O, map: &mut Vec<(O, O)>);
}

// https://github.com/Mojang/DataFixerUpper/blob/master/src/main/java/com/mojang/serialization/DynamicOps.java
pub trait Ops: Sized {
    type List: IntoIterator<Item = Self>;
    type Map: MapLike<Self>;

    fn empty(&self) -> WriteResult<Self> {
        self.create_map(&[])
    }
    fn empty_list(&self) -> WriteResult<Self> {
        self.create_list(&[])
    }

    fn get_number(&self) -> ReadResult<Number> {
        Err(ReadError::unimplemented())
    }
    fn create_number(&self, _n: Number) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_list(&self) -> ReadResult<Self::List> {
        Err(ReadError::unimplemented())
    }
    fn create_list(&self, _list: &[Self]) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_map(&self) -> ReadResult<Self::Map> {
        Err(ReadError::unimplemented())
    }
    fn create_map(&self, _map: &[(Self, Self)]) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_string(&self) -> ReadResult<String> {
        Err(ReadError::unimplemented())
    }
    fn create_string(&self, _v: &str) -> WriteResult<Self> {
        Err(WriteError::unimplemented())
    }

    fn get_byte_buffer(&self) -> ReadResult<Box<[u8]>> {
        fallback_get_list(self)
    }
    fn create_byte_list(&self, v: &[u8]) -> WriteResult<Self> {
        fallback_create_list(self, v)
    }

    fn get_int_list(&self) -> ReadResult<Box<[i32]>> {
        fallback_get_list(self)
    }
    fn create_int_list(&self, v: &[i32]) -> WriteResult<Self> {
        fallback_create_list(self, v)
    }

    fn get_long_list(&self) -> ReadResult<Box<[i64]>> {
        fallback_get_list(self)
    }
    fn create_long_list(&self, v: &[i64]) -> WriteResult<Self> {
        fallback_create_list(self, v)
    }
}

fn fallback_get_list<O: Ops, N: From<Number>>(o: &O) -> ReadResult<Box<[N]>> {
    o.get_list()?
        .into_iter()
        .map(|o| o.get_number().map(|n| n.into()))
        .collect()
}
fn fallback_create_list<O: Ops, N: Into<Number> + Copy>(o: &O, v: &[N]) -> WriteResult<O> {
    o.create_list(
        &v.iter()
            .map(|&i| o.create_number(i.into()))
            .collect::<WriteResult<Box<[O]>>>()?,
    )
}

pub trait MapLike<T>: Sized {
    fn for_each(&self, cb: impl FnMut(&T, &T));
}

pub struct NoMap;
impl<T> MapLike<T> for NoMap {
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

pub trait Defaultable<O: Ops>: Codec<O> {
    fn is_default(&self) -> bool;
}
