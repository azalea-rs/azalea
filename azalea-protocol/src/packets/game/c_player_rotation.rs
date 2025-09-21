use azalea_buf::AzBuf;
use azalea_entity::LookDirection;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundPlayerRotation {
    pub look_direction: LookDirection,
}
