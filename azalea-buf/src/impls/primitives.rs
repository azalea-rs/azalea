use std::io::{self, Cursor, Read, Write};

use byteorder::{BE, ReadBytesExt, WriteBytesExt};
use tracing::warn;

use crate::{AzBuf, AzBufVar, BufReadError};

impl AzBuf for () {
    fn azalea_read(_buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(())
    }
    fn azalea_write(&self, _buf: &mut impl Write) -> io::Result<()> {
        Ok(())
    }
}

impl AzBuf for i32 {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(buf.read_i32::<BE>()?)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        buf.write_i32::<BE>(*self)
    }
}

impl AzBufVar for i32 {
    /// Read a single varint from the reader and return the value
    fn azalea_read_var(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        // fast varint impl based on https://github.com/luojia65/mc-varint/blob/master/src/lib.rs#L67
        let mut buffer = [0];
        let mut ans = 0;
        for i in 0..5 {
            buf.read_exact(&mut buffer)?;
            ans |= ((buffer[0] & 0b0111_1111) as i32) << (7 * i);
            if buffer[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(ans)
    }

    fn azalea_write_var(&self, buf: &mut impl Write) -> io::Result<()> {
        let mut buffer = [0];
        let mut value = *self;
        if value == 0 {
            buf.write_all(&buffer)?;
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

impl AzBufVar for i64 {
    fn azalea_read_var(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut buffer = [0];
        let mut ans = 0;
        for i in 0..10 {
            buf.read_exact(&mut buffer)
                .map_err(|_| BufReadError::InvalidVarLong)?;
            ans |= ((buffer[0] & 0b0111_1111) as i64) << (7 * i);
            if buffer[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(ans)
    }

    fn azalea_write_var(&self, buf: &mut impl Write) -> io::Result<()> {
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

impl AzBufVar for u64 {
    fn azalea_read_var(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        i64::azalea_read_var(buf).map(|i| i as u64)
    }
    fn azalea_write_var(&self, buf: &mut impl Write) -> io::Result<()> {
        i64::azalea_write_var(&(*self as i64), buf)
    }
}

impl AzBuf for u32 {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(i32::azalea_read(buf)? as u32)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        i32::azalea_write(&(*self as i32), buf)
    }
}

impl AzBufVar for u32 {
    fn azalea_read_var(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(i32::azalea_read_var(buf)? as u32)
    }
    fn azalea_write_var(&self, buf: &mut impl Write) -> io::Result<()> {
        i32::azalea_write_var(&(*self as i32), buf)
    }
}

impl AzBuf for u16 {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        i16::azalea_read(buf).map(|i| i as u16)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        i16::azalea_write(&(*self as i16), buf)
    }
}

impl AzBuf for i16 {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(buf.read_i16::<BE>()?)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        buf.write_i16::<BE>(*self)
    }
}

impl AzBufVar for u16 {
    fn azalea_read_var(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(i32::azalea_read_var(buf)? as u16)
    }
    fn azalea_write_var(&self, buf: &mut impl Write) -> io::Result<()> {
        i32::azalea_write_var(&(*self as i32), buf)
    }
}

impl AzBuf for i64 {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(buf.read_i64::<BE>()?)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        buf.write_i64::<BE>(*self)
    }
}

impl AzBuf for u64 {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        i64::azalea_read(buf).map(|i| i as u64)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        buf.write_u64::<BE>(*self)
    }
}

impl AzBuf for bool {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let byte = u8::azalea_read(buf)?;
        if byte > 1 {
            warn!("Boolean value was not 0 or 1, but {byte}");
        }
        Ok(byte != 0)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let byte = u8::from(*self);
        byte.azalea_write(buf)
    }
}

impl AzBuf for u8 {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(buf.read_u8()?)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        buf.write_u8(*self)
    }
}

impl AzBuf for i8 {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        u8::azalea_read(buf).map(|i| i as i8)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        (*self as u8).azalea_write(buf)
    }
}

impl AzBuf for f32 {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(buf.read_f32::<BE>()?)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        buf.write_f32::<BE>(*self)
    }
}

impl AzBuf for f64 {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(buf.read_f64::<BE>()?)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        buf.write_f64::<BE>(*self)
    }
}
