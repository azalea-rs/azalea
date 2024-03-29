use azalea_auth::game_profile::GameProfile;
use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(Clone, Debug, McBuf, ClientboundLoginPacket)]
pub struct ClientboundGameProfilePacket {
    pub game_profile: GameProfile,
}
