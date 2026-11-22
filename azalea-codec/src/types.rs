use crate::{
    Codec, MapCodec, Ops,
    error::{ReadError, ReadResult, WriteResult},
};

/// Represents a string value that must be matched exactly when decoding this
/// codec.
///
/// This is useful for representing enums with tag fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct StringLiteral<const S: &'static str>;
impl<const S: &'static str, O: Ops> Codec<O> for StringLiteral<S> {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        let s = ops.get_string()?;
        if s == S {
            Ok(Self)
        } else {
            Err(ReadError::custom(format!("'{s}' != '{S}'")))
        }
    }
    fn codec_write(&self) -> WriteResult<O> {
        O::create_string(S)
    }
}

/// A codec that always matches everything and writes nothing.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct Drain;
impl<O: Ops> Codec<O> for Drain {
    fn codec_read(_ops: &O) -> ReadResult<Self> {
        Ok(Self)
    }
    fn codec_write(&self) -> WriteResult<O> {
        O::empty()
    }
}
impl<O: Ops> MapCodec<O> for Drain {
    fn codec_read_map_item(
        _incomplete: &mut <Self as crate::Codec<O>>::Incomplete,
        _k: &O,
        _v: &O,
    ) {
    }
    fn codec_write_to_map(&self, _map: &mut Vec<(O, O)>) -> WriteResult<()> {
        Ok(())
    }
    fn get_and_reset_result(_incomplete: &mut <Self as Codec<O>>::Incomplete) -> ReadResult<()> {
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompactListCodec<T>(pub T);
impl<O: Ops, T: IntoIterator<Item = V> + FromIterator<V> + Clone, V: Codec<O>> Codec<O>
    for CompactListCodec<T>
{
    fn codec_read(ops: &O) -> ReadResult<Self> {
        if let Ok(v) = V::codec_read(ops) {
            return Ok(Self(T::from_iter([v])));
        }
        Ok(Self(T::from_iter(Vec::<V>::codec_read(ops)?)))
    }
    fn codec_write(&self) -> WriteResult<O> {
        let v = self.0.clone().into_iter().collect::<Vec<V>>();
        if v.len() == 1 {
            v[0].codec_write()
        } else {
            v.codec_write()
        }
    }
}
