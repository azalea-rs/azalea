use azalea_buf::AzBuf;
use azalea_entity::LookDirection;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundMovePlayerRot {
    pub look_direction: LookDirection,
    pub on_ground: bool,
}
