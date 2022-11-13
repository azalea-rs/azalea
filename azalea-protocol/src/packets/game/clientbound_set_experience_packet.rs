use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetExperiencePacket {
    pub experience_progress: f32,
    #[var]
    pub experience_level: u32,
    #[var]
    pub total_experience: u32,
}
