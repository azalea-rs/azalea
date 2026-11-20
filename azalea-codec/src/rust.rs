use crate::{
    Codec, Defaultable, Ops,
    error::{ReadResult, WriteResult},
};

impl<O: Ops> Codec<O> for String {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        ops.get_string()
    }
    fn codec_write(self, ops: &O) -> WriteResult<O> {
        ops.create_string(&self)
    }
}
impl<O: Ops, T: Codec<O>> Codec<O> for Vec<T> {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        ops.get_list()
            .map(|l| {
                l.into_iter()
                    .map(|i| T::codec_read(&i))
                    .collect::<ReadResult<Vec<T>>>()
            })
            .flatten()
    }
    fn codec_write(self, ops: &O) -> WriteResult<O> {
        ops.create_list(
            &self
                .into_iter()
                .map(|i| i.codec_write(ops))
                .collect::<WriteResult<Box<[O]>>>()?,
        )
    }
}
impl<T: Codec<O>, O: Ops> Defaultable<O> for Vec<T> {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

impl<O: Ops, T: Codec<O>> Codec<O> for Option<T> {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        T::codec_read(ops).map(Some)
    }
    fn codec_write(self, ops: &O) -> WriteResult<O> {
        if let Some(v) = self {
            v.codec_write(ops)
        } else {
            ops.empty()
        }
    }
}
impl<T: Codec<O>, O: Ops> Defaultable<O> for Option<T> {
    fn is_default(&self) -> bool {
        self.is_none()
    }
}
