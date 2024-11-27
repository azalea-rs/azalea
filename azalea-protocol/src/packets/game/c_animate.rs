use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundAnimate {
    #[var]
    pub id: u32,
    pub action: AnimationAction,
}

// minecraft actually uses a u8 for this, but a varint still works and makes it
// so i don't have to add a special handler
#[derive(Clone, Debug, Copy, AzBuf)]
pub enum AnimationAction {
    SwingMainHand = 0,
    Hurt = 1,
    WakeUp = 2,
    SwingOffHand = 3,
    CriticalHit = 4,
    MagicCriticalHit = 5,
}
