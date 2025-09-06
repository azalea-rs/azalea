use std::marker::PhantomData;
use std::mem::{align_of, size_of};
use ash::vk;
use vk_mem::MemoryUsage;

use crate::renderer::vulkan::{context::VkContext, buffer::Buffer};

pub struct Mesh<V> {
    pub buffer: Buffer,
    pub vertex_offset: vk::DeviceSize,
    pub index_offset: vk::DeviceSize,
    pub index_count: u32,
    _marker: PhantomData<V>,
}

impl<V> Mesh<V> {
    pub fn new_host(ctx: &VkContext, vertices: &[V], indices: &[u32]) -> Self {
        let vertex_size = (size_of::<V>() * vertices.len()) as vk::DeviceSize;
        let index_size = (size_of::<u32>() * indices.len()) as vk::DeviceSize;

        let align = align_of::<u32>() as vk::DeviceSize;
        let index_offset = (vertex_size + align - 1) & !(align - 1);
        let total_size = index_offset + index_size;

        let mut buffer = Buffer::new(
            ctx,
            total_size,
            vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::INDEX_BUFFER,
            MemoryUsage::AutoPreferHost,
            true,
        );

        buffer.upload_data(ctx, 0, vertices);
        buffer.upload_data(ctx, index_offset, indices);

        Self {
            buffer,
            vertex_offset: 0,
            index_offset,
            index_count: indices.len() as u32,
            _marker: PhantomData,
        }
    }

    pub fn new_staging(ctx: &VkContext, vertices: &[V], indices: &[u32]) -> Self {
        let vertex_size = (size_of::<V>() * vertices.len()) as vk::DeviceSize;
        let index_size = (size_of::<u32>() * indices.len()) as vk::DeviceSize;

        let align = align_of::<u32>() as vk::DeviceSize;
        let index_offset = (vertex_size + align - 1) & !(align - 1);
        let total_size = index_offset + index_size;

        let mut buffer = Buffer::new(
            ctx,
            total_size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            MemoryUsage::AutoPreferHost,
            true,
        );

        buffer.upload_data(ctx, 0, vertices);
        buffer.upload_data(ctx, index_offset, indices);

        Self {
            buffer,
            vertex_offset: 0,
            index_offset,
            index_count: indices.len() as u32,
            _marker: PhantomData,
        }
    }

    /// Upload staging mesh into GPU-local memory
    pub fn upload(&self, ctx: &VkContext) -> Mesh<V> {
        let gpu_buffer = Buffer::new(
            ctx,
            self.buffer.size,
            vk::BufferUsageFlags::VERTEX_BUFFER
                | vk::BufferUsageFlags::INDEX_BUFFER
                | vk::BufferUsageFlags::TRANSFER_DST,
            MemoryUsage::AutoPreferDevice,
            false,
        );

        self.buffer.copy_to(ctx, &gpu_buffer);

        Mesh {
            buffer: gpu_buffer,
            vertex_offset: self.vertex_offset,
            index_offset: self.index_offset,
            index_count: self.index_count,
            _marker: PhantomData,
        }
    }

    pub fn destroy(&mut self, ctx: &VkContext) {
        self.buffer.destroy(ctx);
    }
}
