use std::{sync::Arc, thread, time::Instant};

use azalea::{
    blocks::BlockState,
    core::{
        direction::Direction,
        position::{ChunkPos, ChunkSectionBlockPos, ChunkSectionPos},
    },
    physics::collision::BlockWithShape,
    world::Chunk,
};
use crossbeam::channel::{Receiver, Sender, unbounded};
use glam::{IVec3, Vec3};
use parking_lot::RwLock;

use crate::{
    assets::{MeshAssets, model::Cube},
    renderer::mesh::Vertex,
};

pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub section_pos: ChunkSectionPos,
}

pub struct LocalSection {
    pub blocks: Box<[[[BlockState; 18]; 18]; 18]>,
    pub spos: ChunkSectionPos,
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

impl LocalChunk {
    pub fn build_local_section(&self, spos: ChunkSectionPos) -> LocalSection {
        let mut blocks = Box::new([[[BlockState::AIR; 18]; 18]; 18]);

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

        LocalSection { blocks, spos }
    }

    fn get_block_local(&self, base_y: i32, lx: i32, ly: i32, lz: i32) -> BlockState {
        let cx_off = lx.div_euclid(16);
        let sx = lx.rem_euclid(16) as u8;

        let cy_off = ly.div_euclid(16);
        let sy = ly.rem_euclid(16) as u8;

        let cz_off = lz.div_euclid(16);
        let sz = lz.rem_euclid(16) as u8;

        let chunk_opt = match (cx_off, cz_off) {
            (0, 0) => Some(&self.center),
            (0, -1) => self.neighbors[NORTH].as_ref(),
            (0, 1) => self.neighbors[SOUTH].as_ref(),
            (-1, 0) => self.neighbors[WEST].as_ref(),
            (1, 0) => self.neighbors[EAST].as_ref(),
            (-1, -1) => self.neighbors[NW].as_ref(),
            (1, -1) => self.neighbors[NE].as_ref(),
            (-1, 1) => self.neighbors[SW].as_ref(),
            (1, 1) => self.neighbors[SE].as_ref(),
            _ => None,
        };

        if let Some(chunk_arc) = chunk_opt {
            let chunk = chunk_arc.read();
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
                let start = Instant::now();
                let mesh = mesh_section(&local_section, &assets);
                result_tx.send(mesh).unwrap();
                let time_took = start.elapsed();
                println!("chunk meshing took: {}ns", time_took.as_nanos());
            }
        });

        Self { work_tx, result_rx }
    }

    pub fn submit_chunk(&self, chunk_pos: ChunkPos, local_chunk: &LocalChunk) {
        let chunk = local_chunk.center.read();

        for (i, section) in chunk.sections.iter().enumerate() {
            if section.block_count == 0 {
                continue;
            }
            let spos = ChunkSectionPos::new(chunk_pos.x, i as i32, chunk_pos.z);
            let local_section = local_chunk.build_local_section(spos);
            self.submit(local_section);
        }
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

    for y in 0..16 {
        for x in 0..16 {
            for z in 0..16 {
                let local = IVec3::new(x + 1, y + 1, z + 1);
                let block_state =
                    section.blocks[local.x as usize][local.y as usize][local.z as usize];

                if block_state.is_air() {
                    continue;
                }

                if let Some(model) = assets.get_block_model_for(block_state) {
                    for element in &model.elements {
                        for face in FACES {
                            let model_face = match face.dir {
                                Direction::Down => &element.faces.down,
                                Direction::Up => &element.faces.up,
                                Direction::North => &element.faces.north,
                                Direction::South => &element.faces.south,
                                Direction::West => &element.faces.west,
                                Direction::East => &element.faces.east,
                            };

                            if let Some(model_face) = model_face {
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
                                    let n = cull_face.normal();
                                    let neighbor = local + IVec3::new(n.x, n.y, n.z);

                                    if section.blocks[neighbor.x as usize][neighbor.y as usize]
                                        [neighbor.z as usize]
                                        .is_collision_shape_full()
                                    {
                                        continue;
                                    }
                                }

                                let uvs = generate_uv(face.dir, model_face.uv);
                                for (i, offset) in face.offsets.iter().enumerate() {
                                    let tex_idx = model
                                        .resolve_texture(&model_face.texture)
                                        .and_then(|name| assets.get_texture_id(&name))
                                        .unwrap_or(0);

                                    let world_pos = Vec3::new(
                                        (local.x - 1) as f32 + section.spos.x as f32 * 16.0,
                                        (local.y - 1) as f32 + section.spos.y as f32 * 16.0,
                                        (local.z - 1) as f32 + section.spos.z as f32 * 16.0,
                                    );

                                    vertices.push(Vertex {
                                        position: (offset_to_coord(*offset, element) / 16.0
                                            + world_pos)
                                            .into(),
                                        ao: if model.ambient_occlusion {
                                            compute_ao(local, *offset, face.dir, section) as f32
                                        } else {
                                            3.0
                                        },
                                        tex_idx: tex_idx as u32,
                                        uv: uvs[i].into(),
                                    });
                                }

                                indices.extend_from_slice(&[
                                    len + 0,
                                    len + 1,
                                    len + 2,
                                    len + 0,
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

fn generate_uv(dir: Direction, uvs: Option<[f32; 4]>) -> [glam::Vec2; 4] {
    match uvs {
        Some(uvs) => match dir {
            Direction::Up => [
                glam::Vec2::new(uvs[0] / 16.0, uvs[1] / 16.0),
                glam::Vec2::new(uvs[2] / 16.0, uvs[1] / 16.0),
                glam::Vec2::new(uvs[2] / 16.0, uvs[3] / 16.0),
                glam::Vec2::new(uvs[0] / 16.0, uvs[3] / 16.0),
            ],
            Direction::Down => [
                glam::Vec2::new(uvs[0] / 16.0, uvs[1] / 16.0),
                glam::Vec2::new(uvs[2] / 16.0, uvs[1] / 16.0),
                glam::Vec2::new(uvs[2] / 16.0, uvs[3] / 16.0),
                glam::Vec2::new(uvs[0] / 16.0, uvs[3] / 16.0),
            ],
            Direction::North => [
                glam::Vec2::new(uvs[0] / 16.0, uvs[3] / 16.0),
                glam::Vec2::new(uvs[0] / 16.0, uvs[1] / 16.0),
                glam::Vec2::new(uvs[2] / 16.0, uvs[1] / 16.0),
                glam::Vec2::new(uvs[2] / 16.0, uvs[3] / 16.0),
            ],
            Direction::South => [
                glam::Vec2::new(uvs[0] / 16.0, uvs[3] / 16.0),
                glam::Vec2::new(uvs[2] / 16.0, uvs[3] / 16.0),
                glam::Vec2::new(uvs[2] / 16.0, uvs[1] / 16.0),
                glam::Vec2::new(uvs[0] / 16.0, uvs[1] / 16.0),
            ],
            Direction::East => [
                glam::Vec2::new(uvs[0] / 16.0, uvs[3] / 16.0),
                glam::Vec2::new(uvs[0] / 16.0, uvs[1] / 16.0),
                glam::Vec2::new(uvs[2] / 16.0, uvs[1] / 16.0),
                glam::Vec2::new(uvs[2] / 16.0, uvs[3] / 16.0),
            ],
            Direction::West => [
                glam::Vec2::new(uvs[0] / 16.0, uvs[3] / 16.0),
                glam::Vec2::new(uvs[2] / 16.0, uvs[3] / 16.0),
                glam::Vec2::new(uvs[2] / 16.0, uvs[1] / 16.0),
                glam::Vec2::new(uvs[0] / 16.0, uvs[1] / 16.0),
            ],
        },
        None => match dir {
            Direction::Up => [
                glam::Vec2::new(0.0, 0.0),
                glam::Vec2::new(1.0, 0.0),
                glam::Vec2::new(1.0, 1.0),
                glam::Vec2::new(0.0, 1.0),
            ],
            Direction::Down => [
                glam::Vec2::new(0.0, 0.0),
                glam::Vec2::new(1.0, 0.0),
                glam::Vec2::new(1.0, 1.0),
                glam::Vec2::new(0.0, 1.0),
            ],
            Direction::North => [
                glam::Vec2::new(0.0, 1.0),
                glam::Vec2::new(0.0, 0.0),
                glam::Vec2::new(1.0, 0.0),
                glam::Vec2::new(1.0, 1.0),
            ],
            Direction::South => [
                glam::Vec2::new(0.0, 1.0),
                glam::Vec2::new(1.0, 1.0),
                glam::Vec2::new(1.0, 0.0),
                glam::Vec2::new(0.0, 0.0),
            ],
            Direction::East => [
                glam::Vec2::new(0.0, 1.0),
                glam::Vec2::new(0.0, 0.0),
                glam::Vec2::new(1.0, 0.0),
                glam::Vec2::new(1.0, 1.0),
            ],
            Direction::West => [
                glam::Vec2::new(0.0, 1.0),
                glam::Vec2::new(1.0, 1.0),
                glam::Vec2::new(1.0, 0.0),
                glam::Vec2::new(0.0, 0.0),
            ],
        },
    }
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
        section.blocks[p.x as usize][p.y as usize][p.z as usize].is_collision_shape_full()
    };

    // convert {0,1} offset into {-1,+1}
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
