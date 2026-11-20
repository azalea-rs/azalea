use simdnbt::owned::{NbtCompound, NbtTag};

use crate::{MapLike, Number, Ops, error::Error};

impl Ops for NbtTag {
    type Map = NbtCompound;
    type List = std::vec::IntoIter<NbtTag>;

    fn get_number(&self) -> crate::Result<Number> {
        Ok(match *self {
            NbtTag::Byte(v) => Number::Byte(v),
            NbtTag::Short(v) => Number::Short(v),
            NbtTag::Int(v) => Number::Int(v),
            NbtTag::Long(v) => Number::Long(v),
            NbtTag::Float(v) => Number::Float(v),
            NbtTag::Double(v) => Number::Double(v),
            _ => return Err(Error::custom("not a number")),
        })
    }
    fn create_byte(&self, v: i8) -> Self {
        NbtTag::Byte(v)
    }

    fn get_map(&self) -> crate::Result<NbtCompound> {
        self.compound()
            .cloned()
            .ok_or_else(|| Error::custom("not a map"))
    }
    fn get_list(&self) -> crate::Result<<NbtTag as Ops>::List> {
        let items: Vec<NbtTag> = match self {
            NbtTag::ByteArray(items) => items.into_iter().map(|&i| NbtTag::Byte(i as i8)).collect(),
            NbtTag::List(items) => items.as_nbt_tags(),
            NbtTag::IntArray(items) => items.into_iter().map(|&i| NbtTag::Byte(i as i8)).collect(),
            NbtTag::LongArray(items) => items.into_iter().map(|&i| NbtTag::Byte(i as i8)).collect(),
            _ => return Err(Error::custom("not a list")),
        };
        Ok(items.into_iter())
    }
}

impl MapLike<NbtTag> for NbtCompound {
    fn for_each(&self, mut cb: impl FnMut(&NbtTag, &NbtTag)) {
        for (k, v) in self.iter() {
            cb(&NbtTag::String(k.into()), v);
        }
    }
}
