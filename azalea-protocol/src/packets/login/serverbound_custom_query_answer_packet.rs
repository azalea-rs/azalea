use azalea_protocol_macros::ServerboundLoginPacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ServerboundLoginPacket)]
pub struct ServerboundCustomQueryAnswerPacket {
#[var]
pub transaction_id: u32,
pub payload: Option<todo!()>,
}