use std::fmt;

use azalea_buf::AzBuf;
use azalea_codec::Codec;
use serde::ser;
use thiserror::Error;

use crate::registry_holder::RegistryHolder;

#[derive(AzBuf, Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Checksum(pub u32);
impl Checksum {
    #[must_use]
    fn write(mut self, bytes: &[u8]) -> Checksum {
        self.0 = crc32c::crc32c_append(self.0, bytes);
        self
    }
    fn update(&mut self, bytes: &[u8]) -> &mut Self {
        *self = self.clone().write(bytes);
        self
    }
}

#[derive(Clone)]
pub struct ChecksumOps {
    checksum: Checksum,
}
impl azalea_codec::Ops for ChecksumOps {
    type List = Vec<ChecksumOps>;
    type Map = Vec<(ChecksumOps, ChecksumOps)>;
    fn empty() -> azalea_codec::error::WriteResult<Self> {
        let checksum = Checksum(0).write(&[1]);
        Ok(Self { checksum })
    }
    fn create_number(n: azalea_codec::number::Number) -> azalea_codec::error::WriteResult<Self> {
        use azalea_codec::number::Number as N;
        let checksum = match n {
            N::Byte(n) => Checksum(0).write(&[6]).write(&n.to_le_bytes()),
            N::Short(n) => Checksum(0).write(&[7]).write(&n.to_le_bytes()),
            N::Int(n) => Checksum(0).write(&[8]).write(&n.to_le_bytes()),
            N::Long(n) => Checksum(0).write(&[9]).write(&n.to_le_bytes()),
            N::Float(n) => Checksum(0).write(&[10]).write(&n.to_le_bytes()),
            N::Double(n) => Checksum(0).write(&[11]).write(&n.to_le_bytes()),
        };
        Ok(Self { checksum })
    }
    fn create_boolean(v: bool) -> azalea_codec::error::WriteResult<Self> {
        let checksum = Checksum(0).write(&[13, v as u8]);
        Ok(Self { checksum })
    }
    fn create_map(map: &[(Self, Self)]) -> azalea_codec::error::WriteResult<Self> {
        let mut checksum = Checksum(0).write(&[2]);

        let mut map = map.to_owned();
        map.sort_by_key(|(k, v)| (k.checksum.0, v.checksum.0));

        for (k, v) in map {
            checksum
                .update(&k.checksum.0.to_le_bytes())
                .update(&v.checksum.0.to_le_bytes());
        }
        checksum.update(&[3]);
        Ok(Self { checksum })
    }
    fn create_list(list: &[Self]) -> azalea_codec::error::WriteResult<Self> {
        let mut checksum = Checksum(0).write(&[4]);
        for i in list {
            checksum.update(&i.checksum.0.to_le_bytes());
        }
        checksum = checksum.write(&[5]);
        Ok(Self { checksum })
    }
    fn create_string(v: &str) -> azalea_codec::error::WriteResult<Self> {
        let mut checksum = Checksum(0).write(&[12]);
        let utf16 = v.encode_utf16().collect::<Box<[u16]>>();
        checksum.update(&(utf16.len() as u32).to_le_bytes());
        for c in utf16 {
            checksum.update(&c.to_le_bytes());
        }
        Ok(Self { checksum })
    }
    fn create_byte_list(v: &[u8]) -> azalea_codec::error::WriteResult<Self> {
        let checksum = Checksum(0).write(&[14]).write(v).write(&[15]);
        Ok(Self { checksum })
    }
    fn create_int_list(v: &[i32]) -> azalea_codec::error::WriteResult<Self> {
        let mut checksum = Checksum(0).write(&[16]);
        for i in v {
            checksum.update(&i.to_le_bytes());
        }
        checksum.update(&[17]);
        Ok(Self { checksum })
    }
    fn create_long_list(v: &[i64]) -> azalea_codec::error::WriteResult<Self> {
        let mut checksum = Checksum(0).write(&[18]);
        for i in v {
            checksum.update(&i.to_le_bytes());
        }
        checksum.update(&[19]);
        Ok(Self { checksum })
    }
    fn create_resolvable_item<I: azalea_codec::ResolvableItem>(
        i: I,
    ) -> azalea_codec::error::WriteResult<Self> {
        // let value = self
        //     .registries
        //     .protocol_id_to_identifier(Identifier::from(name), variant_index)
        //     .map(|v| v.to_string())
        //     .unwrap_or_default();
        // self.serialize_str(&value)?;
        // return Ok(());

        Err(azalea_codec::error::WriteError::unimplemented())
    }
}

impl<O: azalea_codec::Ops> Codec<O> for ChecksumOps {
    fn codec_read(_: &O) -> azalea_codec::error::ReadResult<Self> {
        unimplemented!()
    }
    fn codec_write(&self) -> azalea_codec::error::WriteResult<O> {
        unimplemented!()
    }
}

#[derive(Debug, Error)]
#[error("Checksum serialization error")]
pub struct ChecksumError;
impl ser::Error for ChecksumError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        eprintln!("Serialization error: {msg}");
        ChecksumError
    }
}

pub fn get_checksum<T: Codec<ChecksumOps> + ?Sized>(
    value: &T,
    registries: &RegistryHolder,
) -> azalea_codec::error::WriteResult<Checksum> {
    value.codec_write().map(|c| c.checksum)
}
