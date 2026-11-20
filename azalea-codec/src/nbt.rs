use simdnbt::{
    Mutf8String,
    owned::{NbtCompound, NbtList, NbtTag},
};

use crate::{
    Codec, MapLike, Number, Ops,
    error::{ReadError, ReadResult, WriteError, WriteResult},
};

// NbtOps.java
impl Ops for NbtTag {
    type Map = NbtCompound;
    type List = std::vec::IntoIter<NbtTag>;

    fn empty() -> Self {
        NbtCompound::new().into()
    }

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

    fn get_boolean(&self) -> ReadResult<bool> {
        Ok(f64::from(self.get_number()?) != 0.0)
    }
    fn create_boolean(&self, v: bool) -> WriteResult<Self> {
        Ok(NbtTag::Byte(v as i8))
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

// impl Codec<NbtTag> for NbtTag {
//     default fn codec_read(ops: &NbtTag) -> ReadResult<Self> {
//         Ok(ops.clone())
//     }
//     default fn codec_write(self, _: &NbtTag) -> WriteResult<NbtTag> {
//         Ok(self)
//     }
// }

// equivalent to convertTo in mojang codecs
impl<O: Ops> Codec<O> for NbtTag {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        if let Ok(s) = ops.get_string()
            && let Ok(v) = NbtTag::empty().create_string(&s)
        {
            return Ok(v);
        }
        if let Ok(n) = ops.get_number()
            && let Ok(v) = NbtTag::empty().create_number(n)
        {
            return Ok(v);
        }
        if let Ok(b) = ops.get_boolean()
            && let Ok(v) = NbtTag::empty().create_boolean(b)
        {
            return Ok(v);
        }
        if let Ok(map) = ops.get_map() {
            let original_values = map.into_values();
            let mut values = Vec::with_capacity(original_values.len());
            for (k, v) in original_values {
                let k = Self::codec_read(&k)?;
                let v = Self::codec_read(&v)?;
                values.push((k, v));
            }
            if let Ok(map) = NbtTag::empty().create_map(&values) {
                return Ok(map);
            }
        }
        if let Ok(list) = ops.get_list() {
            let values = list
                .into_iter()
                .map(|i| Self::codec_read(&i))
                .collect::<ReadResult<Box<[NbtTag]>>>();
            if let Ok(values) = values
                && let Ok(map) = NbtTag::empty().create_list(&values)
            {
                return Ok(map);
            }
        }

        Err(ReadError::custom("unknown type"))
    }
    fn codec_write(self, ops: &O) -> WriteResult<O> {
        Ok(ops.clone())
    }
}
