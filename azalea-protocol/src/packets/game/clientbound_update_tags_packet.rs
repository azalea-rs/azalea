use azalea_buf::McBuf;
use azalea_buf::{McBufReadable, McBufWritable, Readable, Writable};
use azalea_core::ResourceLocation;
use packet_macros::ClientboundGamePacket;
use std::ops::Deref;
use std::{
    collections::HashMap,
    io::{Read, Write},
};

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateTagsPacket {
    pub tags: TagMap,
}

#[derive(Clone, Debug)]
pub struct Tags {
    pub name: ResourceLocation,
    pub elements: Vec<i32>,
}

#[derive(Clone, Debug)]
pub struct TagMap(HashMap<ResourceLocation, Vec<Tags>>);

impl McBufReadable for TagMap {
    fn read_from(buf: &mut impl Read) -> Result<Self, String> {
        let length = buf.read_varint()? as usize;
        let mut data = HashMap::with_capacity(length);
        for _ in 0..length {
            let tag_type = ResourceLocation::read_from(buf)?;
            let tags_count = buf.read_varint()? as usize;
            let mut tags_vec = Vec::with_capacity(tags_count);
            for _ in 0..tags_count {
                let tags = Tags::read_from(buf)?;
                tags_vec.push(tags);
            }
            data.insert(tag_type, tags_vec);
        }
        Ok(TagMap(data))
    }
}

impl McBufWritable for TagMap {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_varint(self.len() as i32)?;
        for (k, v) in &self.0 {
            k.write_into(buf)?;
            v.write_into(buf)?;
        }
        Ok(())
    }
}
impl McBufReadable for Tags {
    fn read_from(buf: &mut impl Read) -> Result<Self, String> {
        let name = ResourceLocation::read_from(buf)?;
        let elements = buf.read_int_id_list()?;
        Ok(Tags { name, elements })
    }
}

impl McBufWritable for Tags {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.name.write_into(buf)?;
        buf.write_int_id_list(&self.elements)?;
        Ok(())
    }
}

impl Deref for TagMap {
    type Target = HashMap<ResourceLocation, Vec<Tags>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
