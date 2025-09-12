use std::{sync::Arc, thread};

use azalea::{blocks::BlockState, core::position::ChunkSectionPos, registry::Block};
use crossbeam::channel::{Receiver, Sender, unbounded};
use glam::IVec3;

use crate::renderer::{
    assets::Assets,
    chunk::LocalSection,
    world_renderer::{
        BlockVertex,
        mesher::{block::mesh_block, water::mesh_water},
    },
};

mod block;
mod block_colors;
mod helpers;
mod water;

pub struct MeshData {
    pub vertices: Vec<BlockVertex>,
    pub indices: Vec<u32>,
    pub section_pos: ChunkSectionPos,
}

pub struct Mesher {
    work_tx: Sender<LocalSection>,
    result_rx: Receiver<MeshResult>,
}

impl Mesher {
    pub fn new(assets: Arc<Assets>) -> Self {
        let (work_tx, work_rx) = unbounded::<LocalSection>();
        let (result_tx, result_rx) = unbounded::<MeshResult>();

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

    pub fn poll(&self) -> Option<MeshResult> {
        self.result_rx.try_recv().ok()
    }
}

pub struct MeshResult {
    pub blocks: MeshData,
    pub water: MeshData,
}

pub struct MeshBuilder<'a> {
    pub assets: &'a Assets,
    pub block_colors: &'a block_colors::BlockColors,
    pub section: &'a LocalSection,

    block_vertices: Vec<BlockVertex>,
    block_indices: Vec<u32>,
    water_vertices: Vec<BlockVertex>,
    water_indices: Vec<u32>,
}

impl<'a> MeshBuilder<'a> {
    pub fn push_block_quad(&mut self, verts: [BlockVertex; 4]) {
        let start = self.block_vertices.len() as u32;
        self.block_vertices.extend_from_slice(&verts);
        self.block_indices.extend_from_slice(&[
            start,
            start + 1,
            start + 2,
            start,
            start + 2,
            start + 3,
        ]);
    }

    pub fn push_water_quad(&mut self, verts: [BlockVertex; 4]) {
        let start = self.water_vertices.len() as u32;
        self.water_vertices.extend_from_slice(&verts);
        self.water_indices.extend_from_slice(&[
            start,
            start + 1,
            start + 2,
            start,
            start + 2,
            start + 3,
        ]);
    }

    pub fn finish(self) -> MeshResult {
        MeshResult {
            blocks: MeshData {
                section_pos: self.section.spos,
                vertices: self.block_vertices,
                indices: self.block_indices,
            },
            water: MeshData {
                section_pos: self.section.spos,
                vertices: self.water_vertices,
                indices: self.water_indices,
            },
        }
    }
}

pub fn mesh_section(section: &LocalSection, assets: &Assets) -> MeshResult {
    let block_colors = block_colors::BlockColors::create_default();

    let mut builder = MeshBuilder {
        assets,
        block_colors: &block_colors,
        section,
        block_vertices: Vec::with_capacity(1000),
        block_indices: Vec::with_capacity(1000),
        water_vertices: Vec::with_capacity(500),
        water_indices: Vec::with_capacity(500),
    };

    for y in 0..16 {
        for x in 0..16 {
            for z in 0..16 {
                let local = IVec3::new(x + 1, y + 1, z + 1);
                let block = section.blocks[local.x as usize][local.y as usize][local.z as usize].unwrap_or(BlockState::AIR);

                if !block.is_air() {
                    if Block::from(block) == Block::Water {
                        mesh_water(block, local, &mut builder);
                    }

                    mesh_block(block, local, &mut builder);
                }
            }
        }
    }

    builder.finish()
}
