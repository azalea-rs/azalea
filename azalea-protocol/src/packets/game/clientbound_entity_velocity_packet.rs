use async_trait::async_trait;
use azalea_chat::component::Component;
use azalea_core::{resource_location::ResourceLocation, Slot};
use packet_macros::{GamePacket, McBufReadable, McBufWritable};
use tokio::io::AsyncRead;

use crate::mc_buf::{McBufReadable, McBufWritable, Readable, Writable};

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundEntityVelocityPacket {
    #[varint]
    pub entity_id: u32,
    pub x_vel: i16,
    pub y_vel: i16,
    pub z_vel: i16,
}
