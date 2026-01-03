use azalea_auth::game_profile::GameProfile;
use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(AzBuf, ClientboundLoginPacket, Clone, Debug, PartialEq)]
pub struct ClientboundLoginFinished {
    pub game_profile: GameProfile,
}
