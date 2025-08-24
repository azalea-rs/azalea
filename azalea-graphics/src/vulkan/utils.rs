use ash::vk;
use vk_mem::{Alloc, Allocation, AllocationCreateFlags, AllocationCreateInfo, Allocator, MemoryUsage};

use crate::vulkan::context::VkContext;


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

    let (buffer, allocation) = unsafe{allocator
        .create_buffer(&buffer_info, &alloc_info)
        .expect("Failed to create buffer")};

    (buffer, allocation)
}


pub fn copy_buffer(
    ctx: &VkContext,
    src: vk::Buffer,
    dst: vk::Buffer,
    size: vk::DeviceSize,
) {
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

