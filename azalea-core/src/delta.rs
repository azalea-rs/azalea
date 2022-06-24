use crate::EntityPos;
pub use azalea_buf::McBuf;

/// Only works for up to 8 blocks
#[derive(Clone, Debug, McBuf)]
pub struct PositionDelta {
    xa: i16,
    ya: i16,
    za: i16,
}

impl PositionDelta {
    pub fn float(&self) -> (f64, f64, f64) {
        (
            (self.xa as f64) / 4096.0,
            (self.ya as f64) / 4096.0,
            (self.za as f64) / 4096.0,
        )
    }
}

impl EntityPos {
    pub fn with_delta(&self, delta: &PositionDelta) -> EntityPos {
        let (x, y, z) = delta.float();
        EntityPos {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }
}
