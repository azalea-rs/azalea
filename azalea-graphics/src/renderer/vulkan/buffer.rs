
use ash::vk;
use vk_mem::{Alloc, Allocation, AllocationCreateFlags, AllocationCreateInfo, Allocator, MemoryUsage};

use crate::renderer::vulkan::{
    context::VkContext,
};

pub struct Buffer {
    pub buffer: vk::Buffer,
    pub allocation: Allocation,
    pub size: vk::DeviceSize,
}

impl Buffer {
    /// Create a new buffer
    pub fn new(
        ctx: &VkContext,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        memory: MemoryUsage,
        mapped: bool,
    ) -> Self {
        let (buffer, allocation) = create_buffer(ctx.allocator(), size, usage, memory, mapped);
        Self { buffer, allocation, size }
    }

    /// Destroy the buffer
    pub fn destroy(&mut self, ctx: &VkContext) {
        unsafe {
            ctx.allocator().destroy_buffer(self.buffer, &mut self.allocation);
        }
    }

    /// Map memory, copy data into buffer, unmap
    pub fn upload_data<T>(&mut self, ctx: &VkContext, offset: vk::DeviceSize, data: &[T]) {
        let allocator = ctx.allocator();
        let size = (std::mem::size_of::<T>() * data.len()) as vk::DeviceSize;
        assert!(offset + size <= self.size);


        unsafe {
            let ptr = allocator.map_memory(&mut self.allocation).expect("map memory");
            std::ptr::copy_nonoverlapping(
                data.as_ptr() as *const u8,
                ptr.add(offset as usize),
                size as usize,
            );
            allocator.unmap_memory(&mut self.allocation);
        }
    }

    /// Copy contents into another buffer (device-local upload)
    pub fn copy_to(&self, ctx: &VkContext, dst: &Buffer) {
        assert!(self.size <= dst.size);
        copy_buffer(ctx, self.buffer, dst.buffer, self.size);
    }
}

pub fn copy_buffer(ctx: &VkContext, src: vk::Buffer, dst: vk::Buffer, size: vk::DeviceSize) {
    let cmd = ctx.begin_one_time_commands();

    let copy_region = vk::BufferCopy::default()
        .src_offset(0)
        .dst_offset(0)
        .size(size);

    unsafe {
        ctx.device().cmd_copy_buffer(cmd, src, dst, &[copy_region]);
    }

    ctx.end_one_time_commands(cmd);
}

pub fn create_buffer(
    allocator: &Allocator,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    memory_usage: MemoryUsage,
    mapped: bool,
) -> (vk::Buffer, Allocation) {
    let buffer_info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    let mut alloc_info = AllocationCreateInfo {
        usage: memory_usage,
        ..Default::default()
    };

    if mapped {
        alloc_info.flags |= AllocationCreateFlags::MAPPED;
        alloc_info.flags |= AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE;
    }

    let (buffer, allocation) = unsafe {
        allocator
            .create_buffer(&buffer_info, &alloc_info)
            .expect("Failed to create buffer")
    };

    (buffer, allocation)
}


