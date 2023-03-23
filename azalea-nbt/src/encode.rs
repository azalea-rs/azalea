use crate::tag::*;
use azalea_buf::McBufWritable;
use byteorder::{WriteBytesExt, BE};
use flate2::write::{GzEncoder, ZlibEncoder};
use std::io::Write;

#[inline(always)]
fn write_string(writer: &mut impl Write, string: &NbtString) {
    writer.write_u16::<BE>(string.len() as u16).unwrap();
    writer.write_all(string.as_bytes()).unwrap();
}

#[inline(always)]
fn write_compound(writer: &mut impl Write, value: &NbtCompound, end_tag: bool) {
    for (key, tag) in value.iter() {
        writer.write_u8(tag.id()).unwrap();
        write_string(writer, key);
        match tag {
            Nbt::End => {}
            Nbt::Byte(value) => {
                writer.write_i8(*value).unwrap();
            }
            Nbt::Short(value) => {
                writer.write_i16::<BE>(*value).unwrap();
            }
            Nbt::Int(value) => {
                writer.write_i32::<BE>(*value).unwrap();
            }
            Nbt::Long(value) => {
                writer.write_i64::<BE>(*value).unwrap();
            }
            Nbt::Float(value) => {
                writer.write_f32::<BE>(*value).unwrap();
            }
            Nbt::Double(value) => {
                writer.write_f64::<BE>(*value).unwrap();
            }
            Nbt::ByteArray(value) => {
                write_byte_array(writer, value);
            }
            Nbt::String(value) => {
                write_string(writer, value);
            }
            Nbt::List(value) => {
                write_list(writer, value);
            }
            Nbt::Compound(value) => {
                write_compound(writer, value, true);
            }
            Nbt::IntArray(value) => {
                write_int_array(writer, value);
            }
            Nbt::LongArray(value) => {
                write_long_array(writer, value);
            }
        }
    }
    if end_tag {
        writer.write_u8(END_ID).unwrap();
    }
}

#[inline(always)]
fn write_list(writer: &mut impl Write, value: &NbtList) {
    writer.write_u8(value.id()).unwrap();
    match value {
        NbtList::Empty => writer.write_all(&[0; 4]).unwrap(),
        NbtList::Byte(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            let l = l.as_slice();
            writer
                // convert [i8] into [u8]
                .write_all(unsafe { std::slice::from_raw_parts(l.as_ptr() as *const u8, l.len()) })
                .unwrap();
        }
        NbtList::Short(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for &v in l {
                writer.write_i16::<BE>(v).unwrap();
            }
        }
        NbtList::Int(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for &v in l {
                writer.write_i32::<BE>(v).unwrap();
            }
        }
        NbtList::Long(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for &v in l {
                writer.write_i64::<BE>(v).unwrap();
            }
        }
        NbtList::Float(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for &v in l {
                writer.write_f32::<BE>(v).unwrap();
            }
        }
        NbtList::Double(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for &v in l {
                writer.write_f64::<BE>(v).unwrap();
            }
        }
        NbtList::ByteArray(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for v in l {
                write_byte_array(writer, v);
            }
        }
        NbtList::String(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for v in l {
                write_string(writer, v);
            }
        }
        NbtList::List(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for v in l {
                write_list(writer, v);
            }
        }
        NbtList::Compound(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for v in l {
                write_compound(writer, v, true);
            }
        }
        NbtList::IntArray(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for v in l {
                write_int_array(writer, v);
            }
        }
        NbtList::LongArray(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for v in l {
                write_long_array(writer, v);
            }
        }
    }
}

#[inline]
fn write_byte_array(writer: &mut impl Write, value: &[u8]) {
    writer.write_u32::<BE>(value.len() as u32).unwrap();
    writer.write_all(value).unwrap();
}

#[inline]
fn write_int_array(writer: &mut impl Write, value: &Vec<i32>) {
    writer.write_u32::<BE>(value.len() as u32).unwrap();
    for &int in value {
        writer.write_i32::<BE>(int).unwrap();
    }
}

#[inline]
fn write_long_array(writer: &mut impl Write, value: &Vec<i64>) {
    writer.write_u32::<BE>(value.len() as u32).unwrap();
    for &long in value {
        writer.write_i64::<BE>(long).unwrap();
    }
}

impl Nbt {
    /// Write the compound tag as NBT data.
    ///
    /// # Panics
    ///
    /// Will panic if the tag is not a Compound or End tag.
    pub fn write(&self, writer: &mut impl Write) {
        match self {
            Nbt::Compound(value) => {
                write_compound(writer, value, false);
            }
            Nbt::End => {
                END_ID.write_into(writer).unwrap();
            }
            _ => panic!("Not a compound tag"),
        }
    }

    /// Write the compound tag as NBT data compressed wtih zlib.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if it's not a Compound or End tag.
    pub fn write_zlib(&self, writer: &mut impl Write) {
        let mut encoder = ZlibEncoder::new(writer, flate2::Compression::default());
        self.write(&mut encoder)
    }

    /// Write the compound tag as NBT data compressed wtih gzip.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if it's not a Compound or End tag.
    pub fn write_gzip(&self, writer: &mut impl Write) {
        let mut encoder = GzEncoder::new(writer, flate2::Compression::default());
        self.write(&mut encoder)
    }
}

impl McBufWritable for Nbt {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.write(buf);
        Ok(())
    }
}
