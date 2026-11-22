use std::hash::Hash;

use indexmap::IndexMap;

use crate::{
    Codec, MapLike, Ops,
    error::{ReadResult, WriteResult},
};

impl<O: Ops, K: Codec<O> + Hash + Eq, V: Codec<O>> Codec<O> for IndexMap<K, V> {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        let map = ops.get_map()?;
        let mut items = Vec::new();
        map.for_each(|k, v| {
            items.push(K::codec_read(k).and_then(|k| V::codec_read(v).map(|v| (k, v))));
        });

        Ok(IndexMap::from_iter(
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
