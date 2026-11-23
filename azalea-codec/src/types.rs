use std::marker::PhantomData;

use crate::{
    Codec, Ctx, MapCodec, Ops,
    error::{ReadError, ReadResult, WriteResult},
};

/// Represents a string value that must be matched exactly when decoding this
/// codec.
///
/// This is useful for representing enums with tag fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct StringLiteral<const S: &'static str>;
impl<const S: &'static str, O: Ops> Codec<O> for StringLiteral<S> {
    fn codec_read(ops: &O, ctx: Ctx<O>) -> ReadResult<Self> {
        let s = ops.get_string(ctx)?;
        if s == S {
            Ok(Self)
        } else {
            Err(ReadError::custom(format!("'{s}' != '{S}'")))
        }
    }
    fn codec_write(&self, ctx: Ctx<O>) -> WriteResult<O> {
        O::create_string(ctx, S)
    }
}

#[derive(Clone, Debug)]
pub struct FromLiteral<const S: &'static str, T> {
    _unused: PhantomData<T>,
}
impl<const S: &'static str, T> Default for FromLiteral<S, T> {
    fn default() -> Self {
        Self {
            _unused: Default::default(),
        }
    }
}
impl<const S: &'static str, T: const ConstFromStr + ToString + Codec<O> + PartialEq, O: Ops>
    Codec<O> for FromLiteral<S, T>
{
    fn codec_read(ops: &O, ctx: Ctx<O>) -> ReadResult<Self> {
        let attempt = T::codec_read(ops, ctx)?;
        let solution = const { T::const_from_str(S) };
        if attempt == solution {
            Ok(Self::default())
        } else {
            Err(ReadError::custom(format!(
                "mismatching FromLiteral, {} != {}",
                attempt.to_string(),
                solution.to_string()
            )))
        }
    }
    fn codec_write(&self, ctx: Ctx<O>) -> WriteResult<O> {
        let ident = const { T::const_from_str(S) };
        ident.codec_write(ctx)
    }
}
/// Types that can be converted from a `&'static str` at compile-time.
///
/// Used for compile-time validation of `#[codec(tag_as = ...)]` tags.
pub const trait ConstFromStr {
    fn const_from_str(s: &'static str) -> Self;
}

/// A codec that always matches everything and writes nothing.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct Drain;
impl<O: Ops> Codec<O> for Drain {
    fn codec_read(_ops: &O, _ctx: Ctx<O>) -> ReadResult<Self> {
        Ok(Self)
    }
    fn codec_write(&self, ctx: Ctx<O>) -> WriteResult<O> {
        O::empty(ctx)
    }
}
impl<O: Ops> MapCodec<O> for Drain {
    fn codec_read_map_item(
        _incomplete: &mut <Self as crate::Codec<O>>::Incomplete,
        _ctx: Ctx<O>,
        _k: &O,
        _v: &O,
    ) {
    }
    fn codec_write_to_map(&self, _ctx: Ctx<O>, _map: &mut Vec<(O, O)>) -> WriteResult<()> {
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
    fn codec_read(ops: &O, ctx: Ctx<O>) -> ReadResult<Self> {
        let mut errors = Vec::new();
        match V::codec_read(ops, ctx) {
            Ok(v) => {
                return Ok(Self(T::from_iter([v])));
            }
            Err(e) => errors.push(("V", e)),
        }
        match Vec::<V>::codec_read(ops, ctx) {
            Ok(v) => return Ok(Self(T::from_iter(v))),
            Err(e) => errors.push(("Vec<V>", e)),
        }
        Err(ReadError::no_variants_matched(errors))
    }
    fn codec_write(&self, ctx: Ctx<O>) -> WriteResult<O> {
        let v = self.0.clone().into_iter().collect::<Vec<V>>();
        if v.len() == 1 {
            v[0].codec_write(ctx)
        } else {
            v.codec_write(ctx)
        }
    }
}
