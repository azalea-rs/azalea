use crate::tag::*;
use crate::Error;
use azalea_buf::McBufWritable;
use byteorder::{WriteBytesExt, BE};
use flate2::write::{GzEncoder, ZlibEncoder};
use std::io::Write;

#[inline]
fn write_string(writer: &mut dyn Write, string: &str) -> Result<(), Error> {
    writer.write_u16::<BE>(string.len() as u16)?;
    writer.write_all(string.as_bytes())?;

    Ok(())
}

#[inline]
fn write_compound(writer: &mut dyn Write, value: &NbtCompound, end_tag: bool) -> Result<(), Error> {
    for (key, tag) in value.inner() {
        match tag {
            Tag::End => {}
            Tag::Byte(value) => {
                writer.write_u8(BYTE_ID)?;
                write_string(writer, key)?;
                writer.write_i8(*value)?;
            }
            Tag::Short(value) => {
                writer.write_u8(SHORT_ID)?;
                write_string(writer, key)?;
                writer.write_i16::<BE>(*value)?;
            }
            Tag::Int(value) => {
                writer.write_u8(INT_ID)?;
                write_string(writer, key)?;
                writer.write_i32::<BE>(*value)?;
            }
            Tag::Long(value) => {
                writer.write_u8(LONG_ID)?;
                write_string(writer, key)?;
                writer.write_i64::<BE>(*value)?;
            }
            Tag::Float(value) => {
                writer.write_u8(FLOAT_ID)?;
                write_string(writer, key)?;
                writer.write_f32::<BE>(*value)?;
            }
            Tag::Double(value) => {
                writer.write_u8(DOUBLE_ID)?;
                write_string(writer, key)?;
                writer.write_f64::<BE>(*value)?;
            }
            Tag::ByteArray(value) => {
                writer.write_u8(BYTE_ARRAY_ID)?;
                write_string(writer, key)?;
                write_byte_array(writer, value)?;
            }
            Tag::String(value) => {
                writer.write_u8(STRING_ID)?;
                write_string(writer, key)?;
                write_string(writer, value)?;
            }
            Tag::List(value) => {
                writer.write_u8(LIST_ID)?;
                write_string(writer, key)?;
                write_list(writer, value)?;
            }
            Tag::Compound(value) => {
                writer.write_u8(COMPOUND_ID)?;
                write_string(writer, key)?;
                write_compound(writer, value, true)?;
            }
            Tag::IntArray(value) => {
                writer.write_u8(INT_ARRAY_ID)?;
                write_string(writer, key)?;
                write_int_array(writer, value)?;
            }
            Tag::LongArray(value) => {
                writer.write_u8(LONG_ARRAY_ID)?;
                write_string(writer, key)?;
                write_long_array(writer, value)?;
            }
        }
    }
    if end_tag {
        writer.write_u8(Tag::End.id())?;
    }
    Ok(())
}

#[inline]
fn write_list(writer: &mut dyn Write, value: &NbtList) -> Result<(), Error> {
    match value {
        NbtList::Empty => writer.write_all(&[0; 5])?,
        NbtList::Byte(l) => {
            writer.write_u8(BYTE_ID)?;
            writer.write_i32::<BE>(l.len() as i32)?;
            for v in l {
                writer.write_i8(*v)?;
            }
        }
        NbtList::Short(l) => {
            writer.write_u8(SHORT_ID)?;
            writer.write_i32::<BE>(l.len() as i32)?;
            for v in l {
                writer.write_i16::<BE>(*v)?;
            }
        }
        NbtList::Int(l) => {
            writer.write_u8(INT_ID)?;
            writer.write_i32::<BE>(l.len() as i32)?;
            for v in l {
                writer.write_i32::<BE>(*v)?;
            }
        }
        NbtList::Long(l) => {
            writer.write_u8(LONG_ID)?;
            writer.write_i32::<BE>(l.len() as i32)?;
            for v in l {
                writer.write_i64::<BE>(*v)?;
            }
        }
        NbtList::Float(l) => {
            writer.write_u8(FLOAT_ID)?;
            writer.write_i32::<BE>(l.len() as i32)?;
            for v in l {
                writer.write_f32::<BE>(*v)?;
            }
        }
        NbtList::Double(l) => {
            writer.write_u8(DOUBLE_ID)?;
            writer.write_i32::<BE>(l.len() as i32)?;
            for v in l {
                writer.write_f64::<BE>(*v)?;
            }
        }
        NbtList::ByteArray(l) => {
            writer.write_u8(BYTE_ARRAY_ID)?;
            writer.write_i32::<BE>(l.len() as i32)?;
            for v in l {
                write_byte_array(writer, v)?;
            }
        }
        NbtList::String(l) => {
            writer.write_u8(STRING_ID)?;
            writer.write_i32::<BE>(l.len() as i32)?;
            for v in l {
                write_string(writer, v)?;
            }
        }
        NbtList::List(l) => {
            writer.write_u8(LIST_ID)?;
            writer.write_i32::<BE>(l.len() as i32)?;
            for v in l {
                write_list(writer, v)?;
            }
        }
        NbtList::Compound(l) => {
            writer.write_u8(COMPOUND_ID)?;
            writer.write_i32::<BE>(l.len() as i32)?;
            for v in l {
                write_compound(writer, v, true)?;
            }
        }
        NbtList::IntArray(l) => {
            writer.write_u8(INT_ARRAY_ID)?;
            writer.write_i32::<BE>(l.len() as i32)?;
            for v in l {
                write_int_array(writer, v)?;
            }
        }
        NbtList::LongArray(l) => {
            writer.write_u8(LONG_ARRAY_ID)?;
            writer.write_i32::<BE>(l.len() as i32)?;
            for v in l {
                write_long_array(writer, v)?;
            }
        }
    }

    Ok(())
}

#[inline]
fn write_byte_array(writer: &mut dyn Write, value: &Vec<u8>) -> Result<(), Error> {
    writer.write_u32::<BE>(value.len() as u32)?;
    writer.write_all(value)?;
    Ok(())
}

#[inline]
fn write_int_array(writer: &mut dyn Write, value: &Vec<i32>) -> Result<(), Error> {
    writer.write_u32::<BE>(value.len() as u32)?;
    for &int in value {
        writer.write_i32::<BE>(int)?;
    }
    Ok(())
}

#[inline]
fn write_long_array(writer: &mut dyn Write, value: &Vec<i64>) -> Result<(), Error> {
    writer.write_u32::<BE>(value.len() as u32)?;
    for &long in value {
        writer.write_i64::<BE>(long)?;
    }
    Ok(())
}

impl Tag {
    /// Write the tag as unnamed, uncompressed NBT data. If you're writing a
    /// compound tag and the length of the NBT is already known, use
    /// [`Tag::write`] to avoid the `End` tag (this is used when writing NBT to
    /// a file).
    #[inline]
    pub fn write_without_end(&self, writer: &mut dyn Write) -> Result<(), Error> {
        match self {
            Tag::End => {}
            Tag::Byte(value) => writer.write_i8(*value)?,
            Tag::Short(value) => writer.write_i16::<BE>(*value)?,
            Tag::Int(value) => writer.write_i32::<BE>(*value)?,
            Tag::Long(value) => writer.write_i64::<BE>(*value)?,
            Tag::Float(value) => writer.write_f32::<BE>(*value)?,
            Tag::Double(value) => writer.write_f64::<BE>(*value)?,
            Tag::ByteArray(value) => write_byte_array(writer, value)?,
            Tag::String(value) => write_string(writer, value)?,
            Tag::List(value) => write_list(writer, value)?,
            Tag::Compound(value) => write_compound(writer, value, true)?,
            Tag::IntArray(value) => write_int_array(writer, value)?,
            Tag::LongArray(value) => write_long_array(writer, value)?,
        }

        Ok(())
    }

    /// Write the compound tag as NBT data.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if it's not a Compound or End tag.
    pub fn write(&self, writer: &mut impl Write) -> Result<(), Error> {
        match self {
            Tag::Compound(value) => {
                write_compound(writer, value, false)?;
                Ok(())
            }
            Tag::End => {
                0u8.write_into(writer)?;
                Ok(())
            }
            _ => Err(Error::InvalidTag),
        }
    }

    /// Write the compound tag as NBT data compressed wtih zlib.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if it's not a Compound or End tag.
    pub fn write_zlib(&self, writer: &mut impl Write) -> Result<(), Error> {
        let mut encoder = ZlibEncoder::new(writer, flate2::Compression::default());
        self.write(&mut encoder)
    }

    /// Write the compound tag as NBT data compressed wtih gzip.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if it's not a Compound or End tag.
    pub fn write_gzip(&self, writer: &mut impl Write) -> Result<(), Error> {
        let mut encoder = GzEncoder::new(writer, flate2::Compression::default());
        self.write(&mut encoder)
    }
}

impl McBufWritable for Tag {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.write(buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }
}
