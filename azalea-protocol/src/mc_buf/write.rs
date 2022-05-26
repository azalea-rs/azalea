use super::{UnsizedByteArray, MAX_STRING_LENGTH};
use azalea_chat::component::Component;
use azalea_core::{
    difficulty::Difficulty, game_type::GameType, resource_location::ResourceLocation,
    serializable_uuid::SerializableUuid, BlockPos, ChunkSectionPos, Direction, GlobalPos, Slot,
};
use azalea_crypto::SaltSignaturePair;
use byteorder::{BigEndian, WriteBytesExt};
use std::{collections::HashMap, io::Write};
use uuid::Uuid;

pub trait Writable: Write {
    fn write_list<F, T>(&mut self, list: &[T], writer: F) -> Result<(), std::io::Error>
    where
        F: FnOnce(&mut Self, &T) -> Result<(), std::io::Error> + Copy,
    {
        self.write_varint(list.len() as i32)?;
        for item in list {
            writer(self, item)?;
        }
        Ok(())
    }

    fn write_int_id_list(&mut self, list: &[i32]) -> Result<(), std::io::Error> {
        self.write_list(list, |buf, n| buf.write_varint(*n))
    }

    fn write_map<KF, VF, KT, VT>(
        &mut self,
        map: Vec<(KT, VT)>,
        key_writer: KF,
        value_writer: VF,
    ) -> Result<(), std::io::Error>
    where
        KF: Fn(&mut Self, KT) -> Result<(), std::io::Error> + Copy,
        VF: Fn(&mut Self, VT) -> Result<(), std::io::Error> + Copy,
    {
        self.write_varint(map.len() as i32)?;
        for (key, value) in map {
            key_writer(self, key)?;
            value_writer(self, value)?;
        }
        Ok(())
    }

    fn write_byte(&mut self, n: u8) -> Result<(), std::io::Error> {
        WriteBytesExt::write_u8(self, n)
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), std::io::Error> {
        self.write_all(bytes)?;
        Ok(())
    }

    fn write_varint(&mut self, mut value: i32) -> Result<(), std::io::Error> {
        let mut buffer = [0];
        if value == 0 {
            self.write_all(&buffer).unwrap();
        }
        while value != 0 {
            buffer[0] = (value & 0b0111_1111) as u8;
            value = (value >> 7) & (i32::max_value() >> 6);
            if value != 0 {
                buffer[0] |= 0b1000_0000;
            }
            self.write_all(&buffer)?;
        }
        Ok(())
    }

    fn write_utf_with_len(&mut self, string: &str, len: usize) -> Result<(), std::io::Error> {
        if string.len() > len {
            panic!(
                "String too big (was {} bytes encoded, max {})",
                string.len(),
                len
            );
        }
        self.write_varint(string.len() as i32)?;
        self.write_bytes(string.as_bytes())
    }

    fn write_utf(&mut self, string: &str) -> Result<(), std::io::Error> {
        self.write_utf_with_len(string, MAX_STRING_LENGTH.into())
    }

    fn write_short(&mut self, n: i16) -> Result<(), std::io::Error> {
        WriteBytesExt::write_i16::<BigEndian>(self, n)
    }

    fn write_byte_array(&mut self, bytes: &[u8]) -> Result<(), std::io::Error> {
        self.write_varint(bytes.len() as i32)?;
        self.write_bytes(bytes)
    }

    fn write_int(&mut self, n: i32) -> Result<(), std::io::Error> {
        WriteBytesExt::write_i32::<BigEndian>(self, n)
    }

    fn write_boolean(&mut self, b: bool) -> Result<(), std::io::Error> {
        self.write_byte(if b { 1 } else { 0 })
    }

    fn write_nbt(&mut self, nbt: &azalea_nbt::Tag) -> Result<(), std::io::Error>
    where
        Self: Sized,
    {
        nbt.write(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }

    fn write_long(&mut self, n: i64) -> Result<(), std::io::Error> {
        WriteBytesExt::write_i64::<BigEndian>(self, n)
    }

    fn write_float(&mut self, n: f32) -> Result<(), std::io::Error> {
        WriteBytesExt::write_f32::<BigEndian>(self, n)
    }

    fn write_double(&mut self, n: f64) -> Result<(), std::io::Error> {
        WriteBytesExt::write_f64::<BigEndian>(self, n)
    }

    fn write_resource_location(
        &mut self,
        location: &ResourceLocation,
    ) -> Result<(), std::io::Error> {
        self.write_utf(&location.to_string())
    }

    fn write_uuid(&mut self, uuid: &Uuid) -> Result<(), std::io::Error>
    where
        Self: Sized,
    {
        let [a, b, c, d] = uuid.to_int_array();
        a.write_into(self)?;
        b.write_into(self)?;
        c.write_into(self)?;
        d.write_into(self)?;
        Ok(())
    }
}

impl<W: Write + ?Sized> Writable for W {}

pub trait McBufWritable {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

pub trait McBufVarWritable {
    fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

impl McBufWritable for i32 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        Writable::write_int(buf, *self)
    }
}

impl McBufVarWritable for i32 {
    fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_varint(*self)
    }
}

impl McBufWritable for UnsizedByteArray {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_bytes(self)
    }
}

impl<T: McBufWritable> McBufWritable for Vec<T> {
    default fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_list(self, |buf, i| T::write_into(i, buf))
    }
}

impl<K: McBufWritable, V: McBufWritable> McBufWritable for HashMap<K, V> {
    default fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u32::var_write_into(&(self.len() as u32), buf)?;
        for (key, value) in self {
            key.write_into(buf)?;
            value.write_into(buf)?;
        }

        Ok(())
    }
}

impl McBufWritable for Vec<u8> {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_byte_array(self)
    }
}

impl McBufWritable for String {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_utf(self)
    }
}

impl McBufWritable for ResourceLocation {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_resource_location(self)
    }
}

impl McBufWritable for u32 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i16::write_into(&(*self as i16), buf)
    }
}

impl McBufVarWritable for u32 {
    fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i32::var_write_into(&(*self as i32), buf)
    }
}

impl McBufVarWritable for i64 {
    fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut buffer = [0];
        let mut cnt = 0;
        let mut value = *self;
        while value != 0 {
            buffer[0] = (value & 0b0111_1111) as u8;
            value = (value >> 7) & (i64::max_value() >> 6);
            if value != 0 {
                buffer[0] |= 0b1000_0000;
            }
            cnt += buf.write(&mut buffer)?;
        }
        Ok(())
    }
}

impl McBufVarWritable for u64 {
    fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i64::var_write_into(&(*self as i64), buf)
    }
}

impl McBufWritable for u16 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i16::write_into(&(*self as i16), buf)
    }
}

impl McBufVarWritable for u16 {
    fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i32::var_write_into(&(*self as i32), buf)
    }
}

impl<T: McBufVarWritable> McBufVarWritable for Vec<T> {
    fn var_write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u32::var_write_into(&(self.len() as u32), buf)?;
        for i in self {
            i.var_write_into(buf)?;
        }
        Ok(())
    }
}

impl McBufWritable for u8 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_byte(*self)
    }
}

impl McBufWritable for i16 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        Writable::write_short(buf, *self)
    }
}

impl McBufWritable for i64 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        Writable::write_long(buf, *self)
    }
}

impl McBufWritable for u64 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        i64::write_into(&(*self as i64), buf)
    }
}

impl McBufWritable for bool {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_boolean(*self)
    }
}

impl McBufWritable for i8 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_byte(*self as u8)
    }
}

impl McBufWritable for f32 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_float(*self)
    }
}

impl McBufWritable for f64 {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_double(*self)
    }
}

impl McBufWritable for GameType {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u8::write_into(&self.to_id(), buf)
    }
}

impl McBufWritable for Option<GameType> {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_byte(GameType::to_optional_id(self) as u8)
    }
}

impl<T: McBufWritable> McBufWritable for Option<T> {
    default fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        if let Some(s) = self {
            buf.write_boolean(true)?;
            s.write_into(buf)?;
        } else {
            buf.write_boolean(false)?;
        };
        Ok(())
    }
}

impl McBufWritable for azalea_nbt::Tag {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_nbt(self)
    }
}

impl McBufWritable for Difficulty {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u8::write_into(&self.id(), buf)
    }
}

impl McBufWritable for Component {
    // async fn read_into(buf: &mut impl Read) -> Result<Self, String>
    // where
    //     R: AsyncRead + std::marker::Unpin + std::marker::Send,
    // {
    //     let string = buf.read_utf().await?;
    //     let json: serde_json::Value = serde_json::from_str(string.as_str())
    //         .map_err(|e| "Component isn't valid JSON".to_string())?;
    //     let component = Component::deserialize(json).map_err(|e| e.to_string())?;
    //     Ok(component)
    // }
    fn write_into(&self, _buf: &mut impl Write) -> Result<(), std::io::Error> {
        // component doesn't have serialize implemented yet
        todo!()
    }
}

impl McBufWritable for Slot {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            Slot::Empty => buf.write_byte(0)?,
            Slot::Present(i) => {
                buf.write_varint(i.id)?;
                buf.write_byte(i.count)?;
                buf.write_nbt(&i.nbt)?;
            }
        }

        Ok(())
    }
}

impl McBufWritable for Uuid {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_uuid(self)?;

        Ok(())
    }
}

impl McBufWritable for BlockPos {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_long(
            (((self.x & 0x3FFFFFF) as i64) << 38)
                | (((self.z & 0x3FFFFFF) as i64) << 12)
                | ((self.y & 0xFFF) as i64),
        )
    }
}

impl McBufWritable for GlobalPos {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        BlockPos::write_into(&self.pos, buf)?;
        ResourceLocation::write_into(&self.dimension, buf)?;

        Ok(())
    }
}

impl McBufWritable for Direction {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_varint(*self as i32)
    }
}

impl McBufWritable for ChunkSectionPos {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let long = (((self.x & 0x3FFFFF) as i64) << 42)
            | (self.y & 0xFFFFF) as i64
            | (((self.z & 0x3FFFFF) as i64) << 20);
        long.write_into(buf)?;
        Ok(())
    }
}

impl McBufWritable for SaltSignaturePair {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.salt.write_into(buf)?;
        self.signature.write_into(buf)?;
        Ok(())
    }
}
