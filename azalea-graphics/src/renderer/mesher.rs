use azalea::blocks::BlockState;
use azalea::core::direction::Direction;
use azalea::core::position::{ChunkPos, ChunkSectionBlockPos, ChunkSectionPos};
use azalea::ecs::system::Local;
use azalea::world::{Chunk, Section};
use crossbeam::channel::{unbounded, Sender, Receiver};
use parking_lot::RwLock;

use std::sync::Arc;
use std::thread;

use crate::renderer::mesh::Vertex;

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
const EAST: usize  = 2;
const WEST: usize  = 3;
const NE: usize    = 4;
const NW: usize    = 5;
const SE: usize    = 6;
const SW: usize    = 7;


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
            (0, 1)  => self.neighbors[SOUTH].as_ref(),
            (-1, 0) => self.neighbors[WEST].as_ref(),
            (1, 0)  => self.neighbors[EAST].as_ref(),
            (-1, -1) => self.neighbors[NW].as_ref(),
            (1, -1)  => self.neighbors[NE].as_ref(),
            (-1, 1)  => self.neighbors[SW].as_ref(),
            (1, 1)   => self.neighbors[SE].as_ref(),
            _ => None,
        };

        if let Some(chunk_arc) = chunk_opt {
            let chunk = chunk_arc.read();
            let section_index = (base_y + cy_off) as usize;
            if let Some(section) = chunk.sections.get(section_index) {
                return section.get_block_state(ChunkSectionBlockPos { x: sx, y: sy, z: sz });
            }
            if (cx_off, cz_off, cy_off) == (0, 0, 0){
            println!("Fallback AIR at spos={:?}, lx={},ly={},lz={} (offsets {:?})",
                     base_y, lx, ly, lz, (cx_off, cy_off, cz_off));
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
    pub fn new() -> Self {
        let (work_tx, work_rx) = unbounded::<LocalSection>();
        let (result_tx, result_rx) = unbounded::<MeshData>();

        thread::spawn(move || {
            while let Ok(local_section) = work_rx.recv() {
                let mesh = simple_section_meshing(&local_section);
                result_tx.send(mesh).unwrap();
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


fn simple_section_meshing(section: &LocalSection) -> MeshData {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_offset = 0;

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let state = section.blocks[x + 1][y + 1][z + 1];
                if state == BlockState::AIR {
                    continue;
                }

                // world-space block coords
                let wx = section.spos.x * 16 + x as i32;
                let wy = section.spos.y * 16 + y as i32;
                let wz = section.spos.z * 16 + z as i32;

                for dir in [
                    Direction::Down,
                    Direction::Up,
                    Direction::North,
                    Direction::South,
                    Direction::West,
                    Direction::East,
                ] {
                    let nx = x as i32 + dir.x();
                    let ny = y as i32 + dir.y();
                    let nz = z as i32 + dir.z();

                    let neighbor_air = !is_solid(section, nx, ny, nz);

                    if neighbor_air {
                        let face = cube_face(
                            section,
                            x as i32, y as i32, z as i32,                          // local coords
                            [wx as f32, wy as f32, wz as f32], // world-space origin
                            dir,
                        );
                        let start = index_offset;
                        vertices.extend_from_slice(&face.vertices);
                        for idx in &face.indices {
                            indices.push(start + idx);
                        }
                        index_offset += face.vertices.len() as u32;
                    }
                }
            }
        }
    }

    MeshData {
        vertices,
        indices,
        section_pos: section.spos,
    }
}


struct FaceMesh {
    vertices: [Vertex; 4],
    indices: [u32; 6],
}



fn cube_face(section: &LocalSection, bx: i32, by: i32, bz: i32, pos: [f32; 3], dir: Direction) -> FaceMesh {
    let [x, y, z] = pos;

    let c = match dir {
        Direction::East  => [0.5, 0.0, 0.0],
        Direction::West  => [0.5, 0.0, 0.0],
        Direction::Up    => [0.5, 0.0, 0.0],
        Direction::Down  => [0.5, 0.0, 0.0],
        Direction::South => [0.0, 0.5, 0.0],
        Direction::North => [0.0, 0.0, 0.5],
    };


    let ao0 = compute_vertex_ao(section, bx, by, bz, dir, 0);
    let ao1 = compute_vertex_ao(section, bx, by, bz, dir, 1);
    let ao2 = compute_vertex_ao(section, bx, by, bz, dir, 2);
    let ao3 = compute_vertex_ao(section, bx, by, bz, dir, 3);


    match dir {
        Direction::East => {
            let verts = [
                Vertex { pos: [x+1.0, y,     z],     color: c, ao: ao0 },
                Vertex { pos: [x+1.0, y,     z+1.0], color: c, ao: ao1 },
                Vertex { pos: [x+1.0, y+1.0, z+1.0], color: c, ao: ao2 },
                Vertex { pos: [x+1.0, y+1.0, z],     color: c, ao: ao3 },
            ];

            FaceMesh { vertices: verts, indices: [0,2,1, 2,0,3] }
        }
        Direction::West => {
            let verts = [
                Vertex { pos: [x, y,     z],     color: c, ao: ao0 },
                Vertex { pos: [x, y,     z+1.0], color: c, ao: ao1 },
                Vertex { pos: [x, y+1.0, z+1.0], color: c, ao: ao2 },
                Vertex { pos: [x, y+1.0, z],     color: c, ao: ao3 },
            ];

            FaceMesh { vertices: verts, indices: [0,1,2, 2,3,0] }
        }
        Direction::Up => {
            let verts = [
                Vertex { pos: [x,     y+1.0, z],     color: c, ao: ao0  },
                Vertex { pos: [x+1.0, y+1.0, z],     color: c, ao: ao1  },
                Vertex { pos: [x+1.0, y+1.0, z+1.0], color: c, ao: ao2  },
                Vertex { pos: [x,     y+1.0, z+1.0], color: c, ao: ao3  },
            ];

            FaceMesh { vertices: verts, indices: [0,2,1, 2,0,3] }
        }
        Direction::Down => {
            let verts = [
                Vertex { pos: [x,     y, z],     color: c, ao: ao0 },
                Vertex { pos: [x+1.0, y, z],     color: c, ao: ao1 },
                Vertex { pos: [x+1.0, y, z+1.0], color: c, ao: ao2 },
                Vertex { pos: [x,     y, z+1.0], color: c, ao: ao3 },
            ];

            FaceMesh { vertices: verts, indices: [0,1,2, 2,3,0] }
        }
        Direction::South => {
            let verts = [
                Vertex { pos: [x,     y,     z+1.0], color: c, ao: ao0 },
                Vertex { pos: [x+1.0, y,     z+1.0], color: c, ao: ao1 },
                Vertex { pos: [x+1.0, y+1.0, z+1.0], color: c, ao: ao2 },
                Vertex { pos: [x,     y+1.0, z+1.0], color: c, ao: ao3 },
            ];

            FaceMesh { vertices: verts, indices: [0,1,2, 2,3,0] }
        }
        Direction::North => {
            let verts = [
                Vertex { pos: [x,     y,     z], color: c, ao: ao0 },
                Vertex { pos: [x+1.0, y,     z], color: c, ao: ao1 },
                Vertex { pos: [x+1.0, y+1.0, z], color: c, ao: ao2 },
                Vertex { pos: [x,     y+1.0, z], color: c, ao: ao3 },
            ];

            FaceMesh { vertices: verts, indices: [0,2,1, 2,0,3] }
        }
    }
}


fn compute_vertex_ao(section: &LocalSection, bx: i32, by: i32, bz: i32, dir: Direction, corner: usize) -> f32 {
    let [s1, s2, c] = ao_offsets(dir, corner);

    let side1  = is_solid(section, bx + s1.0, by + s1.1, bz + s1.2);
    let side2  = is_solid(section, bx + s2.0, by + s2.1, bz + s2.2);
    let corner = is_solid(section, bx + c.0,  by + c.1,  bz + c.2);

    vertex_ao(side1, side2, corner) as f32 / 3.0
}


fn ao_offsets(dir: Direction, corner: usize) -> [(i32, i32, i32); 3] {
    match dir {
        Direction::Up => match corner {
            0 => [(-1, 1, 0), (0, 1, -1), (-1, 1, -1)],
            1 => [(1, 1, 0), (0, 1, -1), (1, 1, -1)],
            2 => [(1, 1, 0), (0, 1, 1),  (1, 1, 1)],
            3 => [(-1, 1, 0), (0, 1, 1), (-1, 1, 1)],
            _ => unreachable!(),
        },
        Direction::Down => match corner {
            0 => [(-1, -1, 0), (0, -1, -1), (-1, -1, -1)],
            1 => [(1, -1, 0), (0, -1, -1), (1, -1, -1)],
            2 => [(1, -1, 0), (0, -1, 1),  (1, -1, 1)],
            3 => [(-1, -1, 0), (0, -1, 1), (-1, -1, 1)],
            _ => unreachable!(),
        },
        Direction::North => match corner {
            0 => [(-1, 0, -1), (0, -1, -1), (-1, -1, -1)],
            1 => [(1, 0, -1), (0, -1, -1), (1, -1, -1)],
            2 => [(1, 0, -1), (0, 1, -1),  (1, 1, -1)],
            3 => [(-1, 0, -1), (0, 1, -1), (-1, 1, -1)],
            _ => unreachable!(),
        },
        Direction::South => match corner {
            0 => [(-1, 0, 1), (0, -1, 1), (-1, -1, 1)],
            1 => [(1, 0, 1), (0, -1, 1), (1, -1, 1)],
            2 => [(1, 0, 1), (0, 1, 1),  (1, 1, 1)],
            3 => [(-1, 0, 1), (0, 1, 1), (-1, 1, 1)],
            _ => unreachable!(),
        },
        Direction::East => match corner {
            0 => [(1, -1, 0), (1, 0, -1), (1, -1, -1)],
            1 => [(1, -1, 0), (1, 0, 1),  (1, -1, 1)],
            2 => [(1, 1, 0),  (1, 0, 1),  (1, 1, 1)],
            3 => [(1, 1, 0),  (1, 0, -1), (1, 1, -1)],
            _ => unreachable!(),
        },
        Direction::West => match corner {
            0 => [(-1, -1, 0), (-1, 0, -1), (-1, -1, -1)],
            1 => [(-1, -1, 0), (-1, 0, 1),  (-1, -1, 1)],
            2 => [(-1, 1, 0),  (-1, 0, 1),  (-1, 1, 1)],
            3 => [(-1, 1, 0),  (-1, 0, -1), (-1, 1, -1)],
            _ => unreachable!(),
        },
    }
}


fn vertex_ao(side1: bool, side2: bool, corner: bool) -> u8 {
    if side1 && side2 {
        0
    } else {
        3 - (side1 as u8 + side2 as u8 + corner as u8)
    }
}


fn is_solid(local: &LocalSection, x: i32, y: i32, z: i32) -> bool {
    // offset by +1 into the padded array
    let bx = (x + 1) as usize;
    let by = (y + 1) as usize;
    let bz = (z + 1) as usize;

    local.blocks[bx][by][bz] != BlockState::AIR
}

