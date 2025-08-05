use std::{cmp::Ordering, hash::Hasher};

use crc32c::Crc32cHasher;
use simdnbt::owned::{NbtCompound, NbtList, NbtTag};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HashCode(pub u32);

pub trait AzaleaChecksum {
    /// Create a crc32 checksum of the data in this type.
    ///
    /// This is currently used in the Minecraft protocol when the client sends
    /// item components.
    fn azalea_checksum(&self) -> HashCode;
}

impl AzaleaChecksum for Vec<(HashCode, HashCode)> {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(2);

        let mut map = self.clone();
        map.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Equal => a.1.cmp(&b.1),
            other => other,
        });
        for (k, v) in map {
            h.write(&k.0.to_le_bytes());
            h.write(&v.0.to_le_bytes());
        }
        h.write_u8(3);

        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for Vec<HashCode> {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(4);
        for v in self {
            h.write(&v.0.to_le_bytes());
        }
        h.write_u8(5);

        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for i8 {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(6);
        h.write(&self.to_le_bytes());
        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for i16 {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(7);
        h.write(&self.to_le_bytes());
        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for i32 {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(8);
        h.write(&self.to_le_bytes());
        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for i64 {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(9);
        h.write(&self.to_le_bytes());
        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for f32 {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(10);
        h.write(&self.to_le_bytes());
        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for f64 {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(11);
        h.write(&self.to_le_bytes());
        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for &str {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(12);
        h.write(&(self.len() as u32).to_le_bytes());
        h.write(&self.as_bytes());
        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for String {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(12);

        let utf16 = self.encode_utf16().collect::<Vec<_>>();
        h.write(&(utf16.len() as u32).to_le_bytes());
        for c in utf16 {
            h.write(&c.to_le_bytes());
        }

        println!("doing checksum for string: {self:?}");
        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for bool {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(13);
        h.write_u8(*self as u8);
        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for Vec<u8> {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(14);
        h.write(self);
        h.write_u8(15);

        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for Vec<i8> {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(14);
        for item in self {
            h.write(&[*item as u8]);
        }
        h.write_u8(15);

        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for Vec<u32> {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(16);
        for item in self {
            h.write(&item.to_le_bytes());
        }
        h.write_u8(17);

        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for Vec<i32> {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(16);
        for item in self {
            h.write(&item.to_le_bytes());
        }
        h.write_u8(17);

        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for Vec<u64> {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(18);
        for item in self {
            h.write(&item.to_le_bytes());
        }
        h.write_u8(19);

        HashCode(h.finish() as u32)
    }
}
impl AzaleaChecksum for Vec<i64> {
    fn azalea_checksum(&self) -> HashCode {
        let mut h = Crc32cHasher::default();
        h.write_u8(18);
        for item in self {
            h.write(&item.to_le_bytes());
        }
        h.write_u8(19);

        HashCode(h.finish() as u32)
    }
}

impl AzaleaChecksum for NbtTag {
    // the structure of nbt is relatively similar to the hashed format, so we can
    // convert to it like this
    fn azalea_checksum(&self) -> HashCode {
        match self {
            NbtTag::Byte(v) => v.azalea_checksum(),
            NbtTag::Short(v) => v.azalea_checksum(),
            NbtTag::Int(v) => v.azalea_checksum(),
            NbtTag::Long(v) => v.azalea_checksum(),
            NbtTag::Float(v) => v.azalea_checksum(),
            NbtTag::Double(v) => v.azalea_checksum(),
            NbtTag::ByteArray(v) => v.azalea_checksum(),
            NbtTag::String(v) => v.as_str().to_str().as_ref().azalea_checksum(),
            NbtTag::List(v) => v.azalea_checksum(),
            NbtTag::Compound(v) => v.azalea_checksum(),
            NbtTag::IntArray(v) => v.azalea_checksum(),
            NbtTag::LongArray(v) => v.azalea_checksum(),
        }
    }
}
impl AzaleaChecksum for NbtCompound {
    fn azalea_checksum(&self) -> HashCode {
        self.iter()
            .map(|(k, v)| (k.to_str().as_ref().azalea_checksum(), v.azalea_checksum()))
            .collect::<Vec<_>>()
            .azalea_checksum()
    }
}
impl AzaleaChecksum for NbtList {
    fn azalea_checksum(&self) -> HashCode {
        self.as_nbt_tags()
            .iter()
            .map(|v| v.azalea_checksum())
            .collect::<Vec<_>>()
            .azalea_checksum()
    }
}
