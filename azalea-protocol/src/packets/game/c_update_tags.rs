use std::io::Cursor;
use std::ops::Deref;
use std::{collections::HashMap, io::Write};

use azalea_buf::{AzBuf, AzaleaReadVar, AzaleaWriteVar, BufReadError};
use azalea_buf::{AzaleaRead, AzaleaWrite};
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateTags {
    pub tags: TagMap,
}

#[derive(Clone, Debug)]
pub struct Tags {
    pub name: ResourceLocation,
    pub elements: Vec<i32>,
}

#[derive(Clone, Debug)]
pub struct TagMap(pub HashMap<ResourceLocation, Vec<Tags>>);

impl AzaleaRead for TagMap {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let length = u32::azalea_read_var(buf)? as usize;
        let mut data = HashMap::with_capacity(length);
        for _ in 0..length {
            let tag_type = ResourceLocation::azalea_read(buf)?;
            let tags_count = i32::azalea_read_var(buf)? as usize;
            let mut tags_vec = Vec::with_capacity(tags_count);
            for _ in 0..tags_count {
                let tags = Tags::azalea_read(buf)?;
                tags_vec.push(tags);
            }
            data.insert(tag_type, tags_vec);
        }
        Ok(TagMap(data))
    }
}

impl AzaleaWrite for TagMap {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (self.len() as u32).azalea_write_var(buf)?;
        for (k, v) in &self.0 {
            k.azalea_write(buf)?;
            v.azalea_write(buf)?;
        }
        Ok(())
    }
}
impl AzaleaRead for Tags {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let name = ResourceLocation::azalea_read(buf)?;
        let elements = Vec::<i32>::azalea_read_var(buf)?;
        Ok(Tags { name, elements })
    }
}

impl AzaleaWrite for Tags {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.name.azalea_write(buf)?;
        self.elements.azalea_write_var(buf)?;
        Ok(())
    }
}

impl Deref for TagMap {
    type Target = HashMap<ResourceLocation, Vec<Tags>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
