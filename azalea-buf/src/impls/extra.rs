use std::{
    collections::HashMap,
    hash::Hash,
    io::{self, Cursor, Write},
    sync::Arc,
};

use indexmap::IndexMap;

use crate::{
    AzBuf, AzBufLimited, AzBufVar, BufReadError, BufReadErrorRepr, MAX_STRING_LENGTH,
    UnsizedByteArray, read_bytes, read_utf8_with_len, write_utf_with_len,
};

impl AzBuf for UnsizedByteArray {
    fn azalea_read(_ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        // read to end of the buffer
        let data = buf.get_ref()[buf.position() as usize..].to_vec();
        buf.set_position((buf.position()) + data.len() as u64);
        Ok(UnsizedByteArray(data))
    }
    fn azalea_write(&self, _ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
        buf.write_all(self)
    }
}

macro_rules! impl_for_map_type {
    ($ty: ident) => {
        impl<K: AzBuf + Eq + Hash, V: AzBuf> AzBuf for $ty<K, V> {
            fn azalea_read(ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
                let length = i32::azalea_read_var(ctx, buf)? as usize;
                let mut contents = Self::with_capacity(usize::min(length, 65536));
                for _ in 0..length {
                    contents.insert(K::azalea_read(ctx, buf)?, V::azalea_read(ctx, buf)?);
                }
                Ok(contents)
            }
            fn azalea_write(&self, ctx: $crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
                (self.len() as u32).azalea_write_var(ctx, buf)?;
                for (key, value) in self {
                    key.azalea_write(ctx, buf)?;
                    value.azalea_write(ctx, buf)?;
                }

                Ok(())
            }
        }
        impl<K: AzBuf + Eq + Hash, V: AzBufVar> AzBufVar for $ty<K, V> {
            fn azalea_read_var(
                ctx: crate::Ctx,
                buf: &mut Cursor<&[u8]>,
            ) -> Result<Self, BufReadError> {
                let length = i32::azalea_read_var(ctx, buf)? as usize;
                let mut contents = Self::with_capacity(usize::min(length, 65536));
                for _ in 0..length {
                    contents.insert(K::azalea_read(ctx, buf)?, V::azalea_read_var(ctx, buf)?);
                }
                Ok(contents)
            }
            fn azalea_write_var(&self, ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
                u32::azalea_write_var(&(self.len() as u32), ctx, buf)?;
                for (key, value) in self {
                    key.azalea_write(ctx, buf)?;
                    value.azalea_write_var(ctx, buf)?;
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
            default fn azalea_read(
                ctx: crate::Ctx,
                buf: &mut Cursor<&[u8]>,
            ) -> Result<Self, BufReadError> {
                let length = u32::azalea_read_var(ctx, buf)? as usize;
                // we limit the capacity to not get exploited into allocating a bunch
                let mut contents = Vec::with_capacity(usize::min(length, 65536));
                for _ in 0..length {
                    contents.push(T::azalea_read(ctx, buf)?);
                }
                Ok(contents.into())
            }
            default fn azalea_write(
                &self,
                ctx: $crate::Ctx,
                buf: &mut impl Write,
            ) -> io::Result<()> {
                (self.len() as u32).azalea_write_var(ctx, buf)?;
                for item in self.iter() {
                    T::azalea_write(item, ctx, buf)?;
                }
                Ok(())
            }
        }
        impl<T: AzBufVar> AzBufVar for $ty {
            fn azalea_read_var(
                ctx: $crate::Ctx,
                buf: &mut Cursor<&[u8]>,
            ) -> Result<Self, BufReadError> {
                let length = i32::azalea_read_var(ctx, buf)? as usize;
                let mut contents = Vec::with_capacity(usize::min(length, 65536));
                for _ in 0..length {
                    contents.push(T::azalea_read_var(ctx, buf)?);
                }
                Ok(contents.into())
            }
            fn azalea_write_var(&self, ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
                (self.len() as u32).azalea_write_var(ctx, buf)?;
                for item in self.iter() {
                    T::azalea_write_var(item, ctx, buf)?;
                }
                Ok(())
            }
        }
        impl<T: AzBuf> AzBufLimited for $ty {
            fn azalea_read_limited(
                ctx: $crate::Ctx,
                buf: &mut Cursor<&[u8]>,
                limit: u32,
            ) -> Result<Self, BufReadError> {
                let length = u32::azalea_read_var(ctx, buf)?;
                if length > limit {
                    return Err(BufReadErrorRepr::VecLengthTooLong {
                        length: length as u32,
                        max_length: limit as u32,
                    }
                    .into());
                }

                let mut contents = Vec::with_capacity(u32::min(length, 65536) as usize);
                for _ in 0..length {
                    contents.push(T::azalea_read(ctx, buf)?);
                }
                Ok(contents.into())
            }
        }
    };
}

impl_for_list_type!(Vec<T>);
impl_for_list_type!(Box<[T]>);
// `Arc<[T]>` is deliberately not implemented here, because converting a
// `Vec<T>` to `Arc<[T]>` results in allocations (since `Arc` stores the
// counters on the heap) and we'd like to avoid that. for us it's typically
// better to do `Arc<Box<[T]>>`.

impl AzBuf for Vec<u8> {
    fn azalea_read(ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let length = i32::azalea_read_var(ctx, buf)? as usize;
        read_bytes(buf, length).map(|b| b.to_vec())
    }
    fn azalea_write(&self, ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
        (self.len() as u32).azalea_write_var(ctx, buf)?;
        buf.write_all(self)
    }
}

impl AzBuf for Box<[u8]> {
    fn azalea_read(ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let length = i32::azalea_read_var(ctx, buf)? as usize;
        read_bytes(buf, length).map(Box::from)
    }
    fn azalea_write(&self, ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
        (self.len() as u32).azalea_write_var(ctx, buf)?;
        buf.write_all(self)
    }
}

impl AzBuf for String {
    fn azalea_read(ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        read_utf8_with_len(ctx, buf, MAX_STRING_LENGTH).map(Into::into)
    }
    fn azalea_write(&self, ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
        write_utf_with_len(ctx, buf, self, MAX_STRING_LENGTH)
    }
}
impl AzBufLimited for String {
    fn azalea_read_limited(
        ctx: crate::Ctx,
        buf: &mut Cursor<&[u8]>,
        limit: u32,
    ) -> Result<Self, BufReadError> {
        read_utf8_with_len(ctx, buf, limit).map(Into::into)
    }
}

impl AzBuf for Box<str> {
    fn azalea_read(ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        read_utf8_with_len(ctx, buf, MAX_STRING_LENGTH).map(Into::into)
    }
    fn azalea_write(&self, ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
        write_utf_with_len(ctx, buf, self, MAX_STRING_LENGTH)
    }
}

impl AzBufLimited for Box<str> {
    fn azalea_read_limited(
        ctx: crate::Ctx,
        buf: &mut Cursor<&[u8]>,
        limit: u32,
    ) -> Result<Self, BufReadError> {
        String::azalea_read_limited(ctx, buf, limit).map(Into::into)
    }
}

impl<T: AzBuf> AzBuf for Option<T> {
    fn azalea_read(ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let present = bool::azalea_read(ctx, buf)?;
        Ok(if present {
            Some(T::azalea_read(ctx, buf)?)
        } else {
            None
        })
    }
    fn azalea_write(&self, ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
        if let Some(s) = self {
            true.azalea_write(ctx, buf)?;
            s.azalea_write(ctx, buf)?;
        } else {
            false.azalea_write(ctx, buf)?;
        };
        Ok(())
    }
}

impl<T: AzBufVar> AzBufVar for Option<T> {
    fn azalea_read_var(ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let present = bool::azalea_read(ctx, buf)?;
        Ok(if present {
            Some(T::azalea_read_var(ctx, buf)?)
        } else {
            None
        })
    }
    fn azalea_write_var(&self, ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
        if let Some(s) = self {
            true.azalea_write(ctx, buf)?;
            s.azalea_write_var(ctx, buf)?;
        } else {
            false.azalea_write(ctx, buf)?;
        };
        Ok(())
    }
}
impl<T: AzBufLimited> AzBufLimited for Option<T> {
    fn azalea_read_limited(
        ctx: crate::Ctx,
        buf: &mut Cursor<&[u8]>,
        limit: u32,
    ) -> Result<Self, BufReadError> {
        let present = bool::azalea_read(ctx, buf)?;
        Ok(if present {
            Some(T::azalea_read_limited(ctx, buf, limit)?)
        } else {
            None
        })
    }
}

impl<T: AzBuf, const N: usize> AzBuf for [T; N] {
    fn azalea_read(ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut contents = Vec::with_capacity(N);
        for _ in 0..N {
            contents.push(T::azalea_read(ctx, buf)?);
        }
        Ok(contents
            .try_into()
            .unwrap_or_else(|_| unreachable!("The vec is the same size as the array")))
    }
    fn azalea_write(&self, ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
        for i in self {
            i.azalea_write(ctx, buf)?;
        }
        Ok(())
    }
}

impl AzBuf for simdnbt::owned::NbtTag {
    fn azalea_read(_ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(simdnbt::owned::read_tag(buf).map_err(simdnbt::Error::from)?)
    }
    fn azalea_write(&self, _ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
        let mut data = Vec::new();
        self.write(&mut data);
        buf.write_all(&data)
    }
}

impl AzBuf for simdnbt::owned::NbtCompound {
    fn azalea_read(_ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        match simdnbt::owned::read_tag(buf).map_err(simdnbt::Error::from)? {
            simdnbt::owned::NbtTag::Compound(compound) => Ok(compound),
            _ => Err(BufReadError::custom("Expected compound tag")),
        }
    }
    fn azalea_write(&self, _ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
        let mut data = Vec::new();
        simdnbt::owned::NbtTag::Compound(self.clone()).write(&mut data);
        buf.write_all(&data)
    }
}

impl AzBuf for simdnbt::owned::Nbt {
    fn azalea_read(_ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(simdnbt::owned::read_unnamed(buf)?)
    }
    fn azalea_write(&self, _ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
        let mut data = Vec::new();
        self.write_unnamed(&mut data);
        buf.write_all(&data)
    }
}

impl<T> AzBuf for Box<T>
where
    T: AzBuf,
{
    fn azalea_read(ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        T::azalea_read(ctx, buf).map(Box::new)
    }
    fn azalea_write(&self, ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
        T::azalea_write(&**self, ctx, buf)
    }
}

impl<A: AzBuf, B: AzBuf> AzBuf for (A, B) {
    fn azalea_read(ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let a = A::azalea_read(ctx, buf)?;
        let b = B::azalea_read(ctx, buf)?;
        Ok((a, b))
    }
    fn azalea_write(&self, ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
        self.0.azalea_write(ctx, buf)?;
        self.1.azalea_write(ctx, buf)
    }
}

impl<T: AzBuf> AzBuf for Arc<T> {
    fn azalea_read(ctx: crate::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(Arc::new(T::azalea_read(ctx, buf)?))
    }
    fn azalea_write(&self, ctx: crate::Ctx, buf: &mut impl Write) -> io::Result<()> {
        T::azalea_write(&**self, ctx, buf)
    }
}
