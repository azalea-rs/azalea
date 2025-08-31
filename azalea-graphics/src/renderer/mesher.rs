use std::{sync::Arc, thread};

use azalea::{
    blocks::{BlockState, BlockTrait},
    core::{
        direction::Direction,
        position::{ChunkPos, ChunkSectionBiomePos, ChunkSectionBlockPos, ChunkSectionPos},
    },
    physics::collision::BlockWithShape,
    registry::Biome,
    world::Chunk,
};
use crossbeam::channel::{Receiver, Sender, unbounded};
use glam::{IVec3, Vec3};
use parking_lot::RwLock;

use crate::{
    assets::{
        MeshAssets,
        processed::{atlas::PlacedSprite, model::Cube},
    },
    plugin::BiomeCache,
    renderer::{block_colors::{BlockColors}, mesh::Vertex},
};

pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub section_pos: ChunkSectionPos,
}

pub struct LocalSection {
    pub blocks: Box<[[[BlockState; 18]; 18]; 18]>,
    pub biomes: Box<[[[Biome; 4]; 4]; 4]>,
    pub spos: ChunkSectionPos,
    pub biome_cache: BiomeCache,

}

const NORTH: usize = 0;
const SOUTH: usize = 1;
const EAST: usize = 2;
const WEST: usize = 3;
const NE: usize = 4;
const NW: usize = 5;
const SE: usize = 6;
const SW: usize = 7;

pub struct LocalChunk {
    pub center: Arc<RwLock<Chunk>>,
    pub neighbors: [Option<Arc<RwLock<Chunk>>>; 8],
}

pub struct BorrowedChunks<'a> {
    pub center: parking_lot::RwLockReadGuard<'a, Chunk>,
    pub neighbors: [Option<parking_lot::RwLockReadGuard<'a, Chunk>>; 8],
}

impl LocalChunk {
    pub fn borrow_chunks(&self) -> BorrowedChunks<'_> {
        let center = self.center.read();
        let neighbors = [
            self.neighbors[NORTH].as_ref().map(|c| c.read()),
            self.neighbors[SOUTH].as_ref().map(|c| c.read()),
            self.neighbors[EAST].as_ref().map(|c| c.read()),
            self.neighbors[WEST].as_ref().map(|c| c.read()),
            self.neighbors[NE].as_ref().map(|c| c.read()),
            self.neighbors[NW].as_ref().map(|c| c.read()),
            self.neighbors[SE].as_ref().map(|c| c.read()),
            self.neighbors[SW].as_ref().map(|c| c.read()),
        ];

        BorrowedChunks { center, neighbors }
    }

    pub fn local_sections(
        &self,
        chunk_pos: ChunkPos,
    ) -> Vec<LocalSection> {
        let borrowed = self.borrow_chunks();
        borrowed.local_sections(chunk_pos)
    }
}

impl<'a> BorrowedChunks<'a> {
    pub fn local_sections(
        &self,
        chunk_pos: ChunkPos,
    ) -> Vec<LocalSection> {
        let mut sections = Vec::new();

        for (i, section) in self.center.sections.iter().enumerate() {
            if section.block_count == 0 {
                continue;
            }

            let spos = ChunkSectionPos::new(chunk_pos.x, i as i32, chunk_pos.z);

            let local_section = self.build_local_section(spos);
            sections.push(local_section);
        }

        sections
    }

    /// Build a single local section with 18x18x18 extended block data
    pub fn build_local_section(
        &self,
        spos: ChunkSectionPos,
    ) -> LocalSection {
        let mut blocks = Box::new([[[BlockState::AIR; 18]; 18]; 18]);
        let mut biomes = Box::new([[[Default::default(); 4]; 4]; 4]);

        for lx in -1..17 {
            for ly in -1..17 {
                for lz in -1..17 {
                    let ix = (lx + 1) as usize;
                    let iy = (ly + 1) as usize;
                    let iz = (lz + 1) as usize;

                    blocks[ix][iy][iz] = self.get_block_local(spos.y, lx, ly, lz);
                }
            }
        }

        // Copy biome data from the center chunk section
        if let Some(section) = self.center.sections.get(spos.y as usize) {
            for x in 0..4 {
                for y in 0..4 {
                    for z in 0..4 {
                        let pos = ChunkSectionBiomePos { x, y, z };
                        biomes[x as usize][y as usize][z as usize] = section.get_biome(pos);
                    }
                }
            }
        }

        LocalSection {
            blocks,
            biomes,
            spos,
            biome_cache: BiomeCache { biomes: Vec::new() }, 
        }
    }

    /// Get a block at local coordinates (no additional locking needed)
    pub fn get_block_local(&self, base_y: i32, lx: i32, ly: i32, lz: i32) -> BlockState {
        let cx_off = lx.div_euclid(16);
        let sx = lx.rem_euclid(16) as u8;

        let cy_off = ly.div_euclid(16);
        let sy = ly.rem_euclid(16) as u8;

        let cz_off = lz.div_euclid(16);
        let sz = lz.rem_euclid(16) as u8;

        let chunk_ref = match (cx_off, cz_off) {
            (0, 0) => Some(&*self.center),
            (0, -1) => self.neighbors[NORTH].as_deref(),
            (0, 1) => self.neighbors[SOUTH].as_deref(),
            (-1, 0) => self.neighbors[WEST].as_deref(),
            (1, 0) => self.neighbors[EAST].as_deref(),
            (-1, -1) => self.neighbors[NW].as_deref(),
            (1, -1) => self.neighbors[NE].as_deref(),
            (-1, 1) => self.neighbors[SW].as_deref(),
            (1, 1) => self.neighbors[SE].as_deref(),
            _ => None,
        };

        if let Some(chunk) = chunk_ref {
            let section_index = (base_y + cy_off) as usize;
            if let Some(section) = chunk.sections.get(section_index) {
                return section.get_block_state(ChunkSectionBlockPos {
                    x: sx,
                    y: sy,
                    z: sz,
                });
            }
        }

        BlockState::AIR
    }
}

pub struct Mesher {
    work_tx: Sender<LocalSection>,
    result_rx: Receiver<MeshData>,
}

impl Mesher {
    pub fn new(assets: Arc<MeshAssets>) -> Self {
        let (work_tx, work_rx) = unbounded::<LocalSection>();
        let (result_tx, result_rx) = unbounded::<MeshData>();

        thread::spawn(move || {
            while let Ok(local_section) = work_rx.recv() {
                let mesh = mesh_section(&local_section, &assets);
                result_tx.send(mesh).unwrap();
            }
        });

        Self { work_tx, result_rx }
    }

    pub fn submit(&self, local_section: LocalSection) {
        self.work_tx.send(local_section).unwrap();
    }

    pub fn poll(&self) -> Option<MeshData> {
        self.result_rx.try_recv().ok()
    }
}

pub fn mesh_section(section: &LocalSection, assets: &MeshAssets) -> MeshData {
    let mut vertices = Vec::with_capacity(1000);
    let mut indices = Vec::with_capacity(1000);
    
    // Create block colors registry
    let block_colors = BlockColors::create_default();

    for y in 0..16 {
        for x in 0..16 {
            for z in 0..16 {
                let local = IVec3::new(x + 1, y + 1, z + 1);
                let block_state =
                    section.blocks[local.x as usize][local.y as usize][local.z as usize];

                if block_state.is_air() {
                    continue;
                }

                // Block position calculation is now done inside color provider functions

                for desc in assets.get_variant_descs(block_state) {
                    let model = assets
                        .get_block_model(&desc.model)
                        .expect("all block models must be loaded");

                    for element in &model.elements {
                        for face in FACES {
                            let rotated_dir =
                                rotate_direction(face.dir, desc.x_rotation, desc.y_rotation);

                            let model_face = match rotated_dir {
                                Direction::Down => &element.faces.down,
                                Direction::Up => &element.faces.up,
                                Direction::North => &element.faces.north,
                                Direction::South => &element.faces.south,
                                Direction::West => &element.faces.west,
                                Direction::East => &element.faces.east,
                            };

                            if let Some(model_face) = model_face {
                                let tint_index = model_face.tintindex;
                                let local_pos = IVec3::new(x as i32 + 1, y as i32 + 1, z as i32 + 1); // Convert to 1-18 range
                                let tint = block_colors.get_color(
                                    block_state,
                                    section,
                                    local_pos,
                                    tint_index,
                                    assets,
                                );
                                let len = vertices.len() as u32;

                                if let Some(cull_face) =
                                    model_face.cullface.as_deref().and_then(|s| match s {
                                        "down" => Some(Direction::Down),
                                        "up" => Some(Direction::Up),
                                        "north" => Some(Direction::North),
                                        "south" => Some(Direction::South),
                                        "west" => Some(Direction::West),
                                        "east" => Some(Direction::East),
                                        _ => None,
                                    })
                                {
                                    let cull_face = rotate_direction(
                                        cull_face,
                                        desc.x_rotation,
                                        desc.y_rotation,
                                    );

                                    let n = cull_face.normal();
                                    let neighbor = local + IVec3::new(n.x, n.y, n.z);
                                    let neighbor_state = section.blocks[neighbor.x as usize]
                                        [neighbor.y as usize]
                                        [neighbor.z as usize];
                                    let dyn_neighbor: Box<dyn BlockTrait> =
                                        Box::from(neighbor_state);

                                    let occlude = dyn_neighbor.behavior().can_occlude
                                        && !neighbor_state.is_air()
                                        && neighbor_state.is_collision_shape_full();

                                    if occlude {
                                        continue;
                                    }
                                }

                                let mut uvs = generate_uv(face.dir, model_face.uv);

                                if desc.uvlock {
                                    uvs = rotate_uvs(uvs, desc.y_rotation);
                                }

                                for (i, &offset) in face.offsets.iter().enumerate() {
                                    let sprite_name = model
                                        .resolve_texture(&model_face.texture)
                                        .unwrap_or("empty");

                                    let Some(spr) = assets.get_sprite_rect(sprite_name) else {
                                        continue;
                                    };
                                    let offset =
                                        rotate_offset(offset, desc.x_rotation, desc.y_rotation);

                                    let local_pos = offset_to_coord(offset, element) / 16.0;

                                    let world_pos = Vec3::new(
                                        (local.x - 1) as f32 + section.spos.x as f32 * 16.0,
                                        (local.y - 1) as f32 + section.spos.y as f32 * 16.0,
                                        (local.z - 1) as f32 + section.spos.z as f32 * 16.0,
                                    );
                                    let uv = remap_uv_to_atlas(
                                        uvs[i],
                                        spr,
                                        assets.block_atlas.width,
                                        assets.block_atlas.height,
                                    );

                                    vertices.push(Vertex {
                                        position: (local_pos + world_pos).into(),
                                        ao: if model.ambient_occlusion {
                                            compute_ao(local, offset, rotated_dir, section) as f32
                                        } else {
                                            3.0
                                        },
                                        uv,
                                        tint,
                                    });
                                }

                                indices.extend_from_slice(&[
                                    len,
                                    len + 1,
                                    len + 2,
                                    len,
                                    len + 2,
                                    len + 3,
                                ]);
                            }
                        }
                    }
                }
            }
        }
    }

    MeshData {
        section_pos: section.spos,
        vertices,
        indices,
    }
}

#[inline]
fn remap_uv_to_atlas(
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

fn rotate_direction(dir: Direction, x_rot: i32, y_rot: i32) -> Direction {
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

fn rotate_uvs(uvs: [glam::Vec2; 4], deg: i32) -> [glam::Vec2; 4] {
    match deg.rem_euclid(360) {
        0 => uvs,
        90 => [uvs[3], uvs[0], uvs[1], uvs[2]],
        180 => [uvs[2], uvs[3], uvs[0], uvs[1]],
        270 => [uvs[1], uvs[2], uvs[3], uvs[0]],
        _ => uvs,
    }
}

fn rotate_offset(mut p: glam::IVec3, x_rot: i32, y_rot: i32) -> glam::IVec3 {
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

fn offset_to_coord(offset: IVec3, element: &Cube) -> glam::Vec3 {
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

fn compute_ao(local: IVec3, offset: IVec3, dir: Direction, section: &LocalSection) -> u32 {
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

fn ao(side1: bool, side2: bool, corner: bool) -> u32 {
    if side1 && side2 {
        0
    } else {
        3 - ((side1 || side2) as u32 + corner as u32)
    }
}

struct Face {
    offsets: [IVec3; 4],
    dir: Direction,
}

const FACES: [Face; 6] = [
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

fn generate_uv(dir: Direction, uvs: Option<[f32; 4]>) -> [glam::Vec2; 4] {
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
