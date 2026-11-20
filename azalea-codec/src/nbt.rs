use simdnbt::owned::{NbtCompound, NbtList, NbtTag};

use crate::{
    MapLike, Number, Ops,
    error::{ReadError, ReadResult, WriteError, WriteResult},
};

// NbtOps.java
impl Ops for NbtTag {
    type Map = NbtCompound;
    type List = std::vec::IntoIter<NbtTag>;

    fn get_number(&self) -> ReadResult<Number> {
        Ok(match *self {
            NbtTag::Byte(v) => Number::Byte(v),
            NbtTag::Short(v) => Number::Short(v),
            NbtTag::Int(v) => Number::Int(v),
            NbtTag::Long(v) => Number::Long(v),
            NbtTag::Float(v) => Number::Float(v),
            NbtTag::Double(v) => Number::Double(v),
            _ => return Err(ReadError::custom("not a number")),
        })
    }
    fn create_number(&self, n: Number) -> WriteResult<Self> {
        Ok(match n {
            Number::Byte(n) => NbtTag::Byte(n),
            Number::Short(n) => NbtTag::Short(n),
            Number::Int(n) => NbtTag::Int(n),
            Number::Long(n) => NbtTag::Long(n),
            Number::Float(n) => NbtTag::Float(n),
            Number::Double(n) => NbtTag::Double(n),
        })
    }
    fn get_list(&self) -> ReadResult<<NbtTag as Ops>::List> {
        let items: Vec<NbtTag> = match self {
            NbtTag::ByteArray(items) => items.into_iter().map(|&i| NbtTag::Byte(i as i8)).collect(),
            NbtTag::List(items) => items.as_nbt_tags(),
            NbtTag::IntArray(items) => items.into_iter().map(|&i| NbtTag::Byte(i as i8)).collect(),
            NbtTag::LongArray(items) => items.into_iter().map(|&i| NbtTag::Byte(i as i8)).collect(),
            _ => return Err(ReadError::custom("not a list")),
        };
        Ok(items.into_iter())
    }
    fn create_list(&self, list: &[Self]) -> WriteResult<Self> {
        // this does the homogenizing for us
        Ok(NbtTag::List(NbtList::from(list.to_owned())))
    }
    fn get_map(&self) -> ReadResult<NbtCompound> {
        self.compound()
            .cloned()
            .ok_or_else(|| ReadError::custom("not a map"))
    }
    fn create_map(&self, map: &[(Self, Self)]) -> WriteResult<Self> {
        let mut items = Vec::with_capacity(map.len());
        for (k, v) in map {
            let Some(k) = k.string() else {
                return Err(WriteError::custom(
                    "cannot create a map with non-string keys",
                ));
            };

            items.push((k.to_owned(), v.to_owned()));
        }

        Ok(NbtTag::Compound(NbtCompound::from_values(items)))
    }
    fn get_string(&self) -> ReadResult<String> {
        if let Some(s) = self.string() {
            Ok(s.to_string())
        } else {
            Err(ReadError::custom("not a string"))
        }
    }
    fn create_string(&self, v: &str) -> WriteResult<Self> {
        Ok(NbtTag::String(v.into()))
    }
    fn get_byte_buffer(&self) -> ReadResult<Box<[u8]>> {
        self.get_list()?
            .into_iter()
            .map(|o| o.get_number().map(|n| n.into()))
            .collect()
    }
    fn create_byte_list(&self, v: &[u8]) -> WriteResult<Self> {
        Ok(NbtTag::ByteArray(v.to_owned()))
    }
    fn get_int_list(&self) -> ReadResult<Box<[i32]>> {
        self.get_list()?
            .into_iter()
            .map(|o| o.get_number().map(|n| n.into()))
            .collect()
    }
    fn create_int_list(&self, v: &[i32]) -> WriteResult<Self> {
        Ok(NbtTag::IntArray(v.to_owned()))
    }
    fn get_long_list(&self) -> ReadResult<Box<[i64]>> {
        self.get_list()?
            .into_iter()
            .map(|o| o.get_number().map(|n| n.into()))
            .collect()
    }

    fn create_long_list(&self, v: &[i64]) -> WriteResult<Self> {
        Ok(NbtTag::LongArray(v.to_owned()))
    }
}

impl MapLike<NbtTag> for NbtCompound {
    fn for_each(&self, mut cb: impl FnMut(&NbtTag, &NbtTag)) {
        for (k, v) in self.iter() {
            cb(&NbtTag::String(k.into()), v);
        }
    }
}
