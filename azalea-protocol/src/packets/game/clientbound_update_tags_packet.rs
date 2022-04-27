use std::collections::HashMap;

use async_trait::async_trait;
use azalea_core::resource_location::ResourceLocation;
use packet_macros::GamePacket;
use tokio::io::AsyncRead;

use crate::mc_buf::{McBufReadable, McBufWritable, Readable, Writable};

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundUpdateTagsPacket {
    pub tags: HashMap<ResourceLocation, Vec<Tags>>,
}

#[derive(Clone, Debug)]
pub struct Tags {
    pub name: ResourceLocation,
    pub elements: Vec<i32>,
}

#[async_trait]
impl McBufReadable for HashMap<ResourceLocation, Vec<Tags>> {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        let length = buf.read_varint().await? as usize;
        let mut data = HashMap::with_capacity(length);
        for _ in 0..length {
            let tag_type = buf.read_resource_location().await?;
            let tags_count = buf.read_varint().await? as usize;
            let mut tags_vec = Vec::with_capacity(tags_count);
            for _ in 0..tags_count {
                let tags = Tags::read_into(buf).await?;
                tags_vec.push(tags);
            }
            data.insert(tag_type, tags_vec);
        }
        Ok(data)
    }
}

impl McBufWritable for HashMap<ResourceLocation, Vec<Tags>> {
    fn write_into(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buf.write_varint(self.len() as i32)?;
        for (k, v) in self {
            k.write_into(buf)?;
            v.write_into(buf)?;
        }
        Ok(())
    }
}
#[async_trait]
impl McBufReadable for Tags {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        let name = buf.read_resource_location().await?;
        let elements = buf.read_int_id_list().await?;
        Ok(Tags { name, elements })
    }
}

impl McBufWritable for Tags {
    fn write_into(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        self.name.write_into(buf)?;
        buf.write_int_id_list(&self.elements)?;
        Ok(())
    }
}
