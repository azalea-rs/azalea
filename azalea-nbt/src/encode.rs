use byteorder::{ReadBytesExt, BE};
use error::Error;
use std::{collections::HashMap, io::Read};
use tag::Tag;

impl Tag {
    fn write(&self, stream: &mut impl Read) -> Result<(), Error> {
        println!("read_known: id={}", id);
        let tag = match id {
            // Signifies the end of a TAG_Compound. It is only ever used inside
            // a TAG_Compound, and is not named despite being in a TAG_Compound
            0 => Tag::End,
            // A single signed byte
            1 => Tag::Byte(stream.read_i8().map_err(|_| Error::InvalidTag)?),
            // A single signed, big endian 16 bit integer
            2 => Tag::Short(stream.read_i16::<BE>().map_err(|_| Error::InvalidTag)?),
            // A single signed, big endian 32 bit integer
            3 => Tag::Int(stream.read_i32::<BE>().map_err(|_| Error::InvalidTag)?),
            // A single signed, big endian 64 bit integer
            4 => Tag::Long(stream.read_i64::<BE>().map_err(|_| Error::InvalidTag)?),
            // A single, big endian IEEE-754 single-precision floating point
            // number (NaN possible)
            5 => Tag::Float(stream.read_f32::<BE>().map_err(|_| Error::InvalidTag)?),
            // A single, big endian IEEE-754 double-precision floating point
            // number (NaN possible)
            6 => Tag::Double(stream.read_f64::<BE>().map_err(|_| Error::InvalidTag)?),
            // A length-prefixed array of signed bytes. The prefix is a signed
            // integer (thus 4 bytes)
            7 => {
                let length = stream.read_i32::<BE>().map_err(|_| Error::InvalidTag)?;
                let mut bytes = Vec::new();
                for _ in 0..length {
                    bytes.push(stream.read_i8().map_err(|_| Error::InvalidTag)?);
                }
                Tag::ByteArray(bytes)
            }
            // A length-prefixed modified UTF-8 string. The prefix is an
            // unsigned short (thus 2 bytes) signifying the length of the
            // string in bytes
            8 => {
                let length = stream.read_u16::<BE>().map_err(|_| Error::InvalidTag)?;
                let mut bytes = Vec::new();
                for _ in 0..length {
                    bytes.push(stream.read_u8().map_err(|_| Error::InvalidTag)?);
                }
                Tag::String(String::from_utf8(bytes).map_err(|_| Error::InvalidTag)?)
            }
            // A list of nameless tags, all of the same type. The list is
            // prefixed with the Type ID of the items it contains (thus 1
            // byte), and the length of the list as a signed integer (a further
            // 4 bytes). If the length of the list is 0 or negative, the type
            // may be 0 (TAG_End) but otherwise it must be any other type. (The
            // notchian implementation uses TAG_End in that situation, but
            // another reference implementation by Mojang uses 1 instead;
            // parsers should accept any type if the length is <= 0).
            9 => {
                let type_id = stream.read_u8().map_err(|_| Error::InvalidTag)?;
                let length = stream.read_i32::<BE>().map_err(|_| Error::InvalidTag)?;
                let mut list = Vec::new();
                for _ in 0..length {
                    list.push(Tag::read_known(stream, type_id)?);
                }
                Tag::List(list)
            }
            // Effectively a list of a named tags. Order is not guaranteed.
            10 => {
                println!("reading compound {{");
                let mut map = HashMap::new();
                loop {
                    let tag_id = stream.read_u8().unwrap_or(0);
                    println!("compound tag id: {}", tag_id);
                    if tag_id == 0 {
                        break;
                    }
                    let name = match Tag::read_known(stream, 8)? {
                        Tag::String(name) => name,
                        _ => panic!("Expected a string tag"),
                    };
                    println!("compound name: {}", name);
                    let tag = Tag::read_known(stream, tag_id).map_err(|_| Error::InvalidTag)?;
                    println!("aight read tag: {:?}", tag);
                    map.insert(name, tag);
                }
                println!("}} compound map: {:?}", map);
                Tag::Compound(map)
            }
            // A length-prefixed array of signed integers. The prefix is a
            // signed integer (thus 4 bytes) and indicates the number of 4 byte
            // integers.
            11 => {
                let length = stream.read_i32::<BE>().map_err(|_| Error::InvalidTag)?;
                let mut ints = Vec::new();
                for _ in 0..length {
                    ints.push(stream.read_i32::<BE>().map_err(|_| Error::InvalidTag)?);
                }
                Tag::IntArray(ints)
            }
            // A length-prefixed array of signed longs. The prefix is a signed
            // integer (thus 4 bytes) and indicates the number of 8 byte longs.
            12 => {
                let length = stream.read_i32::<BE>().map_err(|_| Error::InvalidTag)?;
                let mut longs = Vec::new();
                for _ in 0..length {
                    longs.push(stream.read_i64::<BE>().map_err(|_| Error::InvalidTag)?);
                }
                Tag::LongArray(longs)
            }
            _ => return Err(Error::InvalidTagType(id)),
        };
        Ok(tag)
    }

    pub fn read(stream: &mut impl Read) -> Result<Tag, Error> {
        // default to compound tag
        Tag::read_known(stream, 10)
    }
}
