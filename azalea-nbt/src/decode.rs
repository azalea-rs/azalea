use crate::tag::*;
use crate::Error;
use azalea_buf::{BufReadError, McBufReadable};
use byteorder::{ReadBytesExt, BE};
use flate2::read::{GzDecoder, ZlibDecoder};
use log::warn;
use std::io::{BufRead, Cursor, Read};

#[inline]
fn read_bytes<'a>(buf: &'a mut Cursor<&[u8]>, length: usize) -> Result<&'a [u8], Error> {
    if length > (buf.get_ref().len() - buf.position() as usize) {
        return Err(Error::UnexpectedEof);
    }
    let initial_position = buf.position() as usize;
    buf.set_position(buf.position() + length as u64);
    let data = &buf.get_ref()[initial_position..initial_position + length];
    Ok(data)
}

#[inline]
fn read_string(stream: &mut Cursor<&[u8]>) -> Result<NbtString, Error> {
    let length = stream.read_u16::<BE>()? as usize;

    let buf = read_bytes(stream, length)?;

    Ok(if let Ok(string) = std::str::from_utf8(buf) {
        string.into()
    } else {
        let lossy_string = String::from_utf8_lossy(buf).into_owned();
        warn!("Error decoding utf8 (bytes: {buf:?}, lossy: \"{lossy_string})\"");
        lossy_string.into()
    })
}

#[inline]
fn read_byte_array(stream: &mut Cursor<&[u8]>) -> Result<NbtByteArray, Error> {
    let length = stream.read_u32::<BE>()? as usize;
    let bytes = read_bytes(stream, length)?.to_vec();
    Ok(bytes)
}

// https://stackoverflow.com/a/59707887
fn vec_u8_into_i8(v: Vec<u8>) -> Vec<i8> {
    // ideally we'd use Vec::into_raw_parts, but it's unstable,
    // so we have to do it manually:

    // first, make sure v's destructor doesn't free the data
    // it thinks it owns when it goes out of scope
    let mut v = std::mem::ManuallyDrop::new(v);

    // then, pick apart the existing Vec
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    // finally, adopt the data into a new Vec
    unsafe { Vec::from_raw_parts(p as *mut i8, len, cap) }
}

#[inline]
fn read_list(stream: &mut Cursor<&[u8]>) -> Result<NbtList, Error> {
    let type_id = stream.read_u8()?;
    let length = stream.read_u32::<BE>()?;
    let list = match type_id {
        END_ID => NbtList::Empty,
        BYTE_ID => NbtList::Byte(vec_u8_into_i8(
            read_bytes(stream, length as usize)?.to_vec(),
        )),
        SHORT_ID => NbtList::Short({
            if ((length * 2) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| stream.read_i16::<BE>())
                .collect::<Result<Vec<_>, _>>()?
        }),
        INT_ID => NbtList::Int({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| stream.read_i32::<BE>())
                .collect::<Result<Vec<_>, _>>()?
        }),
        LONG_ID => NbtList::Long({
            if ((length * 8) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| stream.read_i64::<BE>())
                .collect::<Result<Vec<_>, _>>()?
        }),
        FLOAT_ID => NbtList::Float({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| stream.read_f32::<BE>())
                .collect::<Result<Vec<_>, _>>()?
        }),
        DOUBLE_ID => NbtList::Double({
            if ((length * 8) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| stream.read_f64::<BE>())
                .collect::<Result<Vec<_>, _>>()?
        }),
        BYTE_ARRAY_ID => NbtList::ByteArray({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| read_byte_array(stream))
                .collect::<Result<Vec<_>, _>>()?
        }),
        STRING_ID => NbtList::String({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| read_string(stream))
                .collect::<Result<Vec<_>, _>>()?
        }),
        LIST_ID => NbtList::List({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| read_list(stream))
                .collect::<Result<Vec<_>, _>>()?
        }),
        COMPOUND_ID => NbtList::Compound({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| read_compound(stream))
                .collect::<Result<Vec<_>, _>>()?
        }),
        INT_ARRAY_ID => NbtList::IntArray({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| read_int_array(stream))
                .collect::<Result<Vec<_>, _>>()?
        }),
        LONG_ARRAY_ID => NbtList::LongArray({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| read_long_array(stream))
                .collect::<Result<Vec<_>, _>>()?
        }),
        _ => return Err(Error::InvalidTagType(type_id)),
    };
    Ok(list)
}

#[inline]
fn read_compound(stream: &mut Cursor<&[u8]>) -> Result<NbtCompound, Error> {
    // we default to capacity 4 because it'll probably not be empty
    let mut map = NbtCompound::with_capacity(4);
    loop {
        let tag_id = stream.read_u8().unwrap_or(0);
        if tag_id == 0 {
            break;
        }
        let name = read_string(stream)?;
        let tag = Nbt::read_known(stream, tag_id)?;
        map.insert_unsorted(name, tag);
    }
    map.sort();
    Ok(map)
}

#[inline]
fn read_int_array(stream: &mut Cursor<&[u8]>) -> Result<NbtIntArray, Error> {
    let length = stream.read_u32::<BE>()? as usize;
    if length * 4 > (stream.get_ref().len() - stream.position() as usize) {
        return Err(Error::UnexpectedEof);
    }
    let mut ints = NbtIntArray::with_capacity(length);
    for _ in 0..length {
        ints.push(stream.read_i32::<BE>()?);
    }
    Ok(ints)
}

#[inline]
fn read_long_array(stream: &mut Cursor<&[u8]>) -> Result<NbtLongArray, Error> {
    let length = stream.read_u32::<BE>()? as usize;
    if length * 8 > (stream.get_ref().len() - stream.position() as usize) {
        return Err(Error::UnexpectedEof);
    }
    let mut longs = NbtLongArray::with_capacity(length);
    for _ in 0..length {
        longs.push(stream.read_i64::<BE>()?);
    }
    Ok(longs)
}

impl Nbt {
    /// Read the NBT data when you already know the ID of the tag. You usually
    /// want [`Nbt::read`] if you're reading an NBT file.
    #[inline]
    fn read_known(stream: &mut Cursor<&[u8]>, id: u8) -> Result<Nbt, Error> {
        Ok(match id {
            // Signifies the end of a TAG_Compound. It is only ever used inside
            // a TAG_Compound, and is not named despite being in a TAG_Compound
            END_ID => Nbt::End,
            // A single signed byte
            BYTE_ID => Nbt::Byte(stream.read_i8()?),
            // A single signed, big endian 16 bit integer
            SHORT_ID => Nbt::Short(stream.read_i16::<BE>()?),
            // A single signed, big endian 32 bit integer
            INT_ID => Nbt::Int(stream.read_i32::<BE>()?),
            // A single signed, big endian 64 bit integer
            LONG_ID => Nbt::Long(stream.read_i64::<BE>()?),
            // A single, big endian IEEE-754 single-precision floating point
            // number (NaN possible)
            FLOAT_ID => Nbt::Float(stream.read_f32::<BE>()?),
            // A single, big endian IEEE-754 double-precision floating point
            // number (NaN possible)
            DOUBLE_ID => Nbt::Double(stream.read_f64::<BE>()?),
            // A length-prefixed array of signed bytes. The prefix is a signed
            // integer (thus 4 bytes)
            BYTE_ARRAY_ID => Nbt::ByteArray(read_byte_array(stream)?),
            // A length-prefixed modified UTF-8 string. The prefix is an
            // unsigned short (thus 2 bytes) signifying the length of the
            // string in bytes
            STRING_ID => Nbt::String(read_string(stream)?),
            // A list of nameless tags, all of the same type. The list is
            // prefixed with the Type ID of the items it contains (thus 1
            // byte), and the length of the list as a signed integer (a further
            // 4 bytes). If the length of the list is 0 or negative, the type
            // may be 0 (TAG_End) but otherwise it must be any other type. (The
            // notchian implementation uses TAG_End in that situation, but
            // another reference implementation by Mojang uses 1 instead;
            // parsers should accept any type if the length is <= 0).
            LIST_ID => Nbt::List(read_list(stream)?),
            // Effectively a list of a named tags. Order is not guaranteed.
            COMPOUND_ID => Nbt::Compound(read_compound(stream)?),
            // A length-prefixed array of signed integers. The prefix is a
            // signed integer (thus 4 bytes) and indicates the number of 4 byte
            // integers.
            INT_ARRAY_ID => Nbt::IntArray(read_int_array(stream)?),
            // A length-prefixed array of signed longs. The prefix is a signed
            // integer (thus 4 bytes) and indicates the number of 8 byte longs.
            LONG_ARRAY_ID => Nbt::LongArray(read_long_array(stream)?),
            _ => return Err(Error::InvalidTagType(id)),
        })
    }

    /// Read the NBT data. This will return a compound tag with a single item.
    pub fn read(stream: &mut Cursor<&[u8]>) -> Result<Nbt, Error> {
        // default to compound tag

        // the parent compound only ever has one item
        let tag_id = stream.read_u8().unwrap_or(0);
        if tag_id == 0 {
            return Ok(Nbt::End);
        }
        let name = read_string(stream)?;
        let tag = Nbt::read_known(stream, tag_id)?;
        let mut map = NbtCompound::with_capacity(1);
        map.insert_unsorted(name, tag);

        Ok(Nbt::Compound(map))
    }

    /// Read the NBT data compressed wtih zlib.
    pub fn read_zlib(stream: &mut impl BufRead) -> Result<Nbt, Error> {
        let mut gz = ZlibDecoder::new(stream);
        let mut buf = Vec::new();
        gz.read_to_end(&mut buf)?;
        Nbt::read(&mut Cursor::new(&buf))
    }

    /// Read the NBT data compressed wtih gzip.
    pub fn read_gzip(stream: &mut Cursor<Vec<u8>>) -> Result<Nbt, Error> {
        let mut gz = GzDecoder::new(stream);
        let mut buf = Vec::new();
        gz.read_to_end(&mut buf)?;
        Nbt::read(&mut Cursor::new(&buf))
    }
}

impl McBufReadable for Nbt {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(Nbt::read(buf)?)
    }
}
impl From<Error> for BufReadError {
    fn from(e: Error) -> Self {
        BufReadError::Custom(e.to_string())
    }
}
