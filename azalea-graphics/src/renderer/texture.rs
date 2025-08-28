
use ash::vk;
use vk_mem::{Alloc, Allocation, MemoryUsage};

use crate::vulkan::{context::VkContext, utils::create_buffer};

pub struct Texture {
    pub image: vk::Image,
    pub allocation: Allocation,
    pub view: vk::ImageView,
    pub sampler: vk::Sampler,
}

impl Texture {
    pub fn new(ctx: &VkContext, image: image::RgbaImage) -> Self {
        let (width, height) = image.dimensions();
        let image_size = image.len() as vk::DeviceSize;

        let allocator = ctx.allocator();
        let (staging_buf, mut staging_alloc) = create_buffer(
            allocator,
            image_size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            MemoryUsage::AutoPreferHost,
            true,
        );

        unsafe {
            let ptr = allocator
                .map_memory(&mut staging_alloc)
                .expect("map staging");
            std::ptr::copy_nonoverlapping(image.as_ptr(), ptr, image.len());
            allocator.unmap_memory(&mut staging_alloc);
        }

        let extent = vk::Extent3D {
            width,
            height,
            depth: 1,
        };
        let image_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(vk::Format::R8G8B8A8_SRGB)
            .extent(extent)
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED);

        let (image, allocation) = unsafe {
            allocator
                .create_image(
                    &image_info,
                    &vk_mem::AllocationCreateInfo {
                        usage: MemoryUsage::AutoPreferDevice,
                        ..Default::default()
                    },
                )
                .expect("create image")
        };

        let cmd = ctx.begin_one_time_commands();

        let subresource = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };

        let barrier = vk::ImageMemoryBarrier::default()
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .image(image)
            .subresource_range(subresource);
        unsafe {
            ctx.device().cmd_pipeline_barrier(
                cmd,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[barrier],
            );
        }

        let copy_region = vk::BufferImageCopy::default()
            .buffer_offset(0)
            .image_subresource(vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
            })
            .image_extent(extent);

        unsafe {
            ctx.device().cmd_copy_buffer_to_image(
                cmd,
                staging_buf,
                image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[copy_region],
            );
        }

        let barrier2 = vk::ImageMemoryBarrier::default()
            .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .dst_access_mask(vk::AccessFlags::SHADER_READ)
            .image(image)
            .subresource_range(subresource);
        unsafe {
            ctx.device().cmd_pipeline_barrier(
                cmd,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::FRAGMENT_SHADER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[barrier2],
            );
        }

        ctx.end_one_time_commands(cmd);

        unsafe { allocator.destroy_buffer(staging_buf, &mut staging_alloc) };

        let view_info = vk::ImageViewCreateInfo::default()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(vk::Format::R8G8B8A8_SRGB)
            .subresource_range(subresource);
        let view = unsafe { ctx.device().create_image_view(&view_info, None).unwrap() };

        let sampler_info = vk::SamplerCreateInfo::default()
            .mag_filter(vk::Filter::NEAREST)
            .min_filter(vk::Filter::NEAREST)
            .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE);
        let sampler = unsafe { ctx.device().create_sampler(&sampler_info, None).unwrap() };

        Self {
            image,
            allocation,
            view,
            sampler,
        }
    }

    pub fn destroy(&mut self, ctx: &VkContext) {
        unsafe {
            ctx.device().destroy_sampler(self.sampler, None);
            ctx.device().destroy_image_view(self.view, None);
            ctx.allocator()
                .destroy_image(self.image, &mut self.allocation);
        }
    }
}
