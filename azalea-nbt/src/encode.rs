use crate::Error;
use crate::Tag;
use byteorder::{WriteBytesExt, BE};
use flate2::write::{GzEncoder, ZlibEncoder};
use std::io::Write;

impl Tag {
    pub fn write_without_end(&self, writer: &mut dyn Write) -> Result<(), Error> {
        match self {
            Tag::End => {}
            Tag::Byte(value) => writer.write_i8(*value).map_err(|_| Error::WriteError)?,
            Tag::Short(value) => writer
                .write_i16::<BE>(*value)
                .map_err(|_| Error::WriteError)?,
            Tag::Int(value) => writer
                .write_i32::<BE>(*value)
                .map_err(|_| Error::WriteError)?,
            Tag::Long(value) => writer
                .write_i64::<BE>(*value)
                .map_err(|_| Error::WriteError)?,
            Tag::Float(value) => writer
                .write_f32::<BE>(*value)
                .map_err(|_| Error::WriteError)?,
            Tag::Double(value) => writer
                .write_f64::<BE>(*value)
                .map_err(|_| Error::WriteError)?,
            Tag::ByteArray(value) => {
                writer
                    .write_i32::<BE>(value.len() as i32)
                    .map_err(|_| Error::WriteError)?;
                for byte in value {
                    writer.write_i8(*byte).map_err(|_| Error::WriteError)?;
                }
            }
            Tag::String(value) => {
                writer
                    .write_i16::<BE>(value.len() as i16)
                    .map_err(|_| Error::WriteError)?;
                writer
                    .write_all(value.as_bytes())
                    .map_err(|_| Error::WriteError)?;
            }
            Tag::List(value) => {
                // we just get the type from the first item, or default the type to END
                let type_id = value.first().and_then(|f| Some(f.id())).unwrap_or(0);
                writer.write_u8(type_id).map_err(|_| Error::WriteError)?;
                writer
                    .write_i32::<BE>(value.len() as i32)
                    .map_err(|_| Error::WriteError)?;
                for tag in value {
                    tag.write_without_end(writer)?;
                }
            }
            Tag::Compound(value) => {
                for (key, tag) in value {
                    writer.write_u8(tag.id()).map_err(|_| Error::WriteError)?;
                    Tag::String(key.clone()).write_without_end(writer)?;
                    tag.write_without_end(writer)?;
                }
                writer
                    .write_u8(Tag::End.id())
                    .map_err(|_| Error::WriteError)?;
            }
            Tag::IntArray(value) => {
                writer
                    .write_i32::<BE>(value.len() as i32)
                    .map_err(|_| Error::WriteError)?;
                for int in value {
                    writer
                        .write_i32::<BE>(*int)
                        .map_err(|_| Error::WriteError)?;
                }
            }
            Tag::LongArray(value) => {
                writer
                    .write_i32::<BE>(value.len() as i32)
                    .map_err(|_| Error::WriteError)?;
                for long in value {
                    writer
                        .write_i64::<BE>(*long)
                        .map_err(|_| Error::WriteError)?;
                }
            }
        }

        Ok(())
    }

    pub fn write(&self, writer: &mut impl Write) -> Result<(), Error> {
        match self {
            Tag::Compound(value) => {
                for (key, tag) in value {
                    writer.write_u8(tag.id()).map_err(|_| Error::WriteError)?;
                    Tag::String(key.clone()).write_without_end(writer)?;
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
