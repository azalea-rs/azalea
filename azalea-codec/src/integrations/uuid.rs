use uuid::Uuid;

use crate::{
    Codec, Ops,
    error::{ReadResult, WriteResult},
};

impl<O: Ops> Codec<O> for uuid::Uuid {
    fn codec_read(ops: &O) -> ReadResult<Self> {
        let r = <[u32; 4]>::codec_read(ops)?;
        let msb = (r[0] as u64) << 32 | (r[1] as u64);
        let lsb = (r[2] as u64) << 32 | (r[3] as u64);
        Ok(Uuid::from_u64_pair(msb, lsb))
    }

    fn codec_write(&self) -> WriteResult<O> {
        let (msb, lsb) = self.as_u64_pair();
        let arr = [
            (msb >> 32) as u32,
            msb as u32,
            (lsb >> 32) as u32,
            lsb as u32,
        ];
        arr.codec_write()
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::Codec;

    #[test]
    #[cfg(feature = "serde_json")]
    fn roundtrip_uuid() {
        let uuid = Uuid::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        let res: serde_json::Value = uuid.codec_write().unwrap();
        let new = Uuid::codec_read(&res).unwrap();
        assert_eq!(uuid, new);
    }
}
