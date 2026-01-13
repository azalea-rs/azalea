use std::{
    collections::HashMap,
    hash::Hash,
    io::{self, Cursor, Write},
    sync::Arc,
};

use indexmap::IndexMap;

use crate::{
    AzBuf, AzBufLimited, AzBufVar, BufReadError, MAX_STRING_LENGTH, UnsizedByteArray, read_bytes,
    read_utf_with_len, write_utf_with_len,
};

impl AzBuf for UnsizedByteArray {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        // read to end of the buffer
        let data = buf.get_ref()[buf.position() as usize..].to_vec();
        buf.set_position((buf.position()) + data.len() as u64);
        Ok(UnsizedByteArray(data))
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        buf.write_all(self)
    }
}

macro_rules! impl_for_map_type {
    ($ty: ident) => {
        impl<K: AzBuf + Eq + Hash, V: AzBuf> AzBuf for $ty<K, V> {
            fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
                let length = i32::azalea_read_var(buf)? as usize;
                let mut contents = Self::with_capacity(usize::min(length, 65536));
                for _ in 0..length {
                    contents.insert(K::azalea_read(buf)?, V::azalea_read(buf)?);
                }
                Ok(contents)
            }
            fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
                u32::azalea_write_var(&(self.len() as u32), buf)?;
                for (key, value) in self {
                    key.azalea_write(buf)?;
                    value.azalea_write(buf)?;
                }

                Ok(())
            }
        }
        impl<K: AzBuf + Eq + Hash, V: AzBufVar> AzBufVar for $ty<K, V> {
            fn azalea_read_var(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
                let length = i32::azalea_read_var(buf)? as usize;
                let mut contents = Self::with_capacity(usize::min(length, 65536));
                for _ in 0..length {
                    contents.insert(K::azalea_read(buf)?, V::azalea_read_var(buf)?);
                }
                Ok(contents)
            }
            fn azalea_write_var(&self, buf: &mut impl Write) -> io::Result<()> {
                u32::azalea_write_var(&(self.len() as u32), buf)?;
                for (key, value) in self {
                    key.azalea_write(buf)?;
                    value.azalea_write_var(buf)?;
                }

                Ok(())
            }
        }
    };
}

impl_for_map_type!(HashMap);
impl_for_map_type!(IndexMap);

macro_rules! impl_for_list_type {
    ($ty: ty) => {
        impl<T: AzBuf> AzBuf for $ty {
            default fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
                let length = u32::azalea_read_var(buf)? as usize;
                // we limit the capacity to not get exploited into allocating a bunch
                let mut contents = Vec::with_capacity(usize::min(length, 65536));
                for _ in 0..length {
                    contents.push(T::azalea_read(buf)?);
                }
                Ok(contents.into())
            }
            default fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
                (self.len() as u32).azalea_write_var(buf)?;
                for item in self {
                    T::azalea_write(item, buf)?;
                }
                Ok(())
            }
        }
        impl<T: AzBufVar> AzBufVar for $ty {
            fn azalea_read_var(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
                let length = i32::azalea_read_var(buf)? as usize;
                let mut contents = Vec::with_capacity(usize::min(length, 65536));
                for _ in 0..length {
                    contents.push(T::azalea_read_var(buf)?);
                }
                Ok(contents.into())
            }
            fn azalea_write_var(&self, buf: &mut impl Write) -> io::Result<()> {
                (self.len() as u32).azalea_write_var(buf)?;
                for item in self {
                    T::azalea_write_var(item, buf)?;
                }
                Ok(())
            }
        }
        impl<T: AzBuf> AzBufLimited for $ty {
            fn azalea_read_limited(
                buf: &mut Cursor<&[u8]>,
                limit: u32,
            ) -> Result<Self, BufReadError> {
                let length = u32::azalea_read_var(buf)?;
                if length > limit {
                    return Err(BufReadError::VecLengthTooLong {
                        length: length as u32,
                        max_length: limit as u32,
                    });
                }

                let mut contents = Vec::with_capacity(u32::min(length, 65536) as usize);
                for _ in 0..length {
                    contents.push(T::azalea_read(buf)?);
                }
                Ok(contents.into())
            }
        }
    };
}

impl_for_list_type!(Vec<T>);
impl_for_list_type!(Box<[T]>);

impl AzBuf for Vec<u8> {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let length = i32::azalea_read_var(buf)? as usize;
        read_bytes(buf, length).map(|b| b.to_vec())
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        (self.len() as u32).azalea_write_var(buf)?;
        buf.write_all(self)
    }
}

impl AzBuf for String {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        read_utf_with_len(buf, MAX_STRING_LENGTH).map(Into::into)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        write_utf_with_len(buf, self, MAX_STRING_LENGTH)
    }
}
impl AzBufLimited for String {
    fn azalea_read_limited(buf: &mut Cursor<&[u8]>, limit: u32) -> Result<Self, BufReadError> {
        read_utf_with_len(buf, limit).map(Into::into)
    }
}

impl AzBuf for Box<str> {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        read_utf_with_len(buf, MAX_STRING_LENGTH).map(Into::into)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        write_utf_with_len(buf, self, MAX_STRING_LENGTH)
    }
}

impl AzBufLimited for Box<str> {
    fn azalea_read_limited(buf: &mut Cursor<&[u8]>, limit: u32) -> Result<Self, BufReadError> {
        String::azalea_read_limited(buf, limit).map(Into::into)
    }
}

impl<T: AzBuf> AzBuf for Option<T> {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let present = bool::azalea_read(buf)?;
        Ok(if present {
            Some(T::azalea_read(buf)?)
        } else {
            None
        })
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        if let Some(s) = self {
            true.azalea_write(buf)?;
            s.azalea_write(buf)?;
        } else {
            false.azalea_write(buf)?;
        };
        Ok(())
    }
}

impl<T: AzBufVar> AzBufVar for Option<T> {
    fn azalea_read_var(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let present = bool::azalea_read(buf)?;
        Ok(if present {
            Some(T::azalea_read_var(buf)?)
        } else {
            None
        })
    }
    fn azalea_write_var(&self, buf: &mut impl Write) -> io::Result<()> {
        if let Some(s) = self {
            true.azalea_write(buf)?;
            s.azalea_write_var(buf)?;
        } else {
            false.azalea_write(buf)?;
        };
        Ok(())
    }
}
impl<T: AzBufLimited> AzBufLimited for Option<T> {
    fn azalea_read_limited(buf: &mut Cursor<&[u8]>, limit: u32) -> Result<Self, BufReadError> {
        let present = bool::azalea_read(buf)?;
        Ok(if present {
            Some(T::azalea_read_limited(buf, limit)?)
        } else {
            None
        })
    }
}

impl<T: AzBuf, const N: usize> AzBuf for [T; N] {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut contents = Vec::with_capacity(N);
        for _ in 0..N {
            contents.push(T::azalea_read(buf)?);
        }
        Ok(contents
            .try_into()
            .unwrap_or_else(|_| unreachable!("The vec is the same size as the array")))
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        for i in self {
            i.azalea_write(buf)?;
        }
        Ok(())
    }
}

impl AzBuf for simdnbt::owned::NbtTag {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(simdnbt::owned::read_tag(buf).map_err(simdnbt::Error::from)?)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let mut data = Vec::new();
        self.write(&mut data);
        buf.write_all(&data)
    }
}

impl AzBuf for simdnbt::owned::NbtCompound {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        match simdnbt::owned::read_tag(buf).map_err(simdnbt::Error::from)? {
            simdnbt::owned::NbtTag::Compound(compound) => Ok(compound),
            _ => Err(BufReadError::Custom("Expected compound tag".to_owned())),
        }
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let mut data = Vec::new();
        simdnbt::owned::NbtTag::Compound(self.clone()).write(&mut data);
        buf.write_all(&data)
    }
}

impl AzBuf for simdnbt::owned::Nbt {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(simdnbt::owned::read_unnamed(buf)?)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let mut data = Vec::new();
        self.write_unnamed(&mut data);
        buf.write_all(&data)
    }
}

impl<T> AzBuf for Box<T>
where
    T: AzBuf,
{
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        T::azalea_read(buf).map(Box::new)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        T::azalea_write(&**self, buf)
    }
}

impl<A: AzBuf, B: AzBuf> AzBuf for (A, B) {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok((A::azalea_read(buf)?, B::azalea_read(buf)?))
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        self.0.azalea_write(buf)?;
        self.1.azalea_write(buf)
    }
}

impl<T: AzBuf> AzBuf for Arc<T> {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(Arc::new(T::azalea_read(buf)?))
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        T::azalea_write(&**self, buf)
    }
}
