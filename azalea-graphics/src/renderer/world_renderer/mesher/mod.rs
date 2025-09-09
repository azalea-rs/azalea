use std::{sync::Arc, thread};

use azalea::core::position::ChunkSectionPos;
use crossbeam::channel::{Receiver, Sender, unbounded};
use glam::IVec3;

use crate::renderer::{
    assets::MeshAssets,
    chunk::LocalSection,
    world_renderer::{BlockVertex, mesher::block::mesh_block},
};

mod block;
mod block_colors;
mod helpers;

pub struct MeshData {
    pub vertices: Vec<BlockVertex>,
    pub indices: Vec<u32>,
    pub section_pos: ChunkSectionPos,
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

pub struct MeshBuilder<'a> {
    pub assets: &'a MeshAssets,
    pub block_colors: &'a block_colors::BlockColors,
    pub section: &'a LocalSection,
    pub vertices: &'a mut Vec<BlockVertex>,
    pub indices: &'a mut Vec<u32>,
}

pub fn mesh_section(section: &LocalSection, assets: &MeshAssets) -> MeshData {
    let mut vertices = Vec::with_capacity(1000);
    let mut indices = Vec::with_capacity(1000);
    let block_colors = block_colors::BlockColors::create_default();

    let mut builder = MeshBuilder {
        assets,
        block_colors: &block_colors,
        section,
        vertices: &mut vertices,
        indices: &mut indices,
    };

    for y in 0..16 {
        for x in 0..16 {
            for z in 0..16 {
                let local = IVec3::new(x + 1, y + 1, z + 1);
                let block = section.blocks[local.x as usize][local.y as usize][local.z as usize];
                if !block.is_air() {
                    mesh_block(block, local, &mut builder);
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
