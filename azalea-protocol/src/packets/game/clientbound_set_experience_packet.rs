use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSetExperiencePacket {
    pub experience_progress: f32,
    #[var]
    pub experience_level: u32,
    #[var]
    pub total_experience: u32,
}
