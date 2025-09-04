use std::{sync::Arc, time::Duration};

use ash::vk::{self};
use raw_window_handle::{DisplayHandle, WindowHandle};
use vk_mem::{Alloc, Allocation, AllocationCreateInfo, Allocator, MemoryUsage};
use vulkan::{
    context::VkContext,
    frame_sync::{FrameSync, MAX_FRAMES_IN_FLIGHT},
    swapchain::Swapchain,
};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, MouseScrollDelta, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    window::Window,
};

use self::{
    camera::{Camera, CameraController, Projection},
    mesher::LocalSection,
    passes::create_render_pass,
    pipelines::{create_pipeline, create_pipeline_layout, create_wireframe_pipeline},
    render_world::RenderWorld,
    texture::Texture,
    ui::EguiVulkan,
};

mod block_colors;
mod camera;
mod egui_painter;
mod mesh;
pub(crate) mod mesher;
mod passes;
mod pipelines;
mod render_world;
mod texture;
mod ui;
pub(crate) mod vulkan;

mod assets;

const TRIANGLE_VERT: &[u8] = include_bytes!(env!("TRIANGLE_VERT"));
const TRIANGLE_FRAG: &[u8] = include_bytes!(env!("TRIANGLE_FRAG"));

pub struct Renderer {
    pub context: VkContext,
    pub swapchain: Swapchain,
    should_recreate: bool,
    width: u32,
    height: u32,

    depth_image: vk::Image,
    depth_allocation: Allocation,
    depth_view: vk::ImageView,

    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    wireframe_pipeline: Option<vk::Pipeline>,
    wireframe_mode: bool,
    framebuffers: Vec<vk::Framebuffer>,

    descriptor_set_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    descriptor_set: vk::DescriptorSet,
    blocks_texture: Texture,

    command_pool: vk::CommandPool,
    command_buffers: [vk::CommandBuffer; MAX_FRAMES_IN_FLIGHT],

    sync: FrameSync,

    world: RenderWorld,

    camera: Camera,
    projection: Projection,
    camera_controller: CameraController,

    egui: EguiVulkan,
}

impl Renderer {
    pub fn new(
        window_handle: &WindowHandle,
        display_handle: &DisplayHandle,
        size: PhysicalSize<u32>,
        event_loop: &ActiveEventLoop,
    ) -> anyhow::Result<Self> {
        let context = VkContext::new(window_handle, display_handle);
        let swapchain = Swapchain::new(&context, size.width, size.height);

        let (assets, atlas_image) = assets::load_assets(&context, "assets/minecraft");
        let blocks_texture = Texture::new(&context, atlas_image);
        let descriptor_set_layout = create_descriptor_set_layout(context.device());
        let descriptor_pool = create_descriptor_pool(context.device());
        let descriptor_set =
            allocate_descriptor_set(context.device(), descriptor_pool, descriptor_set_layout);
        update_texture_descriptor(context.device(), descriptor_set, &blocks_texture);

        let render_pass = create_render_pass(&context, &swapchain);

        let pipeline_layout = create_pipeline_layout(context.device(), descriptor_set_layout);
        let pipeline = create_pipeline(
            &context,
            render_pass,
            pipeline_layout,
            TRIANGLE_VERT,
            TRIANGLE_FRAG,
        );
        let wireframe_pipeline = create_wireframe_pipeline(
            &context,
            render_pass,
            pipeline_layout,
            TRIANGLE_VERT,
            TRIANGLE_FRAG,
        );

        let (depth_image, depth_allocation, depth_view) =
            create_depth_resources(&context, context.allocator(), swapchain.extent);

        let framebuffers = create_framebuffers(&context, &swapchain, render_pass, depth_view);

        let command_pool = create_command_pool(&context);
        let command_buffers = allocate_command_buffers(&context, command_pool);

        let sync = FrameSync::new(context.device(), swapchain.images.len());

        let world = RenderWorld::new(Arc::new(assets));

        let camera = Camera::new(glam::vec3(0.0, 150.0, 2.0), -90.0, 0.0);
        let projection = Projection::new(size.width, size.height, 90.0, 1.0, 10000.0);
        let camera_controller = CameraController::new(4.0, 1.0);

        // Initialize egui
        let egui = EguiVulkan::new(event_loop, &context, &swapchain, None)?;

        Ok(Self {
            context,
            swapchain,
            should_recreate: false,
            width: size.width,
            height: size.height,

            depth_image,
            depth_allocation,
            depth_view,

            render_pass,
            pipeline_layout,
            pipeline,
            wireframe_pipeline,
            wireframe_mode: false,
            framebuffers,
            command_pool,
            command_buffers,

            descriptor_set_layout,
            descriptor_pool,
            descriptor_set,

            blocks_texture,

            sync,
            world,
            camera,
            projection,
            camera_controller,

            egui,
        })
    }

    pub fn update_section(&self, section: LocalSection) {
        self.world.update_section(section);
    }

    pub fn update(&mut self, dt: Duration) {
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.world.process_meshing_results(&self.context);
    }

    pub fn process_keyboard(&mut self, key: KeyCode, state: ElementState) -> bool {
        self.camera_controller.process_keyboard(key, state)
    }

    pub fn handle_mouse_scroll(&mut self, delta: &MouseScrollDelta) {
        self.camera_controller.handle_mouse_scroll(delta);
    }

    pub fn handle_mouse(&mut self, dx: f64, dy: f64) {
        self.camera_controller.handle_mouse(dx, dy);
    }

    pub fn toggle_wireframe(&mut self) {
        if self.wireframe_pipeline.is_some() {
            self.wireframe_mode = !self.wireframe_mode;
        }
    }

    pub fn set_wireframe(&mut self, enabled: bool) {
        if self.wireframe_pipeline.is_some() {
            self.wireframe_mode = enabled;
        }
    }

    pub fn is_wireframe(&self) -> bool {
        self.wireframe_mode
    }

    pub fn draw_frame(&mut self) {
        let device = self.context.device();
        let frame = self.sync.next_frame();

        self.sync.wait_for_fence(device, frame);

        let image_index = match self.swapchain.acquire_next_image(&self.sync, frame) {
            Ok(idx) => idx,
            Err(true) => {
                self.should_recreate = true;
                return;
            }
            Err(false) => panic!("Failed to acquire swapchain image"),
        };

        let cmd = self.command_buffers[frame];
        unsafe {
            device
                .reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty())
                .unwrap();

            let begin_info = vk::CommandBufferBeginInfo::default();
            device.begin_command_buffer(cmd, &begin_info).unwrap();

            let clear_color = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            };
            let clear_values = [
                clear_color,
                vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0,
                    },
                },
            ];
            let rp_info = vk::RenderPassBeginInfo::default()
                .render_pass(self.render_pass)
                .framebuffer(self.framebuffers[image_index as usize])
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: self.swapchain.extent,
                })
                .clear_values(&clear_values);

            device.cmd_begin_render_pass(cmd, &rp_info, vk::SubpassContents::INLINE);

            let viewport = vk::Viewport {
                x: 0.0,
                y: 0.0,
                width: self.swapchain.extent.width as f32,
                height: self.swapchain.extent.height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            };
            let scissor = vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.swapchain.extent,
            };
            device.cmd_set_viewport(cmd, 0, &[viewport]);
            device.cmd_set_scissor(cmd, 0, &[scissor]);

            let current_pipeline = if self.wireframe_mode {
                self.wireframe_pipeline.unwrap_or(self.pipeline)
            } else {
                self.pipeline
            };

            self.world.draw(
                device,
                cmd,
                current_pipeline,
                self.descriptor_set,
                self.pipeline_layout,
                self.projection.calc_proj() * self.camera.calc_view(),
            );

            device.cmd_end_render_pass(cmd);
        }

        // Render egui (it will handle its own render pass)
        if let Err(e) = self.render_egui(cmd, image_index, frame) {
            log::warn!("Failed to render egui: {}", e);
        }
        let device = self.context.device();

        unsafe {
            self.context.device().end_command_buffer(cmd).unwrap();
        }

        let wait_semaphores = [self.sync.image_available[frame]];
        let signal_semaphores = [self.sync.render_finished[image_index as usize]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let submit_info = vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(std::slice::from_ref(&cmd))
            .signal_semaphores(&signal_semaphores);

        unsafe {
            device
                .queue_submit(
                    self.context.graphics_queue(),
                    &[submit_info],
                    self.sync.in_flight[frame],
                )
                .unwrap();
        }

        match self
            .swapchain
            .present(self.context.present_queue(), &self.sync, image_index)
        {
            Ok(true) => {}
            Ok(false) => self.should_recreate = true,
            Err(e) => panic!("Present failed: {:?}", e),
        }
    }

    /// Mark swapchain as invalid, to be recreated later.
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.projection.resize(size.width, size.height);
        if size.width > 0 && size.height > 0 {
            self.should_recreate = true;
            self.width = size.width;
            self.height = size.height;
        }
    }

    /// Actually recreate swapchain if marked.
    pub fn maybe_recreate(&mut self) {
        if self.should_recreate {
            unsafe {
                self.context
                    .device()
                    .queue_wait_idle(self.context.present_queue())
                    .unwrap();
                self.context
                    .device()
                    .queue_wait_idle(self.context.graphics_queue())
                    .unwrap();
                for fb in &self.framebuffers {
                    self.context.device().destroy_framebuffer(*fb, None);
                }
            }
            self.framebuffers.clear();
            self.swapchain
                .recreate(&self.context, self.width, self.height);

            unsafe {
                self.context
                    .device()
                    .destroy_image_view(self.depth_view, None);
                self.context
                    .allocator()
                    .destroy_image(self.depth_image, &mut self.depth_allocation);
            }

            let (depth_image, depth_allocation, depth_view) = create_depth_resources(
                &self.context,
                self.context.allocator(),
                self.swapchain.extent,
            );

            self.depth_image = depth_image;
            self.depth_allocation = depth_allocation;
            self.depth_view = depth_view;

            self.framebuffers = create_framebuffers(
                &self.context,
                &self.swapchain,
                self.render_pass,
                self.depth_view,
            );

            // Resize egui
            self.egui.resize(&self.context, &self.swapchain);

            self.should_recreate = false;
        }
    }

    pub fn destroy(&mut self) {
        let device = self.context.device();

        unsafe {
            device.device_wait_idle().unwrap();

            self.world.destroy(&self.context);

            for fb in &self.framebuffers {
                device.destroy_framebuffer(*fb, None);
            }
            self.framebuffers.clear();

            device.destroy_image_view(self.depth_view, None);
            self.context
                .allocator()
                .destroy_image(self.depth_image, &mut self.depth_allocation);

            device.destroy_pipeline(self.pipeline, None);
            if let Some(wireframe_pipeline) = self.wireframe_pipeline {
                device.destroy_pipeline(wireframe_pipeline, None);
            }
            device.destroy_pipeline_layout(self.pipeline_layout, None);

            device.destroy_render_pass(self.render_pass, None);

            device.destroy_descriptor_pool(self.descriptor_pool, None);
            device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            self.blocks_texture.destroy(&self.context);

            device.destroy_command_pool(self.command_pool, None);
        }

        self.egui.destroy(&self.context);

        self.swapchain.destroy(device);
        self.sync.destroy(device);
    }

    /// Handle window events for egui.
    pub fn handle_egui_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
        let response = self.egui.on_window_event(window, event);
        response.consumed
    }

    /// Run the built-in debug UI.
    pub fn run_debug_ui(&mut self, window: &Window, frame_time_ms: f64) {
        let mut wireframe_changed = None;
        let current_wireframe = self.wireframe_mode;
        let wireframe_available = self.wireframe_pipeline.is_some();

        self.egui.run(window, |ctx| {
            egui::Window::new("Debug Info").show(ctx, |ui| {
                ui.label(format!("Frame time: {:.2}ms", frame_time_ms));
                ui.label("Azalea Graphics Renderer");

                ui.separator();

                if wireframe_available {
                    let mut wireframe = current_wireframe;
                    if ui.checkbox(&mut wireframe, "Wireframe mode (F3)").changed() {
                        wireframe_changed = Some(wireframe);
                    }
                } else {
                    ui.add_enabled(
                        false,
                        egui::Checkbox::new(&mut false, "Wireframe mode (not supported)"),
                    );
                    ui.label("⚠ fillModeNonSolid feature not available on this device");
                }
            });
        });

        // Apply wireframe change outside the closure
        if let Some(wireframe) = wireframe_changed {
            self.wireframe_mode = wireframe;
        }
    }

    /// Render egui to the given command buffer.
    pub fn render_egui(
        &mut self,
        cmd: vk::CommandBuffer,
        image_index: u32,
        frame_index: usize,
    ) -> anyhow::Result<()> {
        let dimensions = [self.swapchain.extent.width, self.swapchain.extent.height];
        self.egui
            .paint(&self.context, cmd, dimensions, image_index, frame_index)
    }
}

pub fn create_depth_resources(
    ctx: &VkContext,
    allocator: &Allocator,
    extent: vk::Extent2D,
) -> (vk::Image, Allocation, vk::ImageView) {
    let format = vk::Format::D32_SFLOAT;

    let image_info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::TYPE_2D)
        .format(format)
        .extent(vk::Extent3D {
            width: extent.width,
            height: extent.height,
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    let alloc_info = AllocationCreateInfo {
        usage: MemoryUsage::AutoPreferDevice,
        ..Default::default()
    };

    let (image, allocation) = unsafe {
        allocator
            .create_image(&image_info, &alloc_info)
            .expect("Failed to create depth image")
    };

    let view_info = vk::ImageViewCreateInfo::default()
        .image(image)
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(format)
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::DEPTH,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        });

    let depth_view = unsafe { ctx.device().create_image_view(&view_info, None).unwrap() };

    (image, allocation, depth_view)
}

pub fn create_framebuffers(
    ctx: &VkContext,
    swapchain: &Swapchain,
    render_pass: vk::RenderPass,
    depth_view: vk::ImageView,
) -> Vec<vk::Framebuffer> {
    let device = ctx.device();
    let mut framebuffers = Vec::with_capacity(swapchain.image_views.len());

    for &view in &swapchain.image_views {
        let attachments = [view, depth_view];

        let info = vk::FramebufferCreateInfo::default()
            .render_pass(render_pass)
            .attachments(&attachments)
            .width(swapchain.extent.width)
            .height(swapchain.extent.height)
            .layers(1);

        let framebuffer = unsafe { device.create_framebuffer(&info, None).unwrap() };
        framebuffers.push(framebuffer);
    }

    framebuffers
}

pub fn create_command_pool(ctx: &VkContext) -> vk::CommandPool {
    let device = ctx.device();
    let family_index = ctx.queue_families().graphics_index;

    let info = vk::CommandPoolCreateInfo::default()
        .queue_family_index(family_index)
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

    unsafe { device.create_command_pool(&info, None).unwrap() }
}

pub fn allocate_command_buffers(
    ctx: &VkContext,
    pool: vk::CommandPool,
) -> [vk::CommandBuffer; MAX_FRAMES_IN_FLIGHT] {
    let device = ctx.device();

    let alloc_info = vk::CommandBufferAllocateInfo::default()
        .command_pool(pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(MAX_FRAMES_IN_FLIGHT as u32);

    let mut buffers = [vk::CommandBuffer::null(); MAX_FRAMES_IN_FLIGHT];

    unsafe {
        (device.fp_v1_0().allocate_command_buffers)(
            device.handle(),
            &alloc_info,
            buffers.as_mut_ptr(),
        )
        .result()
        .unwrap()
    };

    buffers
}

pub fn create_descriptor_set_layout(device: &ash::Device) -> vk::DescriptorSetLayout {
    let sampler_binding = vk::DescriptorSetLayoutBinding::default()
        .binding(0)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT);

    let info = vk::DescriptorSetLayoutCreateInfo::default()
        .bindings(std::slice::from_ref(&sampler_binding));

    unsafe { device.create_descriptor_set_layout(&info, None).unwrap() }
}

pub fn create_descriptor_pool(device: &ash::Device) -> vk::DescriptorPool {
    let pool_size = vk::DescriptorPoolSize::default()
        .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1);

    let info = vk::DescriptorPoolCreateInfo::default()
        .pool_sizes(std::slice::from_ref(&pool_size))
        .max_sets(1);

    unsafe { device.create_descriptor_pool(&info, None).unwrap() }
}

pub fn allocate_descriptor_set(
    device: &ash::Device,
    pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
) -> vk::DescriptorSet {
    let alloc_info = vk::DescriptorSetAllocateInfo::default()
        .descriptor_pool(pool)
        .set_layouts(std::slice::from_ref(&layout));

    unsafe { device.allocate_descriptor_sets(&alloc_info).unwrap()[0] }
}

pub fn update_texture_descriptor(
    device: &ash::Device,
    descriptor_set: vk::DescriptorSet,
    tex: &Texture,
) {
    let image_info = vk::DescriptorImageInfo {
        sampler: tex.sampler,
        image_view: tex.view,
        image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    };

    let write = vk::WriteDescriptorSet::default()
        .dst_set(descriptor_set)
        .dst_binding(0)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .image_info(std::slice::from_ref(&image_info));

    unsafe {
        device.update_descriptor_sets(std::slice::from_ref(&write), &[]);
    }
}
