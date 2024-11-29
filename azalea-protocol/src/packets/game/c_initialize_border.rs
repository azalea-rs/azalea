use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, ClientboundGamePacket, AzBuf)]
pub struct ClientboundInitializeBorder {
    pub new_center_x: f64,
    pub new_center_z: f64,
    pub old_size: f64,
    pub new_size: f64,
    #[var]
    pub lerp_time: u64,
    #[var]
    pub new_absolute_max_size: u32,
    #[var]
    pub warning_blocks: u32,
    #[var]
    pub warning_time: u32,
}
