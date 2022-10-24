use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundLoginPacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ServerboundLoginPacket)]
pub struct ServerboundHelloPacket {
    pub name: String,
    // TODO: {'field': 'b.b', 'operation': 'write', 'type': 'uuid'}
    pub chat_session: Option<(u64, u32, Vec<u8>, u32, Vec<u8>)>,
    pub profile_id: Option<Uuid>,
}
