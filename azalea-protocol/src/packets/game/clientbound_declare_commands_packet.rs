use super::GamePacket;
use crate::mc_buf::{McBufReadable, Readable};
use async_trait::async_trait;
use std::hash::Hash;
use tokio::io::AsyncRead;

#[derive(Hash, Clone, Debug)]
pub struct ClientboundDeclareCommandsPacket {
    pub entries: Vec<BrigadierNodeStub>,
    pub root_index: i32,
}

impl ClientboundDeclareCommandsPacket {
    pub fn get(self) -> GamePacket {
        GamePacket::ClientboundDeclareCommandsPacket(self)
    }

    pub fn write(&self, _buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        panic!("ClientboundDeclareCommandsPacket::write not implemented")
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut T,
    ) -> Result<GamePacket, String> {
        let node_count = buf.read_varint().await?;
        let mut nodes = Vec::with_capacity(node_count as usize);
        for _ in 0..node_count {
            let node = BrigadierNodeStub::read_into(buf).await?;
            nodes.push(node);
        }
        let root_index = buf.read_varint().await?;
        Ok(GamePacket::ClientboundDeclareCommandsPacket(
            ClientboundDeclareCommandsPacket {
                entries: nodes,
                root_index,
            },
        ))
    }
}

#[derive(Hash, Debug, Clone)]
pub struct BrigadierNodeStub {}

// azalea_brigadier::tree::CommandNode
#[async_trait]
impl McBufReadable for BrigadierNodeStub {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        let flags = u8::read_into(buf).await?;

        let node_type = flags & 0x03;
        let is_executable = flags & 0x04 != 0;
        let has_redirect = flags & 0x08 != 0;
        let has_suggestions_type = flags & 0x10 != 0;

        let children = buf.read_int_id_list().await?;
        let redirect_node = if has_redirect {
            buf.read_varint().await?
        } else {
            0
        };

        if node_type == 2 {
            let name = buf.read_utf().await?;

            let resource_location = if has_suggestions_type {
                Some(buf.read_resource_location().await?)
            } else {
                None
            };
            println!(
                "node_type=2, flags={}, name={}, resource_location={:?}",
                flags, name, resource_location
            );
            return Ok(BrigadierNodeStub {});
        }
        if node_type == 1 {
            let name = buf.read_utf().await?;
            println!("node_type=1, flags={}, name={}", flags, name);
            return Ok(BrigadierNodeStub {});
        }
        println!("node_type={}, flags={}", node_type, flags);
        Ok(BrigadierNodeStub {})
        // return Err("Unknown node type".to_string());
    }
}
