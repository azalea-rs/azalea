use crate::tag::NbtByteArray;
use crate::tag::NbtCompound;
use crate::tag::NbtIntArray;
use crate::tag::NbtList;
use crate::tag::NbtLongArray;
use crate::tag::NbtString;
use crate::Error;
use crate::Tag;
use ahash::AHashMap;
use azalea_buf::{BufReadError, McBufReadable};
use byteorder::{ReadBytesExt, BE};
use flate2::read::{GzDecoder, ZlibDecoder};
use log::warn;
use std::io::Cursor;
use std::io::{BufRead, Read};

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
        0 => NbtList::Empty,
        1 => NbtList::Byte(vec_u8_into_i8(
            read_bytes(stream, length as usize)?.to_vec(),
        )),
        2 => NbtList::Short({
            if ((length * 2) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| stream.read_i16::<BE>())
                .collect::<Result<Vec<_>, _>>()?
        }),
        3 => NbtList::Int({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| stream.read_i32::<BE>())
                .collect::<Result<Vec<_>, _>>()?
        }),
        4 => NbtList::Long({
            if ((length * 8) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| stream.read_i64::<BE>())
                .collect::<Result<Vec<_>, _>>()?
        }),
        5 => NbtList::Float({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| stream.read_f32::<BE>())
                .collect::<Result<Vec<_>, _>>()?
        }),
        6 => NbtList::Double({
            if ((length * 8) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| stream.read_f64::<BE>())
                .collect::<Result<Vec<_>, _>>()?
        }),
        7 => NbtList::ByteArray({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| read_byte_array(stream))
                .collect::<Result<Vec<_>, _>>()?
        }),
        8 => NbtList::String({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| read_string(stream))
                .collect::<Result<Vec<_>, _>>()?
        }),
        9 => NbtList::List({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| read_list(stream))
                .collect::<Result<Vec<_>, _>>()?
        }),
        10 => NbtList::Compound({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| read_compound(stream))
                .collect::<Result<Vec<_>, _>>()?
        }),
        11 => NbtList::IntArray({
            if ((length * 4) as usize) > (stream.get_ref().len() - stream.position() as usize) {
                return Err(Error::UnexpectedEof);
            }
            (0..length)
                .map(|_| read_int_array(stream))
                .collect::<Result<Vec<_>, _>>()?
        }),
        12 => NbtList::LongArray({
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
        let tag = Tag::read_known(stream, tag_id)?;
        map.insert(name, tag);
    }
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

impl Tag {
    /// Read the NBT data when you already know the ID of the tag. You usually
    /// want [`Tag::read`] if you're reading an NBT file.
    #[inline]
    fn read_known(stream: &mut Cursor<&[u8]>, id: u8) -> Result<Tag, Error> {
        Ok(match id {
            // Signifies the end of a TAG_Compound. It is only ever used inside
            // a TAG_Compound, and is not named despite being in a TAG_Compound
            0 => Tag::End,
            // A single signed byte
            1 => Tag::Byte(stream.read_i8()?),
            // A single signed, big endian 16 bit integer
            2 => Tag::Short(stream.read_i16::<BE>()?),
            // A single signed, big endian 32 bit integer
            3 => Tag::Int(stream.read_i32::<BE>()?),
            // A single signed, big endian 64 bit integer
            4 => Tag::Long(stream.read_i64::<BE>()?),
            // A single, big endian IEEE-754 single-precision floating point
            // number (NaN possible)
            5 => Tag::Float(stream.read_f32::<BE>()?),
            // A single, big endian IEEE-754 double-precision floating point
            // number (NaN possible)
            6 => Tag::Double(stream.read_f64::<BE>()?),
            // A length-prefixed array of signed bytes. The prefix is a signed
            // integer (thus 4 bytes)
            7 => Tag::ByteArray(read_byte_array(stream)?),
            // A length-prefixed modified UTF-8 string. The prefix is an
            // unsigned short (thus 2 bytes) signifying the length of the
            // string in bytes
            8 => Tag::String(read_string(stream)?),
            // A list of nameless tags, all of the same type. The list is
            // prefixed with the Type ID of the items it contains (thus 1
            // byte), and the length of the list as a signed integer (a further
            // 4 bytes). If the length of the list is 0 or negative, the type
            // may be 0 (TAG_End) but otherwise it must be any other type. (The
            // notchian implementation uses TAG_End in that situation, but
            // another reference implementation by Mojang uses 1 instead;
            // parsers should accept any type if the length is <= 0).
            9 => Tag::List(read_list(stream)?),
            // Effectively a list of a named tags. Order is not guaranteed.
            10 => Tag::Compound(read_compound(stream)?),
            // A length-prefixed array of signed integers. The prefix is a
            // signed integer (thus 4 bytes) and indicates the number of 4 byte
            // integers.
            11 => Tag::IntArray(read_int_array(stream)?),
            // A length-prefixed array of signed longs. The prefix is a signed
            // integer (thus 4 bytes) and indicates the number of 8 byte longs.
            12 => Tag::LongArray(read_long_array(stream)?),
            _ => return Err(Error::InvalidTagType(id)),
        })
    }

    /// Read the NBT data. This will return a compound tag with a single item.
    pub fn read(stream: &mut Cursor<&[u8]>) -> Result<Tag, Error> {
        // default to compound tag

        // the parent compound only ever has one item
        let tag_id = stream.read_u8().unwrap_or(0);
        if tag_id == 0 {
            return Ok(Tag::End);
        }
        let name = read_string(stream)?;
        let tag = Tag::read_known(stream, tag_id)?;
        let mut map = AHashMap::with_capacity(1);
        map.insert(name, tag);

        Ok(Tag::Compound(map))
    }

    /// Read the NBT data compressed wtih zlib.
    pub fn read_zlib(stream: &mut impl BufRead) -> Result<Tag, Error> {
        let mut gz = ZlibDecoder::new(stream);
        let mut buf = Vec::new();
        gz.read_to_end(&mut buf)?;
        Tag::read(&mut Cursor::new(&buf))
    }

    /// Read the NBT data compressed wtih gzip.
    pub fn read_gzip(stream: &mut Cursor<Vec<u8>>) -> Result<Tag, Error> {
        let mut gz = GzDecoder::new(stream);
        let mut buf = Vec::new();
        gz.read_to_end(&mut buf)?;
        Tag::read(&mut Cursor::new(&buf))
    }
}

impl McBufReadable for Tag {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(Tag::read(buf)?)
    }
}
impl From<Error> for BufReadError {
    fn from(e: Error) -> Self {
        BufReadError::Custom(e.to_string())
    }
}
