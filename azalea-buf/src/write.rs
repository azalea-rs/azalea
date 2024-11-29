use std::{collections::HashMap, io::Write};

use byteorder::{BigEndian, WriteBytesExt};

use super::{UnsizedByteArray, MAX_STRING_LENGTH};

fn write_utf_with_len(
    buf: &mut impl Write,
    string: &str,
    len: usize,
) -> Result<(), std::io::Error> {
    if string.len() > len {
        panic!(
            "String too big (was {} bytes encoded, max {})",
            string.len(),
            len
        );
    }
    string.as_bytes().to_vec().azalea_write(buf)?;
    Ok(())
}

pub trait AzaleaWrite {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

pub trait AzaleaWriteVar {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

impl AzaleaWrite for i32 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_i32::<BigEndian>(buf, *self)
    }
}

impl AzaleaWriteVar for i32 {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut buffer = [0];
        let mut value = *self;
        if value == 0 {
            buf.write_all(&buffer).unwrap();
        }
        while value != 0 {
            buffer[0] = (value & 0b0111_1111) as u8;
            value = (value >> 7) & (i32::MAX >> 6);
            if value != 0 {
                buffer[0] |= 0b1000_0000;
            }
            buf.write_all(&buffer)?;
        }
        Ok(())
    }
}

impl AzaleaWrite for UnsizedByteArray {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_all(self)
    }
}

impl<T: AzaleaWrite> AzaleaWrite for Vec<T> {
    default fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self[..].azalea_write(buf)
    }
}

impl<T: AzaleaWrite> AzaleaWrite for [T] {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (self.len() as u32).azalea_write_var(buf)?;
        for item in self {
            T::azalea_write(item, buf)?;
        }
        Ok(())
    }
}

impl<K: AzaleaWrite, V: AzaleaWrite> AzaleaWrite for HashMap<K, V> {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u32::azalea_write_var(&(self.len() as u32), buf)?;
        for (key, value) in self {
            key.azalea_write(buf)?;
            value.azalea_write(buf)?;
        }

        Ok(())
    }
}

impl<K: AzaleaWrite, V: AzaleaWriteVar> AzaleaWriteVar for HashMap<K, V> {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u32::azalea_write_var(&(self.len() as u32), buf)?;
        for (key, value) in self {
            key.azalea_write(buf)?;
            value.azalea_write_var(buf)?;
        }

        Ok(())
    }
}

impl AzaleaWrite for Vec<u8> {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (self.len() as u32).azalea_write_var(buf)?;
        buf.write_all(self)
    }
}

impl AzaleaWrite for String {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        write_utf_with_len(buf, self, MAX_STRING_LENGTH.into())
    }
}

impl AzaleaWrite for &str {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        write_utf_with_len(buf, self, MAX_STRING_LENGTH.into())
    }
}

impl AzaleaWrite for u32 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i32::azalea_write(&(*self as i32), buf)
    }
}

impl AzaleaWriteVar for u32 {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i32::azalea_write_var(&(*self as i32), buf)
    }
}

impl AzaleaWriteVar for i64 {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut buffer = [0];
        let mut value = *self;
        if value == 0 {
            buf.write_all(&buffer).unwrap();
        }
        while value != 0 {
            buffer[0] = (value & 0b0111_1111) as u8;
            value = (value >> 7) & (i64::MAX >> 6);
            if value != 0 {
                buffer[0] |= 0b1000_0000;
            }
            buf.write_all(&buffer)?;
        }
        Ok(())
    }
}

impl AzaleaWriteVar for u64 {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i64::azalea_write_var(&(*self as i64), buf)
    }
}

impl AzaleaWrite for u16 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i16::azalea_write(&(*self as i16), buf)
    }
}

impl AzaleaWriteVar for u16 {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i32::azalea_write_var(&(*self as i32), buf)
    }
}

impl<T: AzaleaWriteVar> AzaleaWriteVar for Vec<T> {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u32::azalea_write_var(&(self.len() as u32), buf)?;
        for i in self {
            i.azalea_write_var(buf)?;
        }
        Ok(())
    }
}

impl AzaleaWrite for u8 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_u8(buf, *self)
    }
}

impl AzaleaWrite for i16 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_i16::<BigEndian>(buf, *self)
    }
}

impl AzaleaWrite for i64 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_i64::<BigEndian>(buf, *self)
    }
}

impl AzaleaWrite for u64 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i64::azalea_write(&(*self as i64), buf)
    }
}

impl AzaleaWrite for bool {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let byte = u8::from(*self);
        byte.azalea_write(buf)
    }
}

impl AzaleaWrite for i8 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (*self as u8).azalea_write(buf)
    }
}

impl AzaleaWrite for f32 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_f32::<BigEndian>(buf, *self)
    }
}

impl AzaleaWrite for f64 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_f64::<BigEndian>(buf, *self)
    }
}

impl<T: AzaleaWrite> AzaleaWrite for Option<T> {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        if let Some(s) = self {
            true.azalea_write(buf)?;
            s.azalea_write(buf)?;
        } else {
            false.azalea_write(buf)?;
        };
        Ok(())
    }
}

impl<T: AzaleaWriteVar> AzaleaWriteVar for Option<T> {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        if let Some(s) = self {
            true.azalea_write(buf)?;
            s.azalea_write_var(buf)?;
        } else {
            false.azalea_write(buf)?;
        };
        Ok(())
    }
}

// [T; N]
impl<T: AzaleaWrite, const N: usize> AzaleaWrite for [T; N] {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        for i in self {
            i.azalea_write(buf)?;
        }
        Ok(())
    }
}

impl AzaleaWrite for simdnbt::owned::NbtTag {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut data = Vec::new();
        self.write(&mut data);
        buf.write_all(&data)
    }
}

impl AzaleaWrite for simdnbt::owned::NbtCompound {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut data = Vec::new();
        simdnbt::owned::NbtTag::Compound(self.clone()).write(&mut data);
        buf.write_all(&data)
    }
}

impl AzaleaWrite for simdnbt::owned::Nbt {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut data = Vec::new();
        self.write_unnamed(&mut data);
        buf.write_all(&data)
    }
}

impl<T> AzaleaWrite for Box<T>
where
    T: AzaleaWrite,
{
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        T::azalea_write(&**self, buf)
    }
}
