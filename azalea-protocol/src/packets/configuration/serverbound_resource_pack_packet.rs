use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundConfigurationPacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ServerboundConfigurationPacket)]
pub struct ServerboundResourcePackPacket {
    pub id: Uuid,
    pub action: Action,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Action {
    SuccessfullyLoaded = 0,
    Declined = 1,
    FailedDownload = 2,
    Accepted = 3,
    InvalidUrl = 4,
    FailedReload = 5,
    Discarded = 6,
}
