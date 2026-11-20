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
    fn codec_write(self, ops: &O) -> WriteResult<O> {
        ops.create_string(S)
    }
}

/// A codec that always matches everything, and if flattened, allows a map to
/// ignore missing fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct Drain;
impl<O: Ops> Codec<O> for Drain {
    fn codec_read(_ops: &O) -> ReadResult<Self> {
        Ok(Self)
    }
    fn codec_write(self, ops: &O) -> WriteResult<O> {
        Ok(ops.clone())
    }
}
impl<O: Ops> MapCodec<O> for Drain {
    fn codec_read_map_item(
        _incomplete: &mut <Self as crate::Codec<O>>::Incomplete,
        _k: &O,
        _v: &O,
    ) {
    }
    fn codec_write_to_map(self, _ops: &O, _map: &mut Vec<(O, O)>) -> WriteResult<()> {
        Ok(())
    }
    fn get_and_reset_result(_incomplete: &mut <Self as Codec<O>>::Incomplete) -> ReadResult<()> {
        Ok(())
    }
}
