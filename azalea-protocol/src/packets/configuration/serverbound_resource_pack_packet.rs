use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundConfigurationPacket;

#[derive(Clone, Debug, McBuf, ServerboundConfigurationPacket)]
pub struct ServerboundResourcePackPacket {
    pub action: Action,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Action {
    SuccessfullyLoaded = 0,
    Declined = 1,
    FailedDownload = 2,
    Accepted = 3,
}
