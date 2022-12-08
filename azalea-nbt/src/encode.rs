use crate::Error;
use crate::Tag;
use ahash::AHashMap;
use azalea_buf::McBufWritable;
use byteorder::{WriteBytesExt, BE};
use flate2::write::{GzEncoder, ZlibEncoder};
use std::io::Write;

// who needs friends when you've got code that runs in nanoseconds?

#[inline]
fn write_string(writer: &mut dyn Write, string: &str) -> Result<(), Error> {
    writer.write_u16::<BE>(string.len() as u16)?;
    writer.write_all(string.as_bytes())?;

    Ok(())
}

#[inline]
fn write_compound(
    writer: &mut dyn Write,
    value: &AHashMap<String, Tag>,
    end_tag: bool,
) -> Result<(), Error> {
    for (key, tag) in value {
        match tag {
            Tag::End => {}
            Tag::Byte(value) => {
                writer.write_u8(1)?;
                write_string(writer, key)?;
                writer.write_i8(*value)?
            }
            Tag::Short(value) => {
                writer.write_u8(2)?;
                write_string(writer, key)?;
                writer.write_i16::<BE>(*value)?
            }
            Tag::Int(value) => {
                writer.write_u8(3)?;
                write_string(writer, key)?;
                writer.write_i32::<BE>(*value)?
            }
            Tag::Long(value) => {
                writer.write_u8(4)?;
                write_string(writer, key)?;
                writer.write_i64::<BE>(*value)?
            }
            Tag::Float(value) => {
                writer.write_u8(5)?;
                write_string(writer, key)?;
                writer.write_f32::<BE>(*value)?
            }
            Tag::Double(value) => {
                writer.write_u8(6)?;
                write_string(writer, key)?;
                writer.write_f64::<BE>(*value)?
            }
            Tag::ByteArray(value) => {
                writer.write_u8(7)?;
                write_string(writer, key)?;
                write_bytearray(writer, value)?
            }
            Tag::String(value) => {
                writer.write_u8(8)?;
                write_string(writer, key)?;
                write_string(writer, value)?
            }
            Tag::List(value) => {
                writer.write_u8(9)?;
                write_string(writer, key)?;
                write_list(writer, value)?
            }
            Tag::Compound(value) => {
                writer.write_u8(10)?;
                write_string(writer, key)?;
                write_compound(writer, value, true)?
            }
            Tag::IntArray(value) => {
                writer.write_u8(11)?;
                write_string(writer, key)?;
                write_intarray(writer, value)?
            }
            Tag::LongArray(value) => {
                writer.write_u8(12)?;
                write_string(writer, key)?;
                write_longarray(writer, value)?
            }
        }
    }
    if end_tag {
        writer.write_u8(Tag::End.id())?;
    }
    Ok(())
}

#[inline]
fn write_list(writer: &mut dyn Write, value: &[Tag]) -> Result<(), Error> {
    // we just get the type from the first item, or default the type to END
    if value.is_empty() {
        writer.write_all(&[0; 5])?;
    } else {
        let first_tag = &value[0];
        writer.write_u8(first_tag.id())?;
        writer.write_i32::<BE>(value.len() as i32)?;
        match first_tag {
            Tag::Int(_) => {
                for tag in value {
                    writer.write_i32::<BE>(
                        *tag.as_int().expect("List of Int should only contains Int"),
                    )?;
                }
            }
            Tag::String(_) => {
                for tag in value {
                    write_string(
                        writer,
                        tag.as_string()
                            .expect("List of String should only contain String"),
                    )?;
                }
            }
            Tag::Compound(_) => {
                for tag in value {
                    write_compound(
                        writer,
                        tag.as_compound()
                            .expect("List of Compound should only contain Compound"),
                        true,
                    )?;
                }
            }
            _ => {
                for tag in value {
                    tag.write_without_end(writer)?;
                }
            }
        }
    }

    Ok(())
}

#[inline]
fn write_bytearray(writer: &mut dyn Write, value: &Vec<u8>) -> Result<(), Error> {
    writer.write_u32::<BE>(value.len() as u32)?;
    writer.write_all(value)?;
    Ok(())
}

#[inline]
fn write_intarray(writer: &mut dyn Write, value: &Vec<i32>) -> Result<(), Error> {
    writer.write_u32::<BE>(value.len() as u32)?;
    for &int in value {
        writer.write_i32::<BE>(int)?;
    }
    Ok(())
}

#[inline]
fn write_longarray(writer: &mut dyn Write, value: &Vec<i64>) -> Result<(), Error> {
    writer.write_u32::<BE>(value.len() as u32)?;
    for &long in value {
        writer.write_i64::<BE>(long)?;
    }
    Ok(())
}

impl Tag {
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
            Tag::ByteArray(value) => write_bytearray(writer, value)?,
            Tag::String(value) => write_string(writer, value)?,
            Tag::List(value) => write_list(writer, value)?,
            Tag::Compound(value) => write_compound(writer, value, true)?,
            Tag::IntArray(value) => write_intarray(writer, value)?,
            Tag::LongArray(value) => write_longarray(writer, value)?,
        }

        Ok(())
    }

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

    pub fn write_zlib(&self, writer: &mut impl Write) -> Result<(), Error> {
        let mut encoder = ZlibEncoder::new(writer, flate2::Compression::default());
        self.write(&mut encoder)
    }

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
