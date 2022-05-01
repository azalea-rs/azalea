use crate::mc_buf::UnsizedByteArray;
use azalea_core::resource_location::ResourceLocation;
use packet_macros::GamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundAddEntityPacket {
    #[varint]
    pub id: i32,
    pub uuid: Uuid,
    // TODO: have an entity type struct
    #[varint]
    pub entity_type: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_rot: i8,
    pub y_rot: i8,
    pub data: i8,
    /// X acceleration
    pub xa: u16,
    /// Y acceleration
    pub ya: u16,
    /// Z acceleration
    pub za: u16,
}
