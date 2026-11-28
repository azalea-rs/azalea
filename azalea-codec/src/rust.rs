use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    mem::ManuallyDrop,
};

use crate::{
    Codec, Defaultable, Incomplete, MapCodec, MapLike, NewOk, Ops,
    error::{ReadError, ReadResult, WriteResult},
    impl_codec_from, impl_defaultable, impl_mapcodec, impl_mapcodec_from,
};

impl<O: Ops> Codec<O> for String {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        ops.get_string()
    }
    fn codec_write(&self) -> WriteResult<O> {
        O::create_string(self)
    }
}

impl<O: Ops> Codec<O> for bool {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        ops.get_boolean()
    }
    fn codec_write(&self) -> WriteResult<O> {
        O::create_boolean(*self)
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
    fn codec_write(&self) -> WriteResult<O> {
        O::empty_map()
    }
}

macro_rules! impl_numbers {
    ($($n:ident),*) => {
        $(
            impl<O: Ops> Codec<O> for $n {
                fn codec_read(ops: &O, ) -> ReadResult<Self> {
                    ops.get_number()?.try_into().map_err(Into::into)
                }
                fn codec_write(&self, ) -> WriteResult<O> {
                    O::create_number( (*self).try_into()?)
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
    fn codec_write(&self) -> WriteResult<O> {
        O::create_list(
            &self
                .iter()
                .map(|i| i.codec_write())
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
    default fn codec_write(&self) -> WriteResult<O> {
        self.to_vec().codec_write()
    }
}
impl<const N: usize, O: Ops> Codec<O> for [i32; N] {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        array_codec_read(ops)
    }
    fn codec_write(&self) -> WriteResult<O> {
        O::create_int_list(self)
    }
}
impl<const N: usize, O: Ops> Codec<O> for [i64; N] {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        array_codec_read(ops)
    }
    fn codec_write(&self) -> WriteResult<O> {
        O::create_long_list(self)
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
    fn codec_write(&self) -> WriteResult<O> {
        let list = [self.0.codec_write()?, self.1.codec_write()?];
        list.codec_write()
    }
}

#[derive(Clone)]
pub struct UnorderedMap<K, V>(Vec<(K, V)>);
impl<K, V> UnorderedMap<K, V> {
    pub fn insert(&mut self, k: K, v: V) {
        self.0.push((k, v));
    }
    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.0.iter()
    }
}
impl<O: Ops, K: Codec<O>, V: Codec<O>> Codec<O> for UnorderedMap<K, V> {
    type Incomplete = IncompleteUnorderedMap<K, V>;
    fn codec_read(ops: &O) -> ReadResult<Self> {
        let map = ops.get_map()?;
        let mut items = Vec::new();
        map.for_each(|k_ops, v_ops| {
            items.push(K::codec_read(k_ops).and_then(|k| {
                V::codec_read(v_ops)
                    .map_err(|e| {
                        if let Ok(key_string) = String::codec_read(k_ops) {
                            // try to attach more metadata if possible, to help
                            // with debugging :3
                            e.while_reading_key(key_string)
                        } else {
                            e
                        }
                    })
                    .map(|v| (k, v))
            }));
        });

        Ok(Self(items.into_iter().collect::<ReadResult<_>>()?))
    }
    fn codec_write(&self) -> WriteResult<O> {
        let mut map = Vec::new();
        for (k, v) in &self.0 {
            map.push((k.codec_write()?, v.codec_write()?));
        }
        O::create_map(&map)
    }
}
impl<K, V> Default for UnorderedMap<K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl_mapcodec!(UnorderedMap<K, V>, IncompleteUnorderedMap<K, V>, K: V);

impl<K, V> From<HashMap<K, V>> for UnorderedMap<K, V> {
    fn from(m: HashMap<K, V>) -> Self {
        UnorderedMap(m.into_iter().collect())
    }
}
impl<K: Hash + Eq, V> From<UnorderedMap<K, V>> for HashMap<K, V> {
    fn from(m: UnorderedMap<K, V>) -> Self {
        m.0.into_iter().collect()
    }
}
impl_codec_from!(<K: Eq + Hash + Clone, V: Clone> HashMap<K, V>, UnorderedMap<K, V>, struct IncompleteHashMap);
impl_mapcodec_from!(<K: Eq + Hash + Clone, V: Clone> HashMap<K, V>, UnorderedMap<K, V>);

//

impl<K, V> From<BTreeMap<K, V>> for UnorderedMap<K, V> {
    fn from(m: BTreeMap<K, V>) -> Self {
        UnorderedMap(m.into_iter().collect())
    }
}
impl<K: Ord, V> From<UnorderedMap<K, V>> for BTreeMap<K, V> {
    fn from(m: UnorderedMap<K, V>) -> Self {
        m.0.into_iter().collect()
    }
}
impl_codec_from!(<K: Ord + Clone, V: Clone> BTreeMap<K, V>, UnorderedMap<K, V>, struct IncompleteBTreeMap);
impl_mapcodec_from!(<K: Ord + Clone, V: Clone> BTreeMap<K, V>, UnorderedMap<K, V>);

impl<O: Ops, T: Codec<O>> Codec<O> for Option<T> {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        T::codec_read(ops).map(Some)
    }
    fn codec_write(&self) -> WriteResult<O> {
        if let Some(v) = self {
            v.codec_write()
        } else {
            O::empty()
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
    fn codec_write(&self) -> WriteResult<O> {
        (**self).codec_write()
    }
}
impl<O: Ops, T: MapCodec<O>> MapCodec<O> for Box<T> {
    fn codec_read_map_item(incomplete: &mut <Self as Codec<O>>::Incomplete, k: &O, v: &O) {
        T::codec_read_map_item(incomplete, k, v)
    }
    fn codec_write_to_map(&self, map: &mut Vec<(O, O)>) -> WriteResult<()> {
        (**self).codec_write_to_map(map)
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
    fn codec_write(&self) -> WriteResult<O> {
        (**self).codec_write()
    }
}

impl<T: Clone> MapLike<T> for Vec<(T, T)> {
    fn for_each(&self, mut cb: impl FnMut(&T, &T)) {
        self.iter().for_each(|(k, v)| cb(k, v));
    }
}
