use azalea_core::{Vec3, AABB};

#[derive(Debug, Default)]
pub struct EntityDimensions {
    pub width: f32,
    pub height: f32,
}

impl EntityDimensions {
    pub fn make_bounding_box(&self, pos: &Vec3) -> AABB {
        let radius = (self.width / 2.0) as f64;
        let height = self.height as f64;
        AABB {
            min_x: pos.x - radius,
            min_y: pos.y,
            min_z: pos.z - radius,

            max_x: pos.x + radius,
            max_y: pos.y + height,
            max_z: pos.z + radius,
        }
    }
}
