use azalea_core::position::Vec3;

#[derive(Debug, Clone, Default)]
pub struct VecDeltaCodec {
    base: Vec3,
}

impl VecDeltaCodec {
    pub fn new(base: Vec3) -> Self {
        Self { base }
    }

    pub fn decode(&self, x: i64, y: i64, z: i64) -> Vec3 {
        if x == 0 && y == 0 && z == 0 {
            return self.base;
        }

        let new_x = if x == 0 {
            self.base.x
        } else {
            decode(encode(self.base.x) + x)
        };
        let new_y = if y == 0 {
            self.base.y
        } else {
            decode(encode(self.base.y) + y)
        };
        let new_z = if z == 0 {
            self.base.z
        } else {
            decode(encode(self.base.z) + z)
        };

        Vec3::new(new_x, new_y, new_z)
    }

    pub fn encode_x(&self, pos: Vec3) -> i64 {
        encode(pos.x) - encode(self.base.x)
    }
    pub fn encode_y(&self, pos: Vec3) -> i64 {
        encode(pos.y) - encode(self.base.y)
    }
    pub fn encode_z(&self, pos: Vec3) -> i64 {
        encode(pos.z) - encode(self.base.z)
    }

    pub fn set_base(&mut self, pos: Vec3) {
        self.base = pos;
    }
    pub fn base(&self) -> Vec3 {
        self.base
    }
}

fn encode(value: f64) -> i64 {
    (value * 4096.).round() as i64
}
fn decode(value: i64) -> f64 {
    (value as f64) / 4096.
}
