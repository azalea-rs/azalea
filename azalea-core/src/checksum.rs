use std::{cmp::Ordering, fmt, hash::Hasher};

use azalea_buf::AzBuf;
use crc32c::Crc32cHasher;
use serde::{Serialize, ser};
use thiserror::Error;
use tracing::error;

use crate::{identifier::Identifier, registry_holder::RegistryHolder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, AzBuf)]
pub struct Checksum(pub u32);

pub struct ChecksumSerializer<'a, 'r> {
    hasher: &'a mut Crc32cHasher,
    registries: &'r RegistryHolder,
}
impl<'a, 'r> ChecksumSerializer<'a, 'r> {
    pub fn checksum(&mut self) -> Checksum {
        Checksum(self.hasher.finish() as u32)
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
        update_hasher_for_map(self.hasher, &[]);
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
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
                .get(&Identifier::from(name))
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
        Ok(ChecksumListSerializer {
            hasher: self.hasher,
            registries: self.registries,
            values: Vec::with_capacity(len.unwrap_or_default()),
            list_kind: ListKind::Normal,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        assert!(self.hasher.finish() == 0);
        Ok(ChecksumListSerializer {
            hasher: self.hasher,
            registries: self.registries,
            values: Vec::with_capacity(len),
            list_kind: ListKind::Normal,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        assert!(self.hasher.finish() == 0);
        let list_kind = if name == "azalea:int_array" {
            self.hasher.write_u8(16);
            ListKind::Int
        } else if name == "azalea:long_array" {
            self.hasher.write_u8(18);
            ListKind::Long
        } else {
            ListKind::Normal
        };
        Ok(ChecksumListSerializer {
            hasher: self.hasher,
            registries: self.registries,
            values: Vec::with_capacity(len),
            list_kind,
        })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        assert!(self.hasher.finish() == 0);
        Ok(ChecksumMapSerializer {
            hasher: self.hasher,
            registries: self.registries,
            entries: Vec::with_capacity(len),
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
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
    values: Vec<Checksum>,
    /// If you set this to not be the default, you should also update the hasher
    /// before creating the list serializer.
    list_kind: ListKind,
}
impl<'a, 'r> ser::SerializeSeq for ChecksumListSerializer<'a, 'r> {
    type Ok = ();
    type Error = ChecksumError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.list_kind == ListKind::Normal {
            // elements are hashed individually
            self.values.push(get_checksum(value, self.registries)?);
        } else {
            value.serialize(IntOrLongArrayChecksumSerializer {
                hasher: self.hasher,
            })?;
        }

        Ok(())
    }

    fn end(self) -> Result<()> {
        match self.list_kind {
            ListKind::Normal => {
                assert!(self.hasher.finish() == 0);
                update_hasher_for_list(self.hasher, &self.values);
            }
            ListKind::Int => {
                self.hasher.write_u8(17);
            }
            ListKind::Long => {
                self.hasher.write_u8(19);
            }
        }

        Ok(())
    }
}
/// Minecraft sometimes serializes u8/i32/i64 lists differently, so we have to
/// keep track of that when serializing the arrays.
///
/// Byte arrays aren't included here as they're handled with `serialize_bytes`.
#[derive(Default, PartialEq, Eq)]
enum ListKind {
    #[default]
    Normal,
    Int,
    Long,
}

impl<'a, 'r> ser::SerializeTuple for ChecksumListSerializer<'a, 'r> {
    type Ok = ();
    type Error = ChecksumError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}
impl<'a, 'r> ser::SerializeTupleStruct for ChecksumListSerializer<'a, 'r> {
    type Ok = ();
    type Error = ChecksumError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

pub struct ChecksumMapSerializer<'a, 'r> {
    // this is only written to at the end
    hasher: &'a mut Crc32cHasher,
    registries: &'r RegistryHolder,
    // we have to keep track of the elements like this because they're sorted at the end
    entries: Vec<(Checksum, Checksum)>,
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
            .push((get_checksum(key, self.registries)?, Checksum(0)));
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

/// A hasher that can only serialize i32 and i64.
struct IntOrLongArrayChecksumSerializer<'a> {
    hasher: &'a mut Crc32cHasher,
}
impl<'a> ser::Serializer for IntOrLongArrayChecksumSerializer<'a> {
    type Ok = ();
    type Error = ChecksumError;
    // unused
    type SerializeSeq = ChecksumListSerializer<'a, 'a>;
    type SerializeTuple = ChecksumListSerializer<'a, 'a>;
    type SerializeTupleStruct = ChecksumListSerializer<'a, 'a>;
    type SerializeTupleVariant = ChecksumMapSerializer<'a, 'a>;
    type SerializeMap = ChecksumMapSerializer<'a, 'a>;
    type SerializeStruct = ChecksumMapSerializer<'a, 'a>;
    type SerializeStructVariant = ChecksumMapSerializer<'a, 'a>;

    fn serialize_bool(self, _v: bool) -> Result<()> {
        unimplemented!()
    }
    fn serialize_i8(self, _v: i8) -> Result<()> {
        unimplemented!()
    }
    fn serialize_i16(self, _v: i16) -> Result<()> {
        unimplemented!()
    }
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.hasher.write(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.hasher.write(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_u8(self, _v: u8) -> Result<()> {
        unimplemented!()
    }
    fn serialize_u16(self, _v: u16) -> Result<()> {
        unimplemented!()
    }
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_i32(v as i32)
    }
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_i64(v as i64)
    }
    fn serialize_f32(self, _v: f32) -> Result<()> {
        unimplemented!()
    }
    fn serialize_f64(self, _v: f64) -> Result<()> {
        unimplemented!()
    }
    fn serialize_char(self, _v: char) -> Result<()> {
        unimplemented!()
    }
    fn serialize_str(self, _v: &str) -> Result<()> {
        unimplemented!()
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        unimplemented!()
    }
    fn serialize_none(self) -> Result<()> {
        unimplemented!()
    }
    fn serialize_some<T>(self, _v: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }
    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }
    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!()
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        unimplemented!()
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
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
) -> Result<Checksum> {
    let mut hasher = Crc32cHasher::default();
    value.serialize(ChecksumSerializer {
        hasher: &mut hasher,
        registries,
    })?;
    Ok(Checksum(hasher.finish() as u32))
}

fn update_hasher_for_list(h: &mut Crc32cHasher, values: &[Checksum]) {
    h.write_u8(4);
    for v in values {
        h.write(&v.0.to_le_bytes());
    }
    h.write_u8(5);
}
fn update_hasher_for_map(h: &mut Crc32cHasher, entries: &[(Checksum, Checksum)]) {
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
