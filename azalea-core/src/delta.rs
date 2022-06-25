use crate::EntityPos;
pub use azalea_buf::McBuf;

pub trait PositionDeltaTrait {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

#[derive(Clone, Debug, McBuf, Default)]
pub struct PositionDelta {
    pub xa: f64,
    pub ya: f64,
    pub za: f64,
}

/// Only works for up to 8 blocks
#[derive(Clone, Debug, McBuf, Default)]
pub struct PositionDelta8 {
    pub xa: i16,
    pub ya: i16,
    pub za: i16,
}

impl PositionDeltaTrait for PositionDelta {
    fn x(&self) -> f64 {
        self.xa
    }
    fn y(&self) -> f64 {
        self.ya
    }
    fn z(&self) -> f64 {
        self.za
    }
}

impl PositionDelta8 {
    #[deprecated]
    pub fn float(&self) -> (f64, f64, f64) {
        (
            (self.xa as f64) / 4096.0,
            (self.ya as f64) / 4096.0,
            (self.za as f64) / 4096.0,
        )
    }
}

impl PositionDeltaTrait for PositionDelta8 {
    fn x(&self) -> f64 {
        (self.xa as f64) / 4096.0
    }
    fn y(&self) -> f64 {
        (self.ya as f64) / 4096.0
    }
    fn z(&self) -> f64 {
        (self.za as f64) / 4096.0
    }
}

impl EntityPos {
    pub fn with_delta(&self, delta: &dyn PositionDeltaTrait) -> EntityPos {
        EntityPos {
            x: self.x + delta.x(),
            y: self.y + delta.y(),
            z: self.z + delta.z(),
        }
    }
}
