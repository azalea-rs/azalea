use azalea_buf::{BufReadError, McBuf, McBufVarReadable, McBufVarWritable};
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_core::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::Cursor;
use std::ops::Deref;
use std::{collections::HashMap, io::Write};

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
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let length = u32::var_read_from(buf)? as usize;
        let mut data = HashMap::with_capacity(length);
        for _ in 0..length {
            let tag_type = ResourceLocation::read_from(buf)?;
            let tags_count = i32::var_read_from(buf)? as usize;
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
        (self.len() as u32).var_write_into(buf)?;
        for (k, v) in &self.0 {
            k.write_into(buf)?;
            v.write_into(buf)?;
        }
        Ok(())
    }
}
impl McBufReadable for Tags {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let name = ResourceLocation::read_from(buf)?;
        let elements = Vec::<i32>::var_read_from(buf)?;
        Ok(Tags { name, elements })
    }
}

impl McBufWritable for Tags {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.name.write_into(buf)?;
        self.elements.var_write_into(buf)?;
        Ok(())
    }
}

impl Deref for TagMap {
    type Target = HashMap<ResourceLocation, Vec<Tags>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
