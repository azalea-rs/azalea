use crate::Error;
use crate::Tag;
use byteorder::{WriteBytesExt, BE};
use flate2::write::{GzEncoder, ZlibEncoder};
use std::collections::HashMap;
use std::io::Write;

#[inline]
fn write_string(writer: &mut dyn Write, string: &str) -> Result<(), Error> {
    writer.write_i16::<BE>(string.len() as i16)?;
    writer.write_all(string.as_bytes())?;

    Ok(())
}
#[inline]
fn write_compound(writer: &mut dyn Write, value: &HashMap<String, Tag>) -> Result<(), Error> {
    for (key, tag) in value {
        writer.write_u8(tag.id())?;
        write_string(writer, key)?;
        tag.write_without_end(writer)?;
    }
    writer.write_u8(Tag::End.id())?;
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
            Tag::ByteArray(value) => {
                writer.write_i32::<BE>(value.len() as i32)?;
                for &byte in value {
                    writer.write_i8(byte)?;
                }
            }
            Tag::String(value) => {
                write_string(writer, value)?;
            }
            Tag::List(value) => {
                // we just get the type from the first item, or default the type to END
                if value.is_empty() {
                    writer.write_all(&[0; 5])?;
                } else {
                    let first_tag = &value[0];
                    writer.write_u8(first_tag.id())?;
                    writer.write_i32::<BE>(value.len() as i32)?;
                    match first_tag {
                        Self::Int(_) => {
                            for i in value {
                                if let Tag::Int(v) = i {
                                    writer.write_i32::<BE>(*v)?
                                } else {
                                    panic!("List of Ints should only contain Ints")
                                }
                            }
                        }
                        Self::String(_) => {
                            for i in value {
                                if let Tag::String(v) = i {
                                    write_string(writer, v)?;
                                } else {
                                    panic!("List of Strings should only contain Strings")
                                }
                            }
                        }
                        &Self::Compound(_) => {
                            for i in value {
                                if let Tag::Compound(v) = i {
                                    write_compound(writer, v)?;
                                } else {
                                    panic!("List of Compounds should only contain Compounds")
                                }
                            }
                        }
                        _ => {
                            for tag in value {
                                tag.write_without_end(writer)?;
                            }
                        }
                    }
                }
            }
            Tag::Compound(value) => write_compound(writer, value)?,
            Tag::IntArray(value) => {
                writer.write_i32::<BE>(value.len() as i32)?;
                for &int in value {
                    writer.write_i32::<BE>(int)?;
                }
            }
            Tag::LongArray(value) => {
                writer.write_i32::<BE>(value.len() as i32)?;
                for &long in value {
                    writer.write_i64::<BE>(long)?;
                }
            }
        }

        Ok(())
    }

    pub fn write(&self, writer: &mut impl Write) -> Result<(), Error> {
        match self {
            Tag::Compound(value) => {
                for (key, tag) in value {
                    writer.write_u8(tag.id())?;
                    write_string(writer, key)?;
                    tag.write_without_end(writer)?;
                }
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
