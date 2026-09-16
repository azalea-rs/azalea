use azalea_buf::AzBuf;
use azalea_core::{delta::PositionDelta8, entity_id::MinecraftEntityId};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundMoveEntityPos {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub properties: Properties,
    pub delta: PositionDelta8,
}

#[derive(AzBuf, Copy, Clone, Debug, PartialEq, Default)]
pub struct Properties {
    #[var]
    data: u32,
}
impl Properties {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_ground(self) -> bool {
        self.data & 0b1 != 0
    }
    pub fn with_on_ground(mut self, value: bool) -> Self {
        if value {
            self.data |= 0b1;
        } else {
            self.data = !(!self.data | 0b1);
        }
        self
    }

    pub fn step_count(self) -> u32 {
        self.data >> 1
    }
    pub fn with_step_count(mut self, value: u32) -> Self {
        self.data = (value << 1) | (self.on_ground() as u32);
        self
    }
}
