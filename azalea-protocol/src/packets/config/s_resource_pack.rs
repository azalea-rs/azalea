use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundConfigPacket;
use uuid::Uuid;

#[derive(Clone, Debug, AzBuf, ServerboundConfigPacket)]
pub struct ServerboundResourcePack {
    pub id: Uuid,
    pub action: Action,
}

#[derive(AzBuf, Clone, Copy, Debug)]
pub enum Action {
    SuccessfullyLoaded = 0,
    Declined = 1,
    FailedDownload = 2,
    Accepted = 3,
    InvalidUrl = 4,
    FailedReload = 5,
    Discarded = 6,
}
