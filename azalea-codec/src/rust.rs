use std::{collections::HashMap, hash::Hash, mem::ManuallyDrop};

use crate::{
    Codec, Defaultable, Incomplete, MapCodec, MapLike, Ops,
    error::{ReadError, ReadResult, WriteResult},
    impl_defaultable,
};

impl<O: Ops> Codec<O> for String {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        ops.get_string()
    }
    fn codec_write(&self, ctx: O::Context) -> WriteResult<O> {
        O::create_string(ctx, self)
    }
}
impl<O: Ops> MapCodec<O> for String {
    fn codec_read_map_item(_incomplete: &mut <Self as Codec<O>>::Incomplete, _k: &O, _v: &O) {
        panic!("not a map")
    }
    fn codec_write_to_map(&self, _ctx: O::Context, _map: &mut Vec<(O, O)>) -> WriteResult<()> {
        panic!("not a map")
    }
    fn get_and_reset_result(_incomplete: &mut <Self as Codec<O>>::Incomplete) -> ReadResult<()> {
        panic!("not a map")
    }
}

impl<O: Ops> Codec<O> for bool {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        ops.get_boolean()
    }
    fn codec_write(&self, ctx: O::Context) -> WriteResult<O> {
        O::create_boolean(ctx, *self)
    }
}
impl_defaultable!(bool);

impl<O: Ops> Codec<O> for () {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        if let Ok(map) = ops.get_map()
            && map.to_values().is_empty()
        {
            return Ok(());
        }
        Err(ReadError::custom("not an empty map"))
    }
    fn codec_write(&self, ctx: O::Context) -> WriteResult<O> {
        O::empty_map(ctx)
    }
}

macro_rules! impl_numbers {
    ($($n:ident),*) => {
        $(
            impl<O: Ops> Codec<O> for $n {
                fn codec_read(ops: &O) -> ReadResult<Self> {
                    ops.get_number()?.try_into().map_err(Into::into)
                }
                fn codec_write(&self, ctx: O::Context) -> WriteResult<O> {
                    O::create_number(ctx, (*self).try_into()?)
                }
            }
            crate::impl_defaultable!($n);
        )*
    }
}

#[rustfmt::skip]
impl_numbers!(
    u8, u16, u32, u64,
    i8, i16, i32, i64,
    f32, f64
);

impl<O: Ops, T: Codec<O>> Codec<O> for Vec<T> {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        ops.get_list().and_then(|l| {
            l.into_iter()
                .map(|i| T::codec_read(&i))
                .collect::<ReadResult<Vec<T>>>()
        })
    }
    fn codec_write(&self, ctx: O::Context) -> WriteResult<O> {
        O::create_list(
            ctx,
            &self
                .iter()
                .map(|i| i.codec_write(ctx))
                .collect::<WriteResult<Box<[O]>>>()?,
        )
    }
}
crate::impl_defaultable!(<T: Codec<O>> for Vec<T>, self, self.is_empty());

fn array_codec_read<const N: usize, O: Ops, T: Codec<O> + Clone>(ops: &O) -> ReadResult<[T; N]> {
    let read = Vec::<T>::codec_read(ops)?;
    let len = read.len();
    read.try_into().map_err(|_| {
        ReadError::custom(format!(
            "wrong array length, was required to be {N} but got {len}"
        ))
    })
}
impl<const N: usize, O: Ops, T: Codec<O> + Clone> Codec<O> for [T; N] {
    default fn codec_read(ops: &O) -> ReadResult<Self> {
        array_codec_read(ops)
    }
    default fn codec_write(&self, ctx: O::Context) -> WriteResult<O> {
        self.to_vec().codec_write(ctx)
    }
}
impl<const N: usize, O: Ops> Codec<O> for [i32; N] {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        array_codec_read(ops)
    }
    fn codec_write(&self, ctx: O::Context) -> WriteResult<O> {
        O::create_int_list(ctx, self)
    }
}
impl<const N: usize, O: Ops> Codec<O> for [i64; N] {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        array_codec_read(ops)
    }
    fn codec_write(&self, ctx: O::Context) -> WriteResult<O> {
        O::create_long_list(ctx, self)
    }
}

impl<O: Ops, A: Codec<O>, B: Codec<O>> Codec<O> for (A, B) {
    /// Read as a list.
    fn codec_read(ops: &O) -> ReadResult<Self> {
        let list = Vec::<O>::codec_read(ops)?;
        if list.len() != 2 {
            return Err(ReadError::custom(format!(
                "wrong array length, was required to be 2 but got {}",
                list.len()
            )));
        }
        let mut iter = list.iter();
        Ok((
            A::codec_read(iter.next().unwrap())?,
            B::codec_read(iter.next().unwrap())?,
        ))
    }
    /// Written as a list.
    fn codec_write(&self, ctx: O::Context) -> WriteResult<O> {
        let list = [self.0.codec_write(ctx)?, self.1.codec_write(ctx)?];
        list.codec_write(ctx)
    }
}

impl<O: Ops, K: Codec<O> + Eq + Hash, V: Codec<O>> Codec<O> for HashMap<K, V> {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        let map = ops.get_map()?;
        let mut items = Vec::new();
        map.for_each(|k, v| {
            items.push(K::codec_read(k).and_then(|k| V::codec_read(v).map(|v| (k, v))));
        });

        Ok(HashMap::from_iter(
            items.into_iter().collect::<ReadResult<Box<[_]>>>()?,
        ))
    }
    fn codec_write(&self, ctx: O::Context) -> WriteResult<O> {
        let mut map = Vec::new();
        for (k, v) in self {
            map.push((k.codec_write(ctx)?, v.codec_write(ctx)?));
        }
        O::create_map(ctx, &map)
    }
}

impl<O: Ops, T: Codec<O>> Codec<O> for Option<T> {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        T::codec_read(ops).map(Some)
    }
    fn codec_write(&self, ctx: O::Context) -> WriteResult<O> {
        if let Some(v) = self {
            v.codec_write(ctx)
        } else {
            O::empty(ctx)
        }
    }
}
impl<T: Codec<O>, O: Ops> Defaultable<O> for Option<T> {
    fn is_default(&self) -> bool {
        self.is_none()
    }
}

impl<O: Ops, T: Codec<O>> Codec<O> for Box<T> {
    type Incomplete = T::Incomplete;
    fn codec_read(ops: &O) -> ReadResult<Self> {
        T::codec_read(ops).map(Box::new)
    }
    fn codec_write(&self, ctx: O::Context) -> WriteResult<O> {
        (**self).codec_write(ctx)
    }
}
impl<O: Ops, T: MapCodec<O>> MapCodec<O> for Box<T> {
    fn codec_read_map_item(incomplete: &mut <Self as Codec<O>>::Incomplete, k: &O, v: &O) {
        T::codec_read_map_item(incomplete, k, v)
    }
    fn codec_write_to_map(&self, ctx: O::Context, map: &mut Vec<(O, O)>) -> WriteResult<()> {
        (**self).codec_write_to_map(ctx, map)
    }
    fn get_and_reset_result(incomplete: &mut <Self as Codec<O>>::Incomplete) -> ReadResult<()> {
        T::get_and_reset_result(incomplete)
    }
}
impl<T: Incomplete> Incomplete for Box<T> {
    type Complete = Box<T::Complete>;
    fn new(name: &'static str) -> Self {
        Box::new(T::new(name))
    }
    fn build(self) -> ReadResult<Self::Complete> {
        (*self).build().map(Box::new)
    }
}
impl<T: Codec<O> + Defaultable<O>, O: Ops> Defaultable<O> for Box<T> {
    fn is_default(&self) -> bool {
        (**self).is_default()
    }
}

impl<O: Ops, T: Codec<O>> Codec<O> for ManuallyDrop<T> {
    type Incomplete = T::Incomplete;
    fn codec_read(ops: &O) -> ReadResult<Self> {
        T::codec_read(ops).map(ManuallyDrop::new)
    }
    fn codec_write(&self, ctx: O::Context) -> WriteResult<O> {
        (**self).codec_write(ctx)
    }
}

impl<T: Clone> MapLike<T> for Vec<(T, T)> {
    fn for_each(&self, mut cb: impl FnMut(&T, &T)) {
        self.iter().for_each(|(k, v)| cb(k, v));
    }
}
