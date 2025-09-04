use ash::{khr::swapchain as khr_swapchain, vk};

use super::{context::VkContext, frame_sync::FrameSync};

pub struct Swapchain {
    loader: khr_swapchain::Device,
    swapchain_khr: vk::SwapchainKHR,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
}

impl Swapchain {
    pub fn new(context: &VkContext, width: u32, height: u32) -> Self {
        let formats = unsafe {
            context
                .surface()
                .get_physical_device_surface_formats(
                    context.physical_device(),
                    context.surface_khr(),
                )
                .expect("Failed to query surface formats")
        };
        let surface_format = choose_surface_format(&formats);

        // compute image count once
        let capabilities = unsafe {
            context
                .surface()
                .get_physical_device_surface_capabilities(
                    context.physical_device(),
                    context.surface_khr(),
                )
                .unwrap()
        };
        let mut image_count = capabilities.min_image_count + 1;
        if capabilities.max_image_count > 0 && image_count > capabilities.max_image_count {
            image_count = capabilities.max_image_count;
        }

        Self::from_old(
            context,
            width,
            height,
            vk::SwapchainKHR::null(),
            surface_format.format,
            image_count,
        )
    }

    pub fn from_old(
        context: &VkContext,
        width: u32,
        height: u32,
        old_swapchain: vk::SwapchainKHR,
        format: vk::Format,
        image_count: u32,
    ) -> Self {
        let loader = khr_swapchain::Device::new(context.instance(), context.device());

        let capabilities = unsafe {
            context
                .surface()
                .get_physical_device_surface_capabilities(
                    context.physical_device(),
                    context.surface_khr(),
                )
                .unwrap()
        };

        let present_modes = unsafe {
            context
                .surface()
                .get_physical_device_surface_present_modes(
                    context.physical_device(),
                    context.surface_khr(),
                )
                .unwrap()
        };

        let present_mode = choose_present_mode(&present_modes);
        let extent = choose_extent(&capabilities, width, height);

        let indices = context.queue_families();
        let queue_family_indices = [indices.graphics_index, indices.present_index];
        let (sharing_mode, indices) = if indices.graphics_index != indices.present_index {
            (vk::SharingMode::CONCURRENT, &queue_family_indices[..])
        } else {
            (vk::SharingMode::EXCLUSIVE, &[][..])
        };

        let create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(context.surface_khr())
            .min_image_count(image_count)
            .image_format(format)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(sharing_mode)
            .queue_family_indices(indices)
            .pre_transform(capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(old_swapchain);

        let swapchain_khr = unsafe { loader.create_swapchain(&create_info, None).unwrap() };

        if old_swapchain != vk::SwapchainKHR::null() {
            unsafe { loader.destroy_swapchain(old_swapchain, None) };
        }

        let images = unsafe { loader.get_swapchain_images(swapchain_khr).unwrap() };

        let mut image_views = Vec::with_capacity(images.len());
        for &image in &images {
            let view_info = vk::ImageViewCreateInfo::default()
                .image(image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format)
                .subresource_range(
                    vk::ImageSubresourceRange::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .level_count(1)
                        .layer_count(1),
                );
            let view = unsafe {
                context
                    .device()
                    .create_image_view(&view_info, None)
                    .unwrap()
            };
            image_views.push(view);
        }

        Self {
            loader,
            swapchain_khr,
            format,
            extent,
            images,
            image_views,
        }
    }

    pub fn recreate(&mut self, context: &VkContext, width: u32, height: u32) {
        unsafe {
            for &view in &self.image_views {
                context.device().destroy_image_view(view, None);
            }
        }
        *self = Swapchain::from_old(
            context,
            width,
            height,
            self.swapchain_khr,
            self.format,
            self.images.len() as u32,
        );
    }

    pub fn acquire_next_image(&self, sync: &FrameSync, frame: usize) -> Result<u32, bool> {
        match unsafe {
            self.loader.acquire_next_image(
                self.swapchain_khr,
                u64::MAX,
                sync.image_available[frame],
                vk::Fence::null(),
            )
        } {
            Ok((index, _)) => Ok(index),
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Err(true),
            Err(_) => Err(false),
        }
    }

    pub fn present(
        &self,
        queue: vk::Queue,
        sync: &FrameSync,
        image_index: u32,
    ) -> Result<bool, vk::Result> {
        let wait_semaphores = [sync.render_finished[image_index as usize]];
        let swapchains = [self.swapchain_khr];
        let indices = [image_index];

        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(&wait_semaphores)
            .swapchains(&swapchains)
            .image_indices(&indices);

        let result = unsafe { self.loader.queue_present(queue, &present_info) };
        match result {
            Ok(_) => Ok(true),
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub fn destroy(&mut self, device: &ash::Device) {
        unsafe {
            for &view in &self.image_views {
                device.destroy_image_view(view, None);
            }
            self.loader.destroy_swapchain(self.swapchain_khr, None);
        }
    }
}

fn choose_surface_format(available: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
    available
        .iter()
        .cloned()
        .find(|fmt| {
            fmt.format == vk::Format::B8G8R8A8_SRGB
                && fmt.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        })
        .unwrap_or(available[0])
}

fn choose_present_mode(available: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
    if available.contains(&vk::PresentModeKHR::MAILBOX) {
        vk::PresentModeKHR::MAILBOX
    } else {
        vk::PresentModeKHR::FIFO
    }
}

fn choose_extent(
    capabilities: &vk::SurfaceCapabilitiesKHR,
    width: u32,
    height: u32,
) -> vk::Extent2D {
    if capabilities.current_extent.width != u32::MAX {
        capabilities.current_extent
    } else {
        vk::Extent2D {
            width: width.clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            ),
            height: height.clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            ),
        }
    }
}
