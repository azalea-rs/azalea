use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ServerboundGamePacket)]
pub struct ServerboundEditBook {
    #[var]
    pub slot: u32,
    pub pages: Vec<String>,
    pub title: Option<String>,
}
