use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_entity::LookDirection;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundAcceptTeleportation {
    #[var]
    pub id: u32,
    pub position: Vec3,
    pub direction: LookDirection,
}
