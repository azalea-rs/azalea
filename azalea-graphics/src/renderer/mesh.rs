use ash::vk;
use vk_mem::{Allocation, MemoryUsage};

use crate::{renderer::state::{copy_buffer, create_buffer}, vulkan::context::VkContext};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Vertex>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: 0,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: std::mem::size_of::<[f32; 2]>() as u32,
            },
        ]
    }
}

pub struct Mesh {
    pub buffer: vk::Buffer,
    pub allocation: Allocation,
    pub vertex_offset: vk::DeviceSize,
    pub index_offset: vk::DeviceSize,
    pub index_count: u32,
}

impl Mesh {
    pub fn new(ctx: &VkContext, vertices: &[Vertex], indices: &[u32]) -> Self {
        let allocator = ctx.allocator();

        let vertex_size = (std::mem::size_of_val(vertices)) as vk::DeviceSize;
        let index_size  = (std::mem::size_of_val(indices)) as vk::DeviceSize;
        let total_size  = vertex_size + index_size;


        let (staging_buf, mut staging_alloc) = create_buffer(
            allocator,
            total_size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            MemoryUsage::AutoPreferHost,
            true,
        );

        unsafe {
            let ptr = allocator.map_memory(&mut staging_alloc).expect("map staging");

            std::ptr::copy_nonoverlapping(
                vertices.as_ptr() as *const u8,
                ptr,
                vertex_size as usize,
            );

            std::ptr::copy_nonoverlapping(
                indices.as_ptr() as *const u8,
                ptr.add(vertex_size as usize),
                index_size as usize,
            );

            allocator.unmap_memory(&mut staging_alloc);
        }

        let (buffer, allocation) = create_buffer(
            allocator,
            total_size,
            vk::BufferUsageFlags::VERTEX_BUFFER
                | vk::BufferUsageFlags::INDEX_BUFFER
                | vk::BufferUsageFlags::TRANSFER_DST,
            MemoryUsage::AutoPreferDevice,
            false,
        );

        copy_buffer(ctx, staging_buf, buffer, total_size);
        unsafe{allocator.destroy_buffer(staging_buf, &mut staging_alloc)};

        Self {
            buffer,
            allocation,
            vertex_offset: 0,
            index_offset: vertex_size,
            index_count: indices.len() as u32,
        }
    }

    pub fn destroy(&mut self, ctx: &VkContext){
        unsafe {
            ctx.allocator().destroy_buffer(self.buffer, &mut self.allocation);
        }
    }
}
