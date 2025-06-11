use crate::{
    direction::{Axis, Direction},
    hit_result::BlockHitResult,
    math::EPSILON,
    position::{BlockPos, Vec3},
};

/// A rectangular prism with a starting and ending point.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

pub struct ClipPointOpts<'a> {
    pub t: &'a mut f64,
    pub approach_dir: Option<Direction>,
    pub delta: &'a Vec3,
    pub begin: f64,
    pub min_x: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_z: f64,
    pub result_dir: Direction,
    pub start: &'a Vec3,
}

impl AABB {
    pub fn contract(&self, amount: Vec3) -> AABB {
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

        AABB { min, max }
    }

    pub fn expand_towards(&self, other: &Vec3) -> AABB {
        let mut min_x = self.min.x;
        let mut min_y = self.min.y;
        let mut min_z = self.min.z;

        let mut max_x = self.max.x;
        let mut max_y = self.max.y;
        let mut max_z = self.max.z;

        if other.x < 0.0 {
            min_x += other.x;
        } else if other.x > 0.0 {
            max_x += other.x;
        }

        if other.y < 0.0 {
            min_y += other.y;
        } else if other.y > 0.0 {
            max_y += other.y;
        }

        if other.z < 0.0 {
            min_z += other.z;
        } else if other.z > 0.0 {
            max_z += other.z;
        }

        AABB {
            min: Vec3::new(min_x, min_y, min_z),
            max: Vec3::new(max_x, max_y, max_z),
        }
    }

    pub fn inflate(&self, amount: Vec3) -> AABB {
        let min_x = self.min.x - amount.x;
        let min_y = self.min.y - amount.y;
        let min_z = self.min.z - amount.z;

        let max_x = self.max.x + amount.x;
        let max_y = self.max.y + amount.y;
        let max_z = self.max.z + amount.z;

        AABB {
            min: Vec3::new(min_x, min_y, min_z),
            max: Vec3::new(max_x, max_y, max_z),
        }
    }
    pub fn inflate_all(&self, amount: f64) -> AABB {
        self.inflate(Vec3::new(amount, amount, amount))
    }

    pub fn intersect(&self, other: &AABB) -> AABB {
        let min_x = self.min.x.max(other.min.x);
        let min_y = self.min.y.max(other.min.y);
        let min_z = self.min.z.max(other.min.z);

        let max_x = self.max.x.min(other.max.x);
        let max_y = self.max.y.min(other.max.y);
        let max_z = self.max.z.min(other.max.z);

        AABB {
            min: Vec3::new(min_x, min_y, min_z),
            max: Vec3::new(max_x, max_y, max_z),
        }
    }

    pub fn minmax(&self, other: &AABB) -> AABB {
        let min_x = self.min.x.min(other.min.x);
        let min_y = self.min.y.min(other.min.y);
        let min_z = self.min.z.min(other.min.z);

        let max_x = self.max.x.max(other.max.x);
        let max_y = self.max.y.max(other.max.y);
        let max_z = self.max.z.max(other.max.z);

        AABB {
            min: Vec3::new(min_x, min_y, min_z),
            max: Vec3::new(max_x, max_y, max_z),
        }
    }

    pub fn move_relative(&self, delta: Vec3) -> AABB {
        AABB {
            min: self.min + delta,
            max: self.max + delta,
        }
    }

    pub fn intersects_aabb(&self, other: &AABB) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
            && self.min.z < other.max.z
            && self.max.z > other.min.z
    }
    pub fn intersects_vec3(&self, corner1: &Vec3, corner2: &Vec3) -> bool {
        self.intersects_aabb(&AABB {
            min: Vec3::new(
                corner1.x.min(corner2.x),
                corner1.y.min(corner2.y),
                corner1.z.min(corner2.z),
            ),
            max: Vec3::new(
                corner1.x.max(corner2.x),
                corner1.y.max(corner2.y),
                corner1.z.max(corner2.z),
            ),
        })
    }

    pub fn contains(&self, point: &Vec3) -> bool {
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

    pub fn get_size(&self, axis: Axis) -> f64 {
        axis.choose(
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    pub fn deflate(&self, amount: Vec3) -> AABB {
        self.inflate(Vec3::new(-amount.x, -amount.y, -amount.z))
    }
    pub fn deflate_all(&self, amount: f64) -> AABB {
        self.deflate(Vec3::new(amount, amount, amount))
    }

    pub fn clip(&self, min: &Vec3, max: &Vec3) -> Option<Vec3> {
        let mut t = 1.0;
        let delta = max - min;
        let _dir = Self::get_direction_aabb(self, min, &mut t, None, &delta)?;
        Some(min + &(delta * t))
    }

    pub fn clip_with_from_and_to(min: &Vec3, max: &Vec3, from: &Vec3, to: &Vec3) -> Option<Vec3> {
        let mut t = 1.0;
        let delta = to - from;
        let _dir = Self::get_direction(min, max, from, &mut t, None, &delta)?;
        Some(from + &(delta * t))
    }

    pub fn clip_iterable(
        boxes: &Vec<AABB>,
        from: &Vec3,
        to: &Vec3,
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
                &delta,
            );
        }
        let dir = dir?;
        Some(BlockHitResult {
            location: from + &(delta * t),
            direction: dir,
            block_pos: pos,
            inside: false,
            miss: false,
            world_border: false,
        })
    }

    fn get_direction_aabb(
        &self,
        from: &Vec3,
        t: &mut f64,
        dir: Option<Direction>,
        delta: &Vec3,
    ) -> Option<Direction> {
        AABB::get_direction(&self.min, &self.max, from, t, dir, delta)
    }

    fn get_direction(
        min: &Vec3,
        max: &Vec3,
        from: &Vec3,
        t: &mut f64,
        mut dir: Option<Direction>,
        delta: &Vec3,
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
                delta: &Vec3 {
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
                start: &Vec3 {
                    x: from.y,
                    y: from.z,
                    z: from.x,
                },
            });
        } else if delta.y < -EPSILON {
            dir = Self::clip_point(ClipPointOpts {
                t,
                approach_dir: dir,
                delta: &Vec3 {
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
                start: &Vec3 {
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
                delta: &Vec3 {
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
                start: &Vec3 {
                    x: from.z,
                    y: from.x,
                    z: from.y,
                },
            });
        } else if delta.z < -EPSILON {
            dir = Self::clip_point(ClipPointOpts {
                t,
                approach_dir: dir,
                delta: &Vec3 {
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
                start: &Vec3 {
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

    pub fn of_size(center: Vec3, dx: f64, dy: f64, dz: f64) -> AABB {
        AABB {
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

    pub fn collided_along_vector(&self, vector: Vec3, boxes: &Vec<AABB>) -> bool {
        let center = self.get_center();
        let new_center = center + vector;

        for aabb in boxes {
            let inflated = aabb.inflate(Vec3::new(
                self.get_size(Axis::X) * 0.5,
                self.get_size(Axis::Y) * 0.5,
                self.get_size(Axis::Z) * 0.5,
            ));
            if inflated.contains(&new_center) || inflated.contains(&center) {
                return true;
            }

            if inflated.clip(&center, &new_center).is_some() {
                return true;
            }
        }

        false
    }
}

impl BlockPos {
    pub fn between_closed_aabb(aabb: &AABB) -> Vec<BlockPos> {
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
            AABB::clip_iterable(
                &vec![AABB {
                    min: Vec3::new(0., 0., 0.),
                    max: Vec3::new(1., 1., 1.),
                }],
                &Vec3::new(-1., -1., -1.),
                &Vec3::new(1., 1., 1.),
                BlockPos::new(0, 0, 0),
            ),
            None
        );
    }
}
