#![feature(associated_type_defaults)]

use azalea_codec_macros::Codec;

pub mod error;
pub mod nbt;
pub mod rust;

pub type Result<T> = std::result::Result<T, error::Error>;

pub trait Codec<O: Ops>: Sized {
    type Incomplete = Result<Self>;

    fn codec_read(ops: &O) -> Result<Self>;
    fn codec_write(self, ops: &O) -> O;
}

pub trait MapCodec<O: Ops>: Sized + Codec<O> {
    fn codec_read_map_item(incomplete: &mut <Self as Codec<O>>::Incomplete, k: &O, v: &O);
    fn codec_write_to_map(self, ops: &O, map: &mut Vec<(O, O)>);
}

pub trait Ops: Sized {
    type List: IntoIterator<Item = Self>;
    type Map: MapLike<Self>;

    fn get_number(&self) -> Result<Number> {
        unimplemented!();
    }
    fn get_list(&self) -> Result<Self::List> {
        unimplemented!();
    }
    fn get_map(&self) -> Result<Self::Map> {
        unimplemented!();
    }
    fn get_string(&self) -> Result<String> {
        unimplemented!();
    }

    fn empty(&self) -> Self {
        unimplemented!();
    }
    fn create_byte(&self, _v: i8) -> Self {
        unimplemented!();
    }
    fn create_short(&self, _v: i16) -> Self {
        unimplemented!();
    }
    fn create_int(&self, _v: i32) -> Self {
        unimplemented!();
    }
    fn create_long(&self, _v: i64) -> Self {
        unimplemented!();
    }
    fn create_list(&self, _list: &[Self]) -> Self {
        unimplemented!();
    }
    fn create_map(&self, _map: &[(Self, Self)]) -> Self {
        unimplemented!();
    }
    fn create_string(&self, _v: &str) -> Self {
        unimplemented!();
    }
}

pub enum Number {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
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

    fn build(self) -> Result<Self::Complete>;
}
impl<T> Incomplete for Result<T> {
    type Complete = T;

    fn new(name: &'static str) -> Self {
        Err(error::Error::missing_key(name))
    }
    fn build(self) -> Result<T> {
        self
    }
}

pub trait Defaultable<O: Ops>: Codec<O> {
    fn is_default(&self) -> bool;
}
