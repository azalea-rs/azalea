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

pub trait McBufWritable {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

pub trait McBufVarWritable {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

impl McBufWritable for i32 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_i32::<BigEndian>(buf, *self)
    }
}

impl McBufVarWritable for i32 {
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

impl McBufWritable for UnsizedByteArray {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_all(self)
    }
}

impl<T: McBufWritable> McBufWritable for Vec<T> {
    default fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self[..].azalea_write(buf)
    }
}

impl<T: McBufWritable> McBufWritable for [T] {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (self.len() as u32).azalea_write_var(buf)?;
        for item in self {
            T::azalea_write(item, buf)?;
        }
        Ok(())
    }
}

impl<K: McBufWritable, V: McBufWritable> McBufWritable for HashMap<K, V> {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u32::azalea_write_var(&(self.len() as u32), buf)?;
        for (key, value) in self {
            key.azalea_write(buf)?;
            value.azalea_write(buf)?;
        }

        Ok(())
    }
}

impl<K: McBufWritable, V: McBufVarWritable> McBufVarWritable for HashMap<K, V> {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u32::azalea_write_var(&(self.len() as u32), buf)?;
        for (key, value) in self {
            key.azalea_write(buf)?;
            value.azalea_write_var(buf)?;
        }

        Ok(())
    }
}

impl McBufWritable for Vec<u8> {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (self.len() as u32).azalea_write_var(buf)?;
        buf.write_all(self)
    }
}

impl McBufWritable for String {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        write_utf_with_len(buf, self, MAX_STRING_LENGTH.into())
    }
}

impl McBufWritable for &str {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        write_utf_with_len(buf, self, MAX_STRING_LENGTH.into())
    }
}

impl McBufWritable for u32 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i32::azalea_write(&(*self as i32), buf)
    }
}

impl McBufVarWritable for u32 {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i32::azalea_write_var(&(*self as i32), buf)
    }
}

impl McBufVarWritable for i64 {
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

impl McBufVarWritable for u64 {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i64::azalea_write_var(&(*self as i64), buf)
    }
}

impl McBufWritable for u16 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i16::azalea_write(&(*self as i16), buf)
    }
}

impl McBufVarWritable for u16 {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i32::azalea_write_var(&(*self as i32), buf)
    }
}

impl<T: McBufVarWritable> McBufVarWritable for Vec<T> {
    fn azalea_write_var(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u32::azalea_write_var(&(self.len() as u32), buf)?;
        for i in self {
            i.azalea_write_var(buf)?;
        }
        Ok(())
    }
}

impl McBufWritable for u8 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_u8(buf, *self)
    }
}

impl McBufWritable for i16 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_i16::<BigEndian>(buf, *self)
    }
}

impl McBufWritable for i64 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_i64::<BigEndian>(buf, *self)
    }
}

impl McBufWritable for u64 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i64::azalea_write(&(*self as i64), buf)
    }
}

impl McBufWritable for bool {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let byte = u8::from(*self);
        byte.azalea_write(buf)
    }
}

impl McBufWritable for i8 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (*self as u8).azalea_write(buf)
    }
}

impl McBufWritable for f32 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_f32::<BigEndian>(buf, *self)
    }
}

impl McBufWritable for f64 {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_f64::<BigEndian>(buf, *self)
    }
}

impl<T: McBufWritable> McBufWritable for Option<T> {
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

impl<T: McBufVarWritable> McBufVarWritable for Option<T> {
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
impl<T: McBufWritable, const N: usize> McBufWritable for [T; N] {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        for i in self {
            i.azalea_write(buf)?;
        }
        Ok(())
    }
}

impl McBufWritable for simdnbt::owned::NbtTag {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut data = Vec::new();
        self.write(&mut data);
        buf.write_all(&data)
    }
}

impl McBufWritable for simdnbt::owned::NbtCompound {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut data = Vec::new();
        simdnbt::owned::NbtTag::Compound(self.clone()).write(&mut data);
        buf.write_all(&data)
    }
}

impl McBufWritable for simdnbt::owned::Nbt {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut data = Vec::new();
        self.write_unnamed(&mut data);
        buf.write_all(&data)
    }
}

impl<T> McBufWritable for Box<T>
where
    T: McBufWritable,
{
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        T::azalea_write(&**self, buf)
    }
}
