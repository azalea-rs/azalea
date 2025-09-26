use std::{
    io::{self, Cursor, Write},
    ops::Deref,
};

use azalea_buf::{AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};
use azalea_core::resource_location::ResourceLocation;
use indexmap::IndexMap;

#[derive(Clone, Debug, PartialEq)]
pub struct TagMap(pub IndexMap<ResourceLocation, Vec<Tags>>);

#[derive(Clone, Debug, PartialEq)]
pub struct Tags {
    pub name: ResourceLocation,
    pub elements: Vec<i32>,
}

impl AzaleaRead for TagMap {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let length = u32::azalea_read_var(buf)? as usize;
        let mut data = IndexMap::with_capacity(length);
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
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
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
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        self.name.azalea_write(buf)?;
        self.elements.azalea_write_var(buf)?;
        Ok(())
    }
}

impl Deref for TagMap {
    type Target = IndexMap<ResourceLocation, Vec<Tags>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
