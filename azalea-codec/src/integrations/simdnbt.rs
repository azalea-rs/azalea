use simdnbt::{
    Mutf8String,
    owned::{NbtCompound, NbtList, NbtTag},
};

use crate::{
    Codec, Ctx, Incomplete, MapLike, NewOk, Number, Ops,
    error::{ReadError, ReadResult, WriteError, WriteResult},
    impl_mapcodec,
};

// NbtOps.java
impl Ops for NbtTag {
    type Map = NbtCompound;
    type List = std::vec::IntoIter<NbtTag>;

    fn get_number(&self, _ctx: Ctx<Self>) -> ReadResult<Number> {
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
    fn create_number(_ctx: Ctx<Self>, n: Number) -> WriteResult<Self> {
        Ok(n.into())
    }

    fn get_boolean(&self, ctx: Ctx<Self>) -> ReadResult<bool> {
        Ok(f64::from(self.get_number(ctx)?) != 0.0)
    }
    fn create_boolean(_ctx: Ctx<Self>, v: bool) -> WriteResult<Self> {
        Ok(NbtTag::Byte(v as i8))
    }

    fn get_list(&self, _ctx: Ctx<Self>) -> ReadResult<<NbtTag as Ops>::List> {
        let items: Vec<NbtTag> = match self {
            NbtTag::ByteArray(items) => items.iter().map(|&i| NbtTag::Byte(i as i8)).collect(),
            NbtTag::List(list) => list_as_tags(list),
            NbtTag::IntArray(items) => items.iter().map(|&i| NbtTag::Int(i)).collect(),
            NbtTag::LongArray(items) => items.iter().map(|&i| NbtTag::Long(i)).collect(),
            _ => return Err(ReadError::custom("not a list")),
        };
        Ok(items.into_iter())
    }
    fn create_list(_ctx: Ctx<Self>, list: &[Self]) -> WriteResult<Self> {
        // this does the homogenizing for us
        Ok(NbtTag::List(NbtList::from(list.to_owned())))
    }
    fn get_map(&self, _ctx: Ctx<Self>) -> ReadResult<NbtCompound> {
        self.compound()
            .cloned()
            .ok_or_else(|| ReadError::custom("not a map"))
    }
    fn create_map(_ctx: Ctx<Self>, map: &[(Self, Self)]) -> WriteResult<Self> {
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
    fn get_string(&self, _ctx: Ctx<Self>) -> ReadResult<String> {
        if let Some(s) = self.string() {
            Ok(s.to_string())
        } else {
            Err(ReadError::custom("not a string"))
        }
    }
    fn create_string(_ctx: Ctx<Self>, v: &str) -> WriteResult<Self> {
        Ok(NbtTag::String(v.into()))
    }
    fn get_byte_buffer(&self, ctx: Ctx<Self>) -> ReadResult<Box<[u8]>> {
        self.get_list(ctx)?
            .map(|o| o.get_number(ctx).and_then(|n| n.try_into()))
            .collect()
    }
    fn create_byte_list(_ctx: Ctx<Self>, v: &[u8]) -> WriteResult<Self> {
        Ok(NbtTag::ByteArray(v.to_owned()))
    }
    fn get_int_list(&self, ctx: Ctx<Self>) -> ReadResult<Box<[i32]>> {
        self.get_list(ctx)?
            .map(|o| o.get_number(ctx).and_then(|n| n.try_into()))
            .collect()
    }
    fn create_int_list(_ctx: Ctx<Self>, v: &[i32]) -> WriteResult<Self> {
        Ok(NbtTag::IntArray(v.to_owned()))
    }
    fn get_long_list(&self, ctx: Ctx<Self>) -> ReadResult<Box<[i64]>> {
        self.get_list(ctx)?
            .map(|o| o.get_number(ctx).and_then(|n| n.try_into()))
            .collect()
    }

    fn create_long_list(_ctx: Ctx<Self>, v: &[i64]) -> WriteResult<Self> {
        Ok(NbtTag::LongArray(v.to_owned()))
    }
}

impl From<Number> for NbtTag {
    fn from(n: Number) -> Self {
        match n {
            Number::Byte(n) => NbtTag::Byte(n),
            Number::Short(n) => NbtTag::Short(n),
            Number::Int(n) => NbtTag::Int(n),
            Number::Long(n) => NbtTag::Long(n),
            Number::Float(n) => NbtTag::Float(n),
            Number::Double(n) => NbtTag::Double(n),
        }
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
    fn codec_read(ops: &O, ctx: Ctx<O>) -> ReadResult<Self> {
        if let Ok(v) = Mutf8String::codec_read(ops, ctx) {
            return Ok(NbtTag::String(v));
        }
        if let Ok(v) = Number::codec_read(ops, ctx) {
            return Ok(NbtTag::from(v));
        }
        if let Ok(v) = bool::codec_read(ops, ctx) {
            return Ok(NbtTag::Byte(v as i8));
        }
        if let Ok(v) = NbtCompound::codec_read(ops, ctx) {
            return Ok(NbtTag::Compound(v));
        }
        if let Ok(v) = NbtList::codec_read(ops, ctx) {
            return Ok(NbtTag::List(v));
        }

        Err(ReadError::custom("unknown type"))
    }
    fn codec_write(&self, ctx: Ctx<O>) -> WriteResult<O> {
        match self {
            NbtTag::Byte(v) => v.codec_write(ctx),
            NbtTag::Short(v) => v.codec_write(ctx),
            NbtTag::Int(v) => v.codec_write(ctx),
            NbtTag::Long(v) => v.codec_write(ctx),
            NbtTag::Float(v) => v.codec_write(ctx),
            NbtTag::Double(v) => v.codec_write(ctx),
            NbtTag::ByteArray(v) => v.codec_write(ctx),
            NbtTag::String(v) => v.codec_write(ctx),
            NbtTag::List(v) => v.codec_write(ctx),
            NbtTag::Compound(v) => v.codec_write(ctx),
            NbtTag::IntArray(v) => v.codec_write(ctx),
            NbtTag::LongArray(v) => v.codec_write(ctx),
        }
    }
}

impl<O: Ops> Codec<O> for Mutf8String {
    fn codec_read(ops: &O, ctx: Ctx<O>) -> ReadResult<Self> {
        Ok(String::codec_read(ops, ctx)?.into())
    }
    fn codec_write(&self, ctx: Ctx<O>) -> WriteResult<O> {
        self.to_string().codec_write(ctx)
    }
}
impl<O: Ops> Codec<O> for NbtList {
    fn codec_read(ops: &O, ctx: Ctx<O>) -> ReadResult<Self> {
        Ok(Vec::<NbtTag>::codec_read(ops, ctx)?.into())
    }
    fn codec_write(&self, ctx: Ctx<O>) -> WriteResult<O> {
        list_as_tags(self).codec_write(ctx)
    }
}

fn list_as_tags(list: &NbtList) -> Vec<NbtTag> {
    if let Some(compounds) = list.compounds() {
        // dehomogenize lists of compounds
        let mut tags = Vec::new();
        for c in compounds {
            if c.len() == 1
                && let Some(item) = c.get("")
            {
                tags.push(item.clone());
            } else {
                tags.push(NbtTag::Compound(c.clone()));
            }
        }
        tags
    } else {
        list.as_nbt_tags()
    }
}

impl<O: Ops> Codec<O> for NbtCompound {
    type Incomplete = IncompleteNbtCompound;
    fn codec_read(ops: &O, ctx: Ctx<O>) -> ReadResult<Self> {
        let map = ops.get_map(ctx)?;

        let mut values = Vec::new();
        map.for_each(|k, v| {
            let k = Mutf8String::codec_read(k, ctx);
            let v = NbtTag::codec_read(v, ctx);
            let r = k.and_then(|k| v.map(|v| (k, v)));
            values.push(r);
        });
        let values = values.into_iter().collect::<ReadResult<Vec<_>>>()?;

        Ok(NbtCompound::from_values(values))
    }
    fn codec_write(&self, ctx: Ctx<O>) -> WriteResult<O> {
        let mut map = Vec::new();
        for (k, v) in self.iter() {
            map.push((k.to_owned().codec_write(ctx)?, v.codec_write(ctx)?))
        }
        O::create_map(ctx, &map)
    }
}
impl_mapcodec!(NbtCompound, IncompleteNbtCompound, Mutf8String: NbtTag);

crate::impl_defaultable!(NbtCompound);
