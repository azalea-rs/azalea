use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundEditBookPacket {
    #[var]
    pub slot: u32,
    pub pages: Vec<String>,
    pub title: Option<String>,
}
