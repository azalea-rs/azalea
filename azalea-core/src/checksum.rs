use std::{any::Any, cmp::Ordering, fmt, hash::Hasher};

use crc32c::Crc32cHasher;
use serde::{Serialize, ser};
use simdnbt::owned::{NbtCompound, NbtList, NbtTag};
use thiserror::Error;
use tracing::error;

use crate::{registry_holder::RegistryHolder, resource_location::ResourceLocation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct HashCode(pub u32);

pub struct ChecksumSerializer<'a, 'r> {
    hasher: &'a mut Crc32cHasher,
    registries: &'r RegistryHolder,
}
impl<'a, 'r> ChecksumSerializer<'a, 'r> {
    pub fn checksum(&mut self) -> HashCode {
        HashCode(self.hasher.finish() as u32)
    }
}

impl<'a, 'r> ser::Serializer for ChecksumSerializer<'a, 'r> {
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = ChecksumError;

    type SerializeSeq = ChecksumListSerializer<'a, 'r>;
    type SerializeTuple = ChecksumListSerializer<'a, 'r>;
    type SerializeTupleStruct = ChecksumListSerializer<'a, 'r>;
    type SerializeTupleVariant = ChecksumMapSerializer<'a, 'r>;
    type SerializeMap = ChecksumMapSerializer<'a, 'r>;
    type SerializeStruct = ChecksumMapSerializer<'a, 'r>;
    type SerializeStructVariant = ChecksumMapSerializer<'a, 'r>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        self.hasher.write_u8(13);
        self.hasher.write(&[v as u8]);
        Ok(())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        assert!(self.hasher.finish() == 0);
        Ok(ChecksumMapSerializer {
            hasher: self.hasher,
            registries: self.registries,
            entries: Vec::new(),
        })
    }
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        assert!(self.hasher.finish() == 0);
        self.serialize_map(Some(len))
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        self.hasher.write_u8(6);
        self.hasher.write(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        self.hasher.write_u8(7);
        self.hasher.write(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        self.hasher.write_u8(8);
        self.hasher.write(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        self.hasher.write_u8(9);
        self.hasher.write(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        self.serialize_i8(v as i8)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        self.serialize_i16(v as i16)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        self.serialize_i32(v as i32)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        self.serialize_i64(v as i64)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        self.hasher.write_u8(10);
        self.hasher.write(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        self.hasher.write_u8(11);
        self.hasher.write(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        self.serialize_u32(v as u32)
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        self.hasher.write_u8(12);
        let utf16 = v.encode_utf16().collect::<Vec<_>>();
        self.hasher.write(&(utf16.len() as u32).to_le_bytes());
        for c in utf16 {
            self.hasher.write(&c.to_le_bytes());
        }
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        self.hasher.write_u8(14);
        self.hasher.write(v);
        self.hasher.write_u8(15);
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        println!("serialize none");
        self.hasher.write_u8(1);
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // check if t

        value.serialize(self)?;
        Ok(())
    }

    fn serialize_unit(self) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // we can't have custom handlers with serde's traits, so we use this silly hack
        // to make serializing data-driven registries work
        if name.starts_with("minecraft:") {
            let value = self
                .registries
                .map
                .get(&ResourceLocation::from(name))
                .and_then(|r| r.get_index(variant_index as usize))
                .map(|r| r.0.to_string())
                .unwrap_or_default();
            self.serialize_str(&value)?;
            return Ok(());
        }

        value.serialize(ChecksumSerializer {
            hasher: self.hasher,
            registries: self.registries,
        })
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        assert!(self.hasher.finish() == 0);
        println!("serialize seq with len: {:?}", len);
        Ok(ChecksumListSerializer {
            hasher: self.hasher,
            registries: self.registries,
            values: Vec::with_capacity(len.unwrap_or_default()),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        assert!(self.hasher.finish() == 0);
        Ok(ChecksumListSerializer {
            hasher: self.hasher,
            registries: self.registries,
            values: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(ChecksumMapSerializer {
            hasher: self.hasher,
            registries: self.registries,
            entries: Vec::with_capacity(len),
        })
    }
}

pub struct ChecksumListSerializer<'a, 'r> {
    hasher: &'a mut Crc32cHasher,
    registries: &'r RegistryHolder,
    values: Vec<HashCode>,
}
impl<'a, 'r> ser::SerializeSeq for ChecksumListSerializer<'a, 'r> {
    type Ok = ();
    type Error = ChecksumError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // elements are hashed individually
        self.values.push(get_checksum(value, self.registries)?);
        Ok(())
    }

    fn end(self) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        update_hasher_for_list(self.hasher, &self.values);
        Ok(())
    }
}
impl<'a, 'r> ser::SerializeTuple for ChecksumListSerializer<'a, 'r> {
    type Ok = ();
    type Error = ChecksumError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // elements are hashed individually
        self.values.push(get_checksum(value, self.registries)?);
        Ok(())
    }

    fn end(self) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        update_hasher_for_list(self.hasher, &self.values);
        Ok(())
    }
}
impl<'a, 'r> ser::SerializeTupleStruct for ChecksumListSerializer<'a, 'r> {
    type Ok = ();
    type Error = ChecksumError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // elements are hashed individually
        self.values.push(get_checksum(value, self.registries)?);
        Ok(())
    }

    fn end(self) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        update_hasher_for_list(self.hasher, &self.values);
        Ok(())
    }
}

pub struct ChecksumMapSerializer<'a, 'r> {
    // this is only written to at the end
    hasher: &'a mut Crc32cHasher,
    registries: &'r RegistryHolder,
    // we have to keep track of the elements like this because they're sorted at the end
    entries: Vec<(HashCode, HashCode)>,
}
impl<'a, 'r> ser::SerializeMap for ChecksumMapSerializer<'a, 'r> {
    type Ok = ();
    type Error = ChecksumError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // this 0 is a placeholder
        self.entries
            .push((get_checksum(key, self.registries)?, HashCode(0)));
        Ok(())
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // placeholder gets replaced here
        self.entries
            .last_mut()
            .expect("entry should've already been added")
            .1 = get_checksum(value, self.registries)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        update_hasher_for_map(self.hasher, &self.entries);
        Ok(())
    }
}
impl<'a, 'r> ser::SerializeTupleVariant for ChecksumMapSerializer<'a, 'r> {
    type Ok = ();
    type Error = ChecksumError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // TODO
        error!("tuple variants are not supported when serializing checksums");
        Ok(())
    }

    fn end(self) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        Ok(())
    }
}
impl<'a, 'r> ser::SerializeStruct for ChecksumMapSerializer<'a, 'r> {
    type Ok = ();
    type Error = ChecksumError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.entries.push((
            get_checksum(key, self.registries)?,
            get_checksum(value, self.registries)?,
        ));
        Ok(())
    }

    fn end(self) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        update_hasher_for_map(self.hasher, &self.entries);
        Ok(())
    }
}
impl<'a, 'r> ser::SerializeStructVariant for ChecksumMapSerializer<'a, 'r> {
    type Ok = ();
    type Error = ChecksumError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.entries.push((
            get_checksum(key, self.registries)?,
            get_checksum(value, self.registries)?,
        ));
        Ok(())
    }

    fn end(self) -> Result<()> {
        assert!(self.hasher.finish() == 0);
        update_hasher_for_map(self.hasher, &self.entries);
        Ok(())
    }
}

#[derive(Error, Debug)]
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
type Result<T> = std::result::Result<T, ChecksumError>;

pub fn get_checksum<T: Serialize + ?Sized>(
    value: &T,
    registries: &RegistryHolder,
) -> Result<HashCode> {
    let mut hasher = Crc32cHasher::default();
    value.serialize(ChecksumSerializer {
        hasher: &mut hasher,
        registries,
    })?;
    Ok(HashCode(hasher.finish() as u32))
}

fn update_hasher_for_list(h: &mut Crc32cHasher, values: &[HashCode]) {
    h.write_u8(4);
    for v in values {
        h.write(&v.0.to_le_bytes());
    }
    h.write_u8(5);
}
fn update_hasher_for_map(h: &mut Crc32cHasher, entries: &[(HashCode, HashCode)]) {
    println!("getting checksum for map with {} entries", entries.len());
    h.write_u8(2);
    let mut entries = entries.to_vec();
    entries.sort_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Equal => a.1.cmp(&b.1),
        other => other,
    });
    for (k, v) in entries {
        h.write(&k.0.to_le_bytes());
        h.write(&v.0.to_le_bytes());
    }
    h.write_u8(3);
}

// impl AzaleaChecksum for i8 {
//     fn azalea_checksum(&self) -> HashCode {
//         let mut h = Crc32cHasher::default();
//         h.write_u8(6);
//         h.write(&self.to_le_bytes());
//         HashCode(h.finish() as u32)
//     }
// }
// impl AzaleaChecksum for i16 {
//     fn azalea_checksum(&self) -> HashCode {
//         let mut h = Crc32cHasher::default();
//         h.write_u8(7);
//         h.write(&self.to_le_bytes());
//         HashCode(h.finish() as u32)
//     }
// }
// impl AzaleaChecksum for i32 {
//     fn azalea_checksum(&self) -> HashCode {
//         let mut h = Crc32cHasher::default();
//         h.write_u8(8);
//         h.write(&self.to_le_bytes());
//         HashCode(h.finish() as u32)
//     }
// }
// impl AzaleaChecksum for i64 {
//     fn azalea_checksum(&self) -> HashCode {
//         let mut h = Crc32cHasher::default();
//         h.write_u8(9);
//         h.write(&self.to_le_bytes());
//         HashCode(h.finish() as u32)
//     }
// }
// impl AzaleaChecksum for f32 {
//     fn azalea_checksum(&self) -> HashCode {
//         let mut h = Crc32cHasher::default();
//         h.write_u8(10);
//         h.write(&self.to_le_bytes());
//         HashCode(h.finish() as u32)
//     }
// }
// impl AzaleaChecksum for f64 {
//     fn azalea_checksum(&self) -> HashCode {
//         let mut h = Crc32cHasher::default();
//         h.write_u8(11);
//         h.write(&self.to_le_bytes());
//         HashCode(h.finish() as u32)
//     }
// }
// impl AzaleaChecksum for &str {
//     fn azalea_checksum(&self) -> HashCode {
//         println!("doing checksum for str: {self:?}");
//         let mut h = Crc32cHasher::default();
//         h.write_u8(12);
//         h.write(&(self.len() as u32).to_le_bytes());
//         h.write(&self.as_bytes());
//         HashCode(h.finish() as u32)
//     }
// }
// impl AzaleaChecksum for String {
//     fn azalea_checksum(&self) -> HashCode {
//         println!("doing checksum for String: {self:?}");
//         let mut h = Crc32cHasher::default();
//         h.write_u8(12);

//         let utf16 = self.encode_utf16().collect::<Vec<_>>();
//         h.write(&(utf16.len() as u32).to_le_bytes());
//         for c in utf16 {
//             h.write(&c.to_le_bytes());
//         }

//         println!("doing checksum for string: {self:?}");
//         HashCode(h.finish() as u32)
//     }
// }
// impl AzaleaChecksum for bool {
//     fn azalea_checksum(&self) -> HashCode {
//         println!("doing checksum for bool: {self:?}");
//         let mut h = Crc32cHasher::default();
//         h.write_u8(13);
//         h.write_u8(*self as u8);
//         HashCode(h.finish() as u32)
//     }
// }
// impl AzaleaChecksum for Vec<u8> {
//     fn azalea_checksum(&self) -> HashCode {
//         let mut h = Crc32cHasher::default();
//         h.write_u8(14);
//         h.write(self);
//         h.write_u8(15);

//         HashCode(h.finish() as u32)
//     }
// }
// impl AzaleaChecksum for Vec<i8> {
//     fn azalea_checksum(&self) -> HashCode {
//         let mut h = Crc32cHasher::default();
//         h.write_u8(14);
//         for item in self {
//             h.write(&[*item as u8]);
//         }
//         h.write_u8(15);

//         HashCode(h.finish() as u32)
//     }
// }
// impl AzaleaChecksum for Vec<u32> {
//     fn azalea_checksum(&self) -> HashCode {
//         let mut h = Crc32cHasher::default();
//         h.write_u8(16);
//         for item in self {
//             h.write(&item.to_le_bytes());
//         }
//         h.write_u8(17);

//         HashCode(h.finish() as u32)
//     }
// }
// impl AzaleaChecksum for Vec<i32> {
//     fn azalea_checksum(&self) -> HashCode {
//         let mut h = Crc32cHasher::default();
//         h.write_u8(16);
//         for item in self {
//             h.write(&item.to_le_bytes());
//         }
//         h.write_u8(17);

//         HashCode(h.finish() as u32)
//     }
// }
// impl AzaleaChecksum for Vec<u64> {
//     fn azalea_checksum(&self) -> HashCode {
//         let mut h = Crc32cHasher::default();
//         h.write_u8(18);
//         for item in self {
//             h.write(&item.to_le_bytes());
//         }
//         h.write_u8(19);

//         HashCode(h.finish() as u32)
//     }
// }
// impl AzaleaChecksum for Vec<i64> {
//     fn azalea_checksum(&self) -> HashCode {
//         let mut h = Crc32cHasher::default();
//         h.write_u8(18);
//         for item in self {
//             h.write(&item.to_le_bytes());
//         }
//         h.write_u8(19);

//         HashCode(h.finish() as u32)
//     }
// }
