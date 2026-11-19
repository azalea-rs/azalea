use azalea_buf::AzBuf;
use azalea_core::{entity_id::MinecraftEntityId, position::Vec3};
use azalea_entity::LookDirection;
use azalea_protocol_macros::ClientboundGamePacket;
use tracing::warn;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundEntityPositionSync {
    #[var]
    pub id: MinecraftEntityId,
    pub position: PositionPath,
    pub look_direction: LookDirection,
    pub on_ground: bool,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub enum PositionPath {
    Linear(Vec3),
    // it's incorrect for this to be empty. vanilla throws an error in the constructor if it is,
    // azalea will probably log an error and default to 0,0,0
    Stepped(Vec<PositionStep>),
}
#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct PositionStep {
    pub pos: Vec3,
    #[var]
    pub tick_offset: i32,
}
impl PositionPath {
    pub fn end_position(&self) -> Vec3 {
        match self {
            PositionPath::Linear(v) => *v,
            PositionPath::Stepped(v) => match v.last() {
                Some(v) => v.pos,
                None => {
                    warn!("called end_position on a stepped PositionPath with no steps");
                    Vec3::default()
                }
            },
        }
    }
}
