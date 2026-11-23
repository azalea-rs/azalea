use uuid::Uuid;

use crate::{
    Codec, Ctx, Ops,
    error::{ReadError, ReadResult, WriteResult},
};

impl<O: Ops> Codec<O> for uuid::Uuid {
    fn codec_read(ops: &O, ctx: Ctx<O>) -> ReadResult<Self> {
        let s = String::codec_read(ops, ctx)?;
        Uuid::parse_str(&s).map_err(|e| ReadError::custom(format!("invalid uuid: {e}")))
    }

    fn codec_write(&self, ctx: Ctx<O>) -> WriteResult<O> {
        self.to_string().codec_write(ctx)
    }
}
