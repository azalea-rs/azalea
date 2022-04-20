// i don't know the actual name of this packet, i couldn't find it in the source code

use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundPlayerAbilitiesPacket {
    pub flags: u8,
    pub flying_speed: f32,
    /// Used for the fov
    pub walking_speed: f32,
}
