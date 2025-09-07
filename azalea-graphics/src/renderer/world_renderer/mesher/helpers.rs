use azalea::{
    blocks::BlockTrait,
    core::direction::Direction,
    physics::collision::BlockWithShape,
};
use glam::IVec3;

use crate::renderer::{
    assets::processed::{atlas::PlacedSprite, model::Cube},
    world_renderer::mesher::LocalSection,
};

/// Face definition for meshing
pub struct Face {
    pub offsets: [IVec3; 4],
    pub dir: Direction,
}

/// Standard cube face definitions
pub const FACES: [Face; 6] = [
    Face {
        offsets: [
            glam::IVec3::new(0, 1, 0),
            glam::IVec3::new(0, 1, 1),
            glam::IVec3::new(1, 1, 1),
            glam::IVec3::new(1, 1, 0),
        ],
        dir: Direction::Up,
    },
    Face {
        offsets: [
            glam::IVec3::new(0, 0, 0),
            glam::IVec3::new(1, 0, 0),
            glam::IVec3::new(1, 0, 1),
            glam::IVec3::new(0, 0, 1),
        ],
        dir: Direction::Down,
    },
    Face {
        offsets: [
            glam::IVec3::new(0, 0, 1),
            glam::IVec3::new(1, 0, 1),
            glam::IVec3::new(1, 1, 1),
            glam::IVec3::new(0, 1, 1),
        ],
        dir: Direction::South,
    },
    Face {
        offsets: [
            glam::IVec3::new(0, 0, 0),
            glam::IVec3::new(0, 1, 0),
            glam::IVec3::new(1, 1, 0),
            glam::IVec3::new(1, 0, 0),
        ],
        dir: Direction::North,
    },
    Face {
        offsets: [
            glam::IVec3::new(1, 0, 0),
            glam::IVec3::new(1, 1, 0),
            glam::IVec3::new(1, 1, 1),
            glam::IVec3::new(1, 0, 1),
        ],
        dir: Direction::East,
    },
    Face {
        offsets: [
            glam::IVec3::new(0, 0, 0),
            glam::IVec3::new(0, 0, 1),
            glam::IVec3::new(0, 1, 1),
            glam::IVec3::new(0, 1, 0),
        ],
        dir: Direction::West,
    },
];

/// Remap UV coordinates to atlas coordinates
#[inline]
pub fn remap_uv_to_atlas(
    uv_px: glam::Vec2,
    spr: &PlacedSprite,
    atlas_w: u32,
    atlas_h: u32,
) -> [f32; 2] {
    const BIAS: f32 = 0.5;

    let aw = atlas_w as f32;
    let ah = atlas_h as f32;

    let u0 = (spr.x as f32 + BIAS) / aw;
    let v0 = (spr.y as f32 + BIAS) / ah;
    let u1 = (spr.x as f32 + spr.width as f32 - BIAS) / aw;
    let v1 = (spr.y as f32 + spr.height as f32 - BIAS) / ah;

    let tu = (uv_px.x / 16.0).clamp(0.0, 1.0);
    let tv = (uv_px.y / 16.0).clamp(0.0, 1.0);

    let u = u0 + (u1 - u0) * tu;
    let v = v0 + (v1 - v0) * tv;

    [u, v]
}

/// Rotate direction based on x and y rotations
pub fn rotate_direction(dir: Direction, x_rot: i32, y_rot: i32) -> Direction {
    let mut d = dir;
    match x_rot.rem_euclid(360) {
        90 => {
            d = match d {
                Direction::Up => Direction::South,
                Direction::South => Direction::Down,
                Direction::Down => Direction::North,
                Direction::North => Direction::Up,
                other => other,
            }
        }
        180 => {
            d = match d {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                other => other,
            }
        }
        270 => {
            d = match d {
                Direction::Up => Direction::North,
                Direction::North => Direction::Down,
                Direction::Down => Direction::South,
                Direction::South => Direction::Up,
                other => other,
            }
        }
        _ => {}
    }
    match y_rot.rem_euclid(360) {
        90 => {
            d = match d {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                other => other,
            }
        }
        180 => {
            d = match d {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
                Direction::West => Direction::East,
                other => other,
            }
        }
        270 => {
            d = match d {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                other => other,
            }
        }
        _ => {}
    }
    d
}

/// Rotate UV coordinates by degrees
pub fn rotate_uvs(uvs: [glam::Vec2; 4], deg: i32) -> [glam::Vec2; 4] {
    match deg.rem_euclid(360) {
        0 => uvs,
        90 => [uvs[3], uvs[0], uvs[1], uvs[2]],
        180 => [uvs[2], uvs[3], uvs[0], uvs[1]],
        270 => [uvs[1], uvs[2], uvs[3], uvs[0]],
        _ => uvs,
    }
}

/// Rotate offset coordinates by x and y rotations
pub fn rotate_offset(mut p: glam::IVec3, x_rot: i32, y_rot: i32) -> glam::IVec3 {
    match x_rot.rem_euclid(360) {
        90 => p = glam::IVec3::new(p.x, 1 - p.z, p.y),
        180 => p = glam::IVec3::new(p.x, 1 - p.y, 1 - p.z),
        270 => p = glam::IVec3::new(p.x, p.z, 1 - p.y),
        _ => {}
    }
    match y_rot.rem_euclid(360) {
        90 => p = glam::IVec3::new(1 - p.z, p.y, p.x),
        180 => p = glam::IVec3::new(1 - p.x, p.y, 1 - p.z),
        270 => p = glam::IVec3::new(p.z, p.y, 1 - p.x),
        _ => {}
    }
    p
}

/// Convert offset to world coordinates
pub fn offset_to_coord(offset: IVec3, element: &Cube) -> glam::Vec3 {
    glam::Vec3::new(
        if offset.x == 0 {
            element.from.x
        } else {
            element.to.x
        },
        if offset.y == 0 {
            element.from.y
        } else {
            element.to.y
        },
        if offset.z == 0 {
            element.from.z
        } else {
            element.to.z
        },
    )
}

/// Compute ambient occlusion for a vertex
pub fn compute_ao(local: IVec3, offset: IVec3, dir: Direction, section: &LocalSection) -> u32 {
    let get = |p: IVec3| {
        if p.x < 0 || p.y < 0 || p.z < 0 || p.x >= 18 || p.y >= 18 || p.z >= 18 {
            return false;
        }
        let state = section.blocks[p.x as usize][p.y as usize][p.z as usize];
        let dyn_state: Box<dyn BlockTrait> = Box::from(state);

        if state.is_air() {
            return false;
        }

        dyn_state.behavior().can_occlude && state.is_collision_shape_full()
    };

    let ox = offset.x * 2 - 1;
    let oy = offset.y * 2 - 1;
    let oz = offset.z * 2 - 1;

    match dir {
        Direction::East | Direction::West => {
            let side1 = get(local + IVec3::new(ox, 0, oz));
            let side2 = get(local + IVec3::new(ox, oy, 0));
            let corner = get(local + IVec3::new(ox, oy, oz));
            ao(side1, side2, corner)
        }
        Direction::Up | Direction::Down => {
            let side1 = get(local + IVec3::new(0, oy, oz));
            let side2 = get(local + IVec3::new(ox, oy, 0));
            let corner = get(local + IVec3::new(ox, oy, oz));
            ao(side1, side2, corner)
        }
        Direction::North | Direction::South => {
            let side1 = get(local + IVec3::new(0, oy, oz));
            let side2 = get(local + IVec3::new(ox, 0, oz));
            let corner = get(local + IVec3::new(ox, oy, oz));
            ao(side1, side2, corner)
        }
    }
}

/// Calculate ambient occlusion value
pub fn ao(side1: bool, side2: bool, corner: bool) -> u32 {
    if side1 && side2 {
        0
    } else {
        3 - ((side1 || side2) as u32 + corner as u32)
    }
}

/// Generate UV coordinates for a face

pub fn generate_uv(dir: Direction, uvs: Option<[f32; 4]>) -> [glam::Vec2; 4] {
    match uvs {
        Some(uvs) => match dir {
            Direction::Up => [
                glam::Vec2::new(uvs[0], uvs[1]),
                glam::Vec2::new(uvs[0], uvs[3]),
                glam::Vec2::new(uvs[2], uvs[3]),
                glam::Vec2::new(uvs[2], uvs[1]),
            ],
            Direction::Down => [
                glam::Vec2::new(uvs[0], uvs[3]),
                glam::Vec2::new(uvs[2], uvs[3]),
                glam::Vec2::new(uvs[2], uvs[1]),
                glam::Vec2::new(uvs[0], uvs[1]),
            ],
            Direction::North => [
                glam::Vec2::new(uvs[2], uvs[3]),
                glam::Vec2::new(uvs[2], uvs[1]),
                glam::Vec2::new(uvs[0], uvs[1]),
                glam::Vec2::new(uvs[0], uvs[3]),
            ],
            Direction::South => [
                glam::Vec2::new(uvs[0], uvs[3]),
                glam::Vec2::new(uvs[2], uvs[3]),
                glam::Vec2::new(uvs[2], uvs[1]),
                glam::Vec2::new(uvs[0], uvs[1]),
            ],
            Direction::East => [
                glam::Vec2::new(uvs[2], uvs[3]),
                glam::Vec2::new(uvs[2], uvs[1]),
                glam::Vec2::new(uvs[0], uvs[1]),
                glam::Vec2::new(uvs[0], uvs[3]),
            ],
            Direction::West => [
                glam::Vec2::new(uvs[0], uvs[3]),
                glam::Vec2::new(uvs[2], uvs[3]),
                glam::Vec2::new(uvs[2], uvs[1]),
                glam::Vec2::new(uvs[0], uvs[1]),
            ],
        },
        None => match dir {
            Direction::Up => [
                glam::Vec2::new(0.0, 0.0),
                glam::Vec2::new(0.0, 16.0),
                glam::Vec2::new(16.0, 16.0),
                glam::Vec2::new(16.0, 0.0),
            ],
            Direction::Down => [
                glam::Vec2::new(0.0, 16.0),
                glam::Vec2::new(16.0, 16.0),
                glam::Vec2::new(16.0, 0.0),
                glam::Vec2::new(0.0, 0.0),
            ],
            Direction::North => [
                glam::Vec2::new(16.0, 16.0),
                glam::Vec2::new(16.0, 0.0),
                glam::Vec2::new(0.0, 0.0),
                glam::Vec2::new(0.0, 16.0),
            ],
            Direction::South => [
                glam::Vec2::new(0.0, 16.0),
                glam::Vec2::new(16.0, 16.0),
                glam::Vec2::new(16.0, 0.0),
                glam::Vec2::new(0.0, 0.0),
            ],
            Direction::East => [
                glam::Vec2::new(16.0, 16.0),
                glam::Vec2::new(16.0, 0.0),
                glam::Vec2::new(0.0, 0.0),
                glam::Vec2::new(0.0, 16.0),
            ],
            Direction::West => [
                glam::Vec2::new(0.0, 16.0),
                glam::Vec2::new(16.0, 16.0),
                glam::Vec2::new(16.0, 0.0),
                glam::Vec2::new(0.0, 0.0),
            ],
        },
    }
}
