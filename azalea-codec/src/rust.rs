use crate::{
    Codec, Defaultable, Ops,
    error::{ReadError, ReadResult, WriteResult},
};

impl<O: Ops> Codec<O> for String {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        ops.get_string()
    }
    fn codec_write(self, ops: &O) -> WriteResult<O> {
        ops.create_string(&self)
    }
}

impl<O: Ops> Codec<O> for bool {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        ops.get_boolean()
    }
    fn codec_write(self, ops: &O) -> WriteResult<O> {
        ops.create_boolean(self)
    }
}

macro_rules! impl_numbers {
    ($($n:ident),*) => {
        $(
            impl<O: Ops> Codec<O> for $n {
                fn codec_read(ops: &O) -> ReadResult<Self> {
                    Ok(ops.get_number()?.into())
                }
                fn codec_write(self, ops: &O) -> WriteResult<O> {
                    ops.create_number(self.into())
                }
            }
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
            Ok(O::empty())
        }
    }
}
impl<T: Codec<O>, O: Ops> Defaultable<O> for Option<T> {
    fn is_default(&self) -> bool {
        self.is_none()
    }
}

impl<O: Ops, T: Codec<O>> Codec<O> for Box<T> {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        T::codec_read(ops).map(Box::new)
    }
    fn codec_write(self, ops: &O) -> WriteResult<O> {
        (*self).codec_write(ops)
    }
}
impl<T: Codec<O> + Defaultable<O>, O: Ops> Defaultable<O> for Box<T> {
    fn is_default(&self) -> bool {
        (**self).is_default()
    }
}
