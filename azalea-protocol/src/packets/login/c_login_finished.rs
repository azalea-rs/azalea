use azalea_auth::game_profile::GameProfile;
use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(Clone, Debug, McBuf, ClientboundLoginPacket)]
pub struct ClientboundLoginFinished {
    pub game_profile: GameProfile,
}
