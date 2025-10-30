use crate::{
    direction::{Axis, Direction},
    hit_result::BlockHitResult,
    math::EPSILON,
    position::{BlockPos, Vec3},
};

/// An axis-aligned bounding box.
///
/// In other words, a rectangular prism with a starting and ending point.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

pub struct ClipPointOpts<'a> {
    pub t: &'a mut f64,
    pub approach_dir: Option<Direction>,
    pub delta: Vec3,
    pub begin: f64,

    pub min_x: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_z: f64,

    pub result_dir: Direction,
    pub start: Vec3,
}

impl Aabb {
    pub fn contract(&self, amount: Vec3) -> Aabb {
        let mut min = self.min;
        let mut max = self.max;

        if amount.x < 0.0 {
            min.x -= amount.x;
        } else if amount.x > 0.0 {
            max.x -= amount.x;
        }

        if amount.y < 0.0 {
            min.y -= amount.y;
        } else if amount.y > 0.0 {
            max.y -= amount.y;
        }

        if amount.z < 0.0 {
            min.z -= amount.z;
        } else if amount.z > 0.0 {
            max.z -= amount.z;
        }

        Aabb { min, max }
    }

    pub fn expand_towards(&self, other: Vec3) -> Aabb {
        let mut min = self.min;
        let mut max = self.max;

        if other.x < 0.0 {
            min.x += other.x;
        } else if other.x > 0.0 {
            max.x += other.x;
        }

        if other.y < 0.0 {
            min.y += other.y;
        } else if other.y > 0.0 {
            max.y += other.y;
        }

        if other.z < 0.0 {
            min.z += other.z;
        } else if other.z > 0.0 {
            max.z += other.z;
        }

        Aabb { min, max }
    }

    pub fn inflate(&self, amount: Vec3) -> Aabb {
        let min = self.min - amount;
        let max = self.max + amount;

        Aabb { min, max }
    }
    pub fn inflate_all(&self, amount: f64) -> Aabb {
        self.inflate(Vec3::new(amount, amount, amount))
    }

    pub fn intersect(&self, other: &Aabb) -> Aabb {
        let min = self.min.max(other.min);
        let max = self.max.min(other.max);
        Aabb { min, max }
    }

    pub fn minmax(&self, other: &Aabb) -> Aabb {
        let min = self.min.min(other.min);
        let max = self.max.max(other.max);
        Aabb { min, max }
    }

    pub fn move_relative(&self, delta: Vec3) -> Aabb {
        Aabb {
            min: self.min + delta,
            max: self.max + delta,
        }
    }

    pub fn intersects_aabb(&self, other: &Aabb) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
            && self.min.z < other.max.z
            && self.max.z > other.min.z
    }
    pub fn intersects_vec3(&self, corner1: Vec3, corner2: Vec3) -> bool {
        let min = corner1.min(corner2);
        let max = corner1.max(corner2);
        self.intersects_aabb(&Aabb { min, max })
    }

    pub fn contains(&self, point: Vec3) -> bool {
        point.x >= self.min.x
            && point.x < self.max.x
            && point.y >= self.min.y
            && point.y < self.max.y
            && point.z >= self.min.z
            && point.z < self.max.z
    }

    pub fn size(&self) -> f64 {
        let x = self.get_size(Axis::X);
        let y = self.get_size(Axis::Y);
        let z = self.get_size(Axis::Z);
        (x + y + z) / 3.0
    }

    #[inline]
    pub fn get_size(&self, axis: Axis) -> f64 {
        axis.choose(
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    pub fn deflate(&self, amount: Vec3) -> Aabb {
        self.inflate(Vec3::new(-amount.x, -amount.y, -amount.z))
    }
    pub fn deflate_all(&self, amount: f64) -> Aabb {
        self.deflate(Vec3::new(amount, amount, amount))
    }

    pub fn clip(&self, min: Vec3, max: Vec3) -> Option<Vec3> {
        let mut t = 1.0;
        let delta = max - min;
        let _dir = Self::get_direction_aabb(self, min, &mut t, None, delta)?;
        Some(min + (delta * t))
    }

    pub fn clip_with_from_and_to(min: Vec3, max: Vec3, from: Vec3, to: Vec3) -> Option<Vec3> {
        let mut t = 1.0;
        let delta = to - from;
        let _dir = Self::get_direction(min, max, from, &mut t, None, delta)?;
        Some(from + (delta * t))
    }

    pub fn clip_iterable(
        boxes: &[Aabb],
        from: Vec3,
        to: Vec3,
        pos: BlockPos,
    ) -> Option<BlockHitResult> {
        let mut t = 1.0;
        let mut dir = None;
        let delta = to - from;

        for aabb in boxes {
            dir = Self::get_direction_aabb(
                &aabb.move_relative(pos.to_vec3_floored()),
                from,
                &mut t,
                dir,
                delta,
            );
        }
        let dir = dir?;
        Some(BlockHitResult {
            location: from + (delta * t),
            direction: dir,
            block_pos: pos,
            inside: false,
            miss: false,
            world_border: false,
        })
    }

    fn get_direction_aabb(
        &self,
        from: Vec3,
        t: &mut f64,
        dir: Option<Direction>,
        delta: Vec3,
    ) -> Option<Direction> {
        Aabb::get_direction(self.min, self.max, from, t, dir, delta)
    }

    fn get_direction(
        min: Vec3,
        max: Vec3,
        from: Vec3,
        t: &mut f64,
        mut dir: Option<Direction>,
        delta: Vec3,
    ) -> Option<Direction> {
        if delta.x > EPSILON {
            dir = Self::clip_point(ClipPointOpts {
                t,
                approach_dir: dir,
                delta,

                begin: min.x,
                min_x: min.y,
                max_x: max.y,
                min_z: min.z,
                max_z: max.z,

                result_dir: Direction::West,
                start: from,
            });
        } else if delta.x < -EPSILON {
            dir = Self::clip_point(ClipPointOpts {
                t,
                approach_dir: dir,
                delta,

                begin: max.x,
                min_x: min.y,
                max_x: max.y,
                min_z: min.z,
                max_z: max.z,

                result_dir: Direction::East,
                start: from,
            });
        }

        if delta.y > EPSILON {
            dir = Self::clip_point(ClipPointOpts {
                t,
                approach_dir: dir,
                delta: Vec3 {
                    x: delta.y,
                    y: delta.z,
                    z: delta.x,
                },
                begin: min.y,
                min_x: min.z,
                max_x: max.z,
                min_z: min.x,
                max_z: max.x,
                result_dir: Direction::Down,
                start: Vec3 {
                    x: from.y,
                    y: from.z,
                    z: from.x,
                },
            });
        } else if delta.y < -EPSILON {
            dir = Self::clip_point(ClipPointOpts {
                t,
                approach_dir: dir,
                delta: Vec3 {
                    x: delta.y,
                    y: delta.z,
                    z: delta.x,
                },
                begin: max.y,
                min_x: min.z,
                max_x: max.z,
                min_z: min.x,
                max_z: max.x,
                result_dir: Direction::Up,
                start: Vec3 {
                    x: from.y,
                    y: from.z,
                    z: from.x,
                },
            });
        }

        if delta.z > EPSILON {
            dir = Self::clip_point(ClipPointOpts {
                t,
                approach_dir: dir,
                delta: Vec3 {
                    x: delta.z,
                    y: delta.x,
                    z: delta.y,
                },
                begin: min.z,
                min_x: min.x,
                max_x: max.x,
                min_z: min.y,
                max_z: max.y,
                result_dir: Direction::North,
                start: Vec3 {
                    x: from.z,
                    y: from.x,
                    z: from.y,
                },
            });
        } else if delta.z < -EPSILON {
            dir = Self::clip_point(ClipPointOpts {
                t,
                approach_dir: dir,
                delta: Vec3 {
                    x: delta.z,
                    y: delta.x,
                    z: delta.y,
                },
                begin: max.z,
                min_x: min.x,
                max_x: max.x,
                min_z: min.y,
                max_z: max.y,
                result_dir: Direction::South,
                start: Vec3 {
                    x: from.z,
                    y: from.x,
                    z: from.y,
                },
            });
        }

        dir
    }

    fn clip_point(opts: ClipPointOpts) -> Option<Direction> {
        let d = (opts.begin - opts.start.x) / opts.delta.x;
        let e = opts.start.y + d * opts.delta.y;
        let f = opts.start.z + d * opts.delta.z;
        if 0.0 < d
            && d < *opts.t
            && opts.min_x - EPSILON < e
            && e < opts.max_x + EPSILON
            && opts.min_z - EPSILON < f
            && f < opts.max_z + EPSILON
        {
            *opts.t = d;
            Some(opts.result_dir)
        } else {
            opts.approach_dir
        }
    }

    pub fn has_nan(&self) -> bool {
        self.min.x.is_nan()
            || self.min.y.is_nan()
            || self.min.z.is_nan()
            || self.max.x.is_nan()
            || self.max.y.is_nan()
            || self.max.z.is_nan()
    }

    pub fn get_center(&self) -> Vec3 {
        Vec3::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
            (self.min.z + self.max.z) / 2.0,
        )
    }

    pub fn of_size(center: Vec3, dx: f64, dy: f64, dz: f64) -> Aabb {
        Aabb {
            min: Vec3::new(
                center.x - dx / 2.0,
                center.y - dy / 2.0,
                center.z - dz / 2.0,
            ),
            max: Vec3::new(
                center.x + dx / 2.0,
                center.y + dy / 2.0,
                center.z + dz / 2.0,
            ),
        }
    }

    pub fn max(&self, axis: &Axis) -> f64 {
        axis.choose(self.max.x, self.max.y, self.max.z)
    }
    pub fn min(&self, axis: &Axis) -> f64 {
        axis.choose(self.min.x, self.min.y, self.min.z)
    }

    pub fn collided_along_vector(&self, vector: Vec3, boxes: &[Aabb]) -> bool {
        let center = self.get_center();
        let new_center = center + vector;

        for aabb in boxes {
            let inflated = aabb.inflate(Vec3::new(
                self.get_size(Axis::X) * 0.5,
                self.get_size(Axis::Y) * 0.5,
                self.get_size(Axis::Z) * 0.5,
            ));
            if inflated.contains(new_center) || inflated.contains(center) {
                return true;
            }

            if inflated.clip(center, new_center).is_some() {
                return true;
            }
        }

        false
    }
}

impl BlockPos {
    pub fn between_closed_aabb(aabb: &Aabb) -> Vec<BlockPos> {
        BlockPos::between_closed(BlockPos::from(aabb.min), BlockPos::from(aabb.max))
    }

    pub fn between_closed(min: BlockPos, max: BlockPos) -> Vec<BlockPos> {
        assert!(min.x <= max.x);
        assert!(min.y <= max.y);
        assert!(min.z <= max.z);

        let length_x = max.x - min.x + 1;
        let length_y = max.y - min.y + 1;
        let length_z = max.z - min.z + 1;
        let volume = length_x * length_y * length_z;

        let mut result = Vec::with_capacity(volume as usize);
        for index in 0..volume {
            let index_x = index % length_x;
            let remaining_after_x = index / length_x;
            let index_y = remaining_after_x % length_y;
            let index_z = remaining_after_x / length_y;
            result.push(BlockPos::new(
                min.x + index_x,
                min.y + index_y,
                min.z + index_z,
            ));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_clip_iterable() {
        assert_ne!(
            Aabb::clip_iterable(
                &[Aabb {
                    min: Vec3::new(0., 0., 0.),
                    max: Vec3::new(1., 1., 1.),
                }],
                Vec3::new(-1., -1., -1.),
                Vec3::new(1., 1., 1.),
                BlockPos::new(0, 0, 0),
            ),
            None
        );
    }
}
