use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_entity::LookDirection;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundMovePlayerPosRot {
    pub pos: Vec3,
    pub look_direction: LookDirection,
    pub on_ground: bool,
}
