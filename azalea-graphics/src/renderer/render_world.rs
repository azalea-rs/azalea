use std::{collections::HashMap, sync::Arc};

use ash::vk;
use azalea::core::position::ChunkSectionPos;

use crate::renderer::{
    assets::MeshAssets,
    mesh::Mesh,
    mesher::{LocalSection, Mesher},
    vulkan::context::VkContext,
};

pub struct RenderWorld {
    pub mesher: Mesher,
    pub meshes: HashMap<ChunkSectionPos, Mesh>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PushConstants {
    pub view_proj: glam::Mat4,
}

impl RenderWorld {
    pub fn new(assets: Arc<MeshAssets>) -> Self {
        Self {
            mesher: Mesher::new(assets),
            meshes: HashMap::new(),
        }
    }

    /// Submit a chunk for meshing (background thread will handle it)
    pub fn update_section(&self, section: LocalSection) {
        self.mesher.submit(section);
    }

    /// Poll mesher results and upload to GPU
    pub fn process_meshing_results(&mut self, ctx: &VkContext) {
        while let Some(mesh_data) = self.mesher.poll() {
            if mesh_data.vertices.is_empty() || mesh_data.indices.is_empty() {
                continue;
            }
            let mesh = Mesh::new(ctx, &mesh_data.vertices, &mesh_data.indices);
            self.meshes.insert(mesh_data.section_pos, mesh);
        }
    }

    /// Draw all chunk meshes
    pub fn draw(
        &self,
        device: &ash::Device,
        cmd: vk::CommandBuffer,
        pipeline: vk::Pipeline,
        descriptor_set: vk::DescriptorSet,
        pipeline_layout: vk::PipelineLayout,
        view_proj: glam::Mat4,
    ) {
        unsafe {
            device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, pipeline);
            let push = PushConstants { view_proj };

            device.cmd_push_constants(
                cmd,
                pipeline_layout,
                vk::ShaderStageFlags::VERTEX,
                0,
                std::slice::from_raw_parts(
                    &push as *const PushConstants as *const u8,
                    std::mem::size_of::<PushConstants>(),
                ),
            );

            device.cmd_bind_descriptor_sets(
                cmd,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline_layout,
                0,
                &[descriptor_set],
                &[],
            );
        }

        for (_, mesh) in &self.meshes {
            let vertex_buffers = [mesh.buffer];
            let offsets = [mesh.vertex_offset];
            unsafe {
                device.cmd_bind_vertex_buffers(cmd, 0, &vertex_buffers, &offsets);
                device.cmd_bind_index_buffer(
                    cmd,
                    mesh.buffer,
                    mesh.index_offset,
                    vk::IndexType::UINT32,
                );
                device.cmd_draw_indexed(cmd, mesh.index_count, 1, 0, 0, 0);
            }
        }
    }

    /// Clean up GPU resources
    pub fn destroy(&mut self, ctx: &VkContext) {
        for (_pos, mut mesh) in self.meshes.drain() {
            mesh.destroy(ctx);
        }
    }
}
