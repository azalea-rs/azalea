use super::{UnsizedByteArray, MAX_STRING_LENGTH};
use azalea_chat::component::Component;
use azalea_core::{
    difficulty::Difficulty, game_type::GameType, resource_location::ResourceLocation,
    serializable_uuid::SerializableUuid, BlockPos, ChunkSectionPos, Direction, Slot, SlotData,
};
use byteorder::{ReadBytesExt, BE};
use serde::Deserialize;
use std::{collections::HashMap, hash::Hash, io::Read};
use tokio::io::{AsyncRead, AsyncReadExt};
use uuid::Uuid;

pub trait Readable {
    fn read_int_id_list(&mut self) -> Result<Vec<i32>, String>;
    fn read_varint(&mut self) -> Result<i32, String>;
    fn get_varint_size(&mut self, value: i32) -> u8;
    fn get_varlong_size(&mut self, value: i32) -> u8;
    fn read_byte_array(&mut self) -> Result<Vec<u8>, String>;
    fn read_bytes_with_len(&mut self, n: usize) -> Result<Vec<u8>, String>;
    fn read_bytes(&mut self) -> Result<Vec<u8>, String>;
    fn read_utf(&mut self) -> Result<String, String>;
    fn read_utf_with_len(&mut self, max_length: u32) -> Result<String, String>;
    fn read_byte(&mut self) -> Result<u8, String>;
    fn read_int(&mut self) -> Result<i32, String>;
    fn read_boolean(&mut self) -> Result<bool, String>;
    fn read_nbt(&mut self) -> Result<azalea_nbt::Tag, String>;
    fn read_long(&mut self) -> Result<i64, String>;
    fn read_resource_location(&mut self) -> Result<ResourceLocation, String>;
    fn read_short(&mut self) -> Result<i16, String>;
    fn read_float(&mut self) -> Result<f32, String>;
    fn read_double(&mut self) -> Result<f64, String>;
    fn read_uuid(&mut self) -> Result<Uuid, String>;
}

impl<R> Readable for R
where
    R: Read,
{
    fn read_int_id_list(&mut self) -> Result<Vec<i32>, String> {
        let len = self.read_varint()?;
        let mut list = Vec::with_capacity(len as usize);
        for _ in 0..len {
            list.push(self.read_varint()?);
        }
        Ok(list)
    }

    // fast varints modified from https://github.com/luojia65/mc-varint/blob/master/src/lib.rs#L67
    /// Read a single varint from the reader and return the value, along with the number of bytes read
    fn read_varint(&mut self) -> Result<i32, String> {
        let mut buffer = [0];
        let mut ans = 0;
        for i in 0..5 {
            self.read_exact(&mut buffer)
                .map_err(|_| "Invalid VarInt".to_string())?;
            ans |= ((buffer[0] & 0b0111_1111) as i32) << (7 * i);
            if buffer[0] & 0b1000_0000 == 0 {
                return Ok(ans);
            }
        }
        Ok(ans)
    }

    fn get_varint_size(&mut self, value: i32) -> u8 {
        for i in 1..5 {
            if (value & -1 << (i * 7)) != 0 {
                continue;
            }
            return i;
        }
        5
    }

    fn get_varlong_size(&mut self, value: i32) -> u8 {
        for i in 1..10 {
            if (value & -1 << (i * 7)) != 0 {
                continue;
            }
            return i;
        }
        10
    }

    fn read_byte_array(&mut self) -> Result<Vec<u8>, String> {
        let length = self.read_varint()? as usize;
        self.read_bytes_with_len(length)
    }

    fn read_bytes_with_len(&mut self, n: usize) -> Result<Vec<u8>, String> {
        let mut buffer = vec![0; n];
        self.read_exact(&mut buffer)
            .map_err(|_| "Error reading bytes".to_string())?;
        Ok(buffer)
    }

    fn read_bytes(&mut self) -> Result<Vec<u8>, String> {
        // read to end of the buffer
        let mut bytes = vec![];
        self.read_to_end(&mut bytes)
            .map_err(|_| "Error reading bytes".to_string())?;
        Ok(bytes)
    }

    fn read_utf(&mut self) -> Result<String, String> {
        self.read_utf_with_len(MAX_STRING_LENGTH.into())
    }

    fn read_utf_with_len(&mut self, max_length: u32) -> Result<String, String> {
        let length = self.read_varint()?;
        // i don't know why it's multiplied by 4 but it's like that in mojang's code so
        if length < 0 {
            return Err(
                "The received encoded string buffer length is less than zero! Weird string!"
                    .to_string(),
            );
        }
        if length as u32 > max_length * 4 {
            return Err(format!(
                "The received encoded string buffer length is longer than maximum allowed ({} > {})",
                length,
                max_length * 4
            ));
        }

        // this is probably quite inefficient, idk how to do it better
        let mut string = String::new();
        let mut buffer = vec![0; length as usize];
        self.read_exact(&mut buffer)
            .map_err(|_| "Invalid UTF-8".to_string())?;
        string.push_str(std::str::from_utf8(&buffer).unwrap());
        if string.len() > length as usize {
            return Err(format!(
                "The received string length is longer than maximum allowed ({} > {})",
                length, max_length
            ));
        }

        Ok(string)
    }

    /// Read a single byte from the reader
    fn read_byte(&mut self) -> Result<u8, String> {
        self.read_u8().map_err(|_| "Error reading byte".to_string())
    }

    fn read_int(&mut self) -> Result<i32, String> {
        match self.read_i32::<BE>() {
            Ok(r) => Ok(r),
            Err(_) => Err("Error reading int".to_string()),
        }
    }

    fn read_boolean(&mut self) -> Result<bool, String> {
        match self.read_byte()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err("Error reading boolean".to_string()),
        }
    }

    fn read_nbt(&mut self) -> Result<azalea_nbt::Tag, String> {
        match azalea_nbt::Tag::read(self) {
            Ok(r) => Ok(r),
            // Err(e) => Err(e.to_string()),
            Err(e) => Err(e.to_string()).unwrap(),
        }
    }

    fn read_long(&mut self) -> Result<i64, String> {
        match self.read_i64::<BE>() {
            Ok(r) => Ok(r),
            Err(_) => Err("Error reading long".to_string()),
        }
    }

    fn read_resource_location(&mut self) -> Result<ResourceLocation, String> {
        // get the resource location from the string
        let location_string = self.read_utf()?;
        let location = ResourceLocation::new(&location_string)?;
        Ok(location)
    }

    fn read_short(&mut self) -> Result<i16, String> {
        match self.read_i16::<BE>() {
            Ok(r) => Ok(r),
            Err(_) => Err("Error reading short".to_string()),
        }
    }

    fn read_float(&mut self) -> Result<f32, String> {
        match self.read_f32::<BE>() {
            Ok(r) => Ok(r),
            Err(_) => Err("Error reading float".to_string()),
        }
    }

    fn read_double(&mut self) -> Result<f64, String> {
        match self.read_f64::<BE>() {
            Ok(r) => Ok(r),
            Err(_) => Err("Error reading double".to_string()),
        }
    }

    fn read_uuid(&mut self) -> Result<Uuid, String> {
        Ok(Uuid::from_int_array([
            Readable::read_int(self)? as u32,
            Readable::read_int(self)? as u32,
            Readable::read_int(self)? as u32,
            Readable::read_int(self)? as u32,
        ]))
    }
}

// fast varints modified from https://github.com/luojia65/mc-varint/blob/master/src/lib.rs#L67
/// Read a single varint from the reader and return the value, along with the number of bytes read
pub async fn read_varint_async(reader: &mut (dyn AsyncRead + Unpin + Send)) -> Result<i32, String> {
    let mut buffer = [0];
    let mut ans = 0;
    for i in 0..5 {
        reader
            .read_exact(&mut buffer)
            .await
            .map_err(|_| "Invalid VarInt".to_string())?;
        ans |= ((buffer[0] & 0b0111_1111) as i32) << (7 * i);
        if buffer[0] & 0b1000_0000 == 0 {
            return Ok(ans);
        }
    }
    Ok(ans)
}

pub trait McBufReadable
where
    Self: Sized,
{
    fn read_into(buf: &mut impl Read) -> Result<Self, String>;
}

pub trait McBufVarReadable
where
    Self: Sized,
{
    fn var_read_into(buf: &mut impl Read) -> Result<Self, String>;
}

impl McBufReadable for i32 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        Readable::read_int(buf)
    }
}

impl McBufVarReadable for i32 {
    fn var_read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_varint()
    }
}

impl McBufVarReadable for i64 {
    // fast varints modified from https://github.com/luojia65/mc-varint/blob/master/src/lib.rs#L54
    fn var_read_into(buf: &mut impl Read) -> Result<Self, String> {
        let mut buffer = [0];
        let mut ans = 0;
        for i in 0..8 {
            buf.read_exact(&mut buffer)
                .map_err(|_| "Invalid VarLong".to_string())?;
            ans |= ((buffer[0] & 0b0111_1111) as i64) << 7 * i;
            if buffer[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(ans)
    }
}
impl McBufVarReadable for u64 {
    fn var_read_into(buf: &mut impl Read) -> Result<Self, String> {
        i64::var_read_into(buf).map(|i| i as u64)
    }
}

impl McBufReadable for UnsizedByteArray {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        Ok(buf.read_bytes()?.into())
    }
}

impl<T: McBufReadable + Send> McBufReadable for Vec<T> {
    default fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let length = buf.read_varint()? as usize;
        let mut contents = Vec::with_capacity(length);
        for _ in 0..length {
            contents.push(T::read_into(buf)?);
        }
        Ok(contents)
    }
}

impl<K: McBufReadable + Send + Eq + Hash, V: McBufReadable + Send> McBufReadable for HashMap<K, V> {
    default fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let length = buf.read_varint()? as usize;
        let mut contents = HashMap::with_capacity(length);
        for _ in 0..length {
            contents.insert(K::read_into(buf)?, V::read_into(buf)?);
        }
        Ok(contents)
    }
}

impl McBufReadable for Vec<u8> {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_byte_array()
    }
}

// string
impl McBufReadable for String {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_utf()
    }
}

// ResourceLocation
impl McBufReadable for ResourceLocation {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_resource_location()
    }
}

// u32
impl McBufReadable for u32 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        Readable::read_int(buf).map(|i| i as u32)
    }
}

// u32 varint
impl McBufVarReadable for u32 {
    fn var_read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_varint().map(|i| i as u32)
    }
}

// u16
impl McBufReadable for u16 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_short().map(|i| i as u16)
    }
}

// i16
impl McBufReadable for i16 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_short()
    }
}

// u16 varint
impl McBufVarReadable for u16 {
    fn var_read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_varint().map(|i| i as u16)
    }
}

// Vec<T> varint
impl<T: McBufVarReadable> McBufVarReadable for Vec<T> {
    fn var_read_into(buf: &mut impl Read) -> Result<Self, String> {
        let length = buf.read_varint()? as usize;
        let mut contents = Vec::with_capacity(length);
        for _ in 0..length {
            contents.push(T::var_read_into(buf)?);
        }
        Ok(contents)
    }
}

// i64
impl McBufReadable for i64 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_long()
    }
}

// u64
impl McBufReadable for u64 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        i64::read_into(buf).map(|i| i as u64)
    }
}

// bool
impl McBufReadable for bool {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_boolean()
    }
}

// u8
impl McBufReadable for u8 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_byte()
    }
}

// i8
impl McBufReadable for i8 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_byte().map(|i| i as i8)
    }
}

// f32
impl McBufReadable for f32 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_float()
    }
}

// f64
impl McBufReadable for f64 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_double()
    }
}

// GameType
impl McBufReadable for GameType {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        GameType::from_id(buf.read_byte()?)
    }
}

// Option<GameType>
impl McBufReadable for Option<GameType> {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        GameType::from_optional_id(buf.read_byte()? as i8)
    }
}

// Option<String>
impl<T: McBufReadable> McBufReadable for Option<T> {
    default fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let present = buf.read_boolean()?;
        Ok(if present {
            Some(T::read_into(buf)?)
        } else {
            None
        })
    }
}

// azalea_nbt::Tag
impl McBufReadable for azalea_nbt::Tag {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_nbt()
    }
}

// Difficulty
impl McBufReadable for Difficulty {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        Ok(Difficulty::by_id(u8::read_into(buf)?))
    }
}

// Component
impl McBufReadable for Component {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let string = buf.read_utf()?;
        let json: serde_json::Value = serde_json::from_str(string.as_str())
            .map_err(|_| "Component isn't valid JSON".to_string())?;
        let component = Component::deserialize(json).map_err(|e| e.to_string())?;
        Ok(component)
    }
}

// Slot
impl McBufReadable for Slot {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let present = buf.read_boolean()?;
        if !present {
            return Ok(Slot::Empty);
        }
        let id = buf.read_varint()?;
        let count = buf.read_byte()?;
        let nbt = buf.read_nbt()?;
        Ok(Slot::Present(SlotData { id, count, nbt }))
    }
}

// Uuid
impl McBufReadable for Uuid {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_uuid()
    }
}

// BlockPos
impl McBufReadable for BlockPos {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let val = u64::read_into(buf)?;
        let x = (val >> 38) as i32;
        let y = (val & 0xFFF) as i32;
        let z = ((val >> 12) & 0x3FFFFFF) as i32;
        Ok(BlockPos { x, y, z })
    }
}

// Direction
impl McBufReadable for Direction {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        match buf.read_varint()? {
            0 => Ok(Self::Down),
            1 => Ok(Self::Up),
            2 => Ok(Self::North),
            3 => Ok(Self::South),
            4 => Ok(Self::West),
            5 => Ok(Self::East),
            _ => Err("Invalid direction".to_string()),
        }
    }
}

// ChunkSectionPos
impl McBufReadable for ChunkSectionPos {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let long = i64::read_into(buf)?;
        Ok(ChunkSectionPos {
            x: (long >> 42) as i32,
            y: (long << 44 >> 44) as i32,
            z: (long << 22 >> 42) as i32,
        })
    }
}
