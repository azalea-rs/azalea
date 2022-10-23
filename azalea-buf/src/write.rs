use super::{UnsizedByteArray, MAX_STRING_LENGTH};
use byteorder::{BigEndian, WriteBytesExt};
use std::{collections::HashMap, io::Write};

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
    string.as_bytes().to_vec().write_into(buf)?;
    Ok(())
}

pub trait McBufWritable {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

pub trait McBufVarWritable {
    fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

impl McBufWritable for i32 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_i32::<BigEndian>(buf, *self)
    }
}

impl McBufVarWritable for i32 {
    fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut buffer = [0];
        let mut value = *self;
        if value == 0 {
            buf.write_all(&buffer).unwrap();
        }
        while value != 0 {
            buffer[0] = (value & 0b0111_1111) as u8;
            value = (value >> 7) & (i32::max_value() >> 6);
            if value != 0 {
                buffer[0] |= 0b1000_0000;
            }
            buf.write_all(&buffer)?;
        }
        Ok(())
    }
}

impl McBufWritable for UnsizedByteArray {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_all(self)
    }
}

impl<T: McBufWritable> McBufWritable for Vec<T> {
    default fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self[..].write_into(buf)
    }
}

impl<T: McBufWritable> McBufWritable for [T] {
    default fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (self.len() as u32).var_write_into(buf)?;
        for item in self {
            T::write_into(item, buf)?;
        }
        Ok(())
    }
}

impl<K: McBufWritable, V: McBufWritable> McBufWritable for HashMap<K, V> {
    default fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u32::var_write_into(&(self.len() as u32), buf)?;
        for (key, value) in self {
            key.write_into(buf)?;
            value.write_into(buf)?;
        }

        Ok(())
    }
}

impl<K: McBufWritable, V: McBufVarWritable> McBufVarWritable for HashMap<K, V> {
    default fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u32::var_write_into(&(self.len() as u32), buf)?;
        for (key, value) in self {
            key.write_into(buf)?;
            value.var_write_into(buf)?;
        }

        Ok(())
    }
}

impl McBufWritable for Vec<u8> {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (self.len() as u32).var_write_into(buf)?;
        buf.write_all(self)
    }
}

impl McBufWritable for String {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        write_utf_with_len(buf, self, MAX_STRING_LENGTH.into())
    }
}

impl McBufWritable for &str {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        write_utf_with_len(buf, self, MAX_STRING_LENGTH.into())
    }
}

impl McBufWritable for u32 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i32::write_into(&(*self as i32), buf)
    }
}

impl McBufVarWritable for u32 {
    fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i32::var_write_into(&(*self as i32), buf)
    }
}

impl McBufVarWritable for i64 {
    fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut buffer = [0];
        let mut value = *self;
        while value != 0 {
            buffer[0] = (value & 0b0111_1111) as u8;
            value = (value >> 7) & (i64::max_value() >> 6);
            if value != 0 {
                buffer[0] |= 0b1000_0000;
            }
            // this only writes a single byte, so write_all isn't necessary
            // the let _ = is so clippy doesn't complain
            let _ = buf.write(&buffer)?;
        }
        Ok(())
    }
}

impl McBufVarWritable for u64 {
    fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i64::var_write_into(&(*self as i64), buf)
    }
}

impl McBufWritable for u16 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i16::write_into(&(*self as i16), buf)
    }
}

impl McBufVarWritable for u16 {
    fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i32::var_write_into(&(*self as i32), buf)
    }
}

impl<T: McBufVarWritable> McBufVarWritable for Vec<T> {
    fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u32::var_write_into(&(self.len() as u32), buf)?;
        for i in self {
            i.var_write_into(buf)?;
        }
        Ok(())
    }
}

impl McBufWritable for u8 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_u8(buf, *self)
    }
}

impl McBufWritable for i16 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_i16::<BigEndian>(buf, *self)
    }
}

impl McBufWritable for i64 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_i64::<BigEndian>(buf, *self)
    }
}

impl McBufWritable for u64 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i64::write_into(&(*self as i64), buf)
    }
}

impl McBufWritable for bool {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let byte = u8::from(*self);
        byte.write_into(buf)
    }
}

impl McBufWritable for i8 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (*self as u8).write_into(buf)
    }
}

impl McBufWritable for f32 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_f32::<BigEndian>(buf, *self)
    }
}

impl McBufWritable for f64 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        WriteBytesExt::write_f64::<BigEndian>(buf, *self)
    }
}

impl<T: McBufWritable> McBufWritable for Option<T> {
    default fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        if let Some(s) = self {
            true.write_into(buf)?;
            s.write_into(buf)?;
        } else {
            false.write_into(buf)?;
        };
        Ok(())
    }
}

impl<T: McBufVarWritable> McBufVarWritable for Option<T> {
    default fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        if let Some(s) = self {
            true.write_into(buf)?;
            s.var_write_into(buf)?;
        } else {
            false.write_into(buf)?;
        };
        Ok(())
    }
}

// [T; N]
impl<T: McBufWritable, const N: usize> McBufWritable for [T; N] {
    default fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        for i in self {
            i.write_into(buf)?;
        }
        Ok(())
    }
}
