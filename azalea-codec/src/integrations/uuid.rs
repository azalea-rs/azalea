use uuid::Uuid;

use crate::{
    Codec, Ops,
    error::{ReadError, ReadResult, WriteResult},
};

impl<O: Ops> Codec<O> for uuid::Uuid {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        let s = String::codec_read(ops)?;
        Uuid::parse_str(&s).map_err(|e| ReadError::custom(format!("invalid uuid: {e}")))
    }

    fn codec_write(&self) -> WriteResult<O> {
        self.to_string().codec_write()
    }
}
