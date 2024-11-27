use azalea_auth::game_profile::GameProfile;
use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(Clone, Debug, AzBuf, ClientboundLoginPacket)]
pub struct ClientboundLoginFinished {
    pub game_profile: GameProfile,
}
