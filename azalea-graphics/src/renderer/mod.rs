use std::{sync::Arc, time::Duration};

use ash::vk::{self};
use raw_window_handle::{DisplayHandle, WindowHandle};
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
    chunk::LocalSection,
    ui::EguiVulkan,
    world_renderer::{WorldRenderer, WorldRendererOptions},
};

mod camera;
pub(crate) mod chunk;
mod mesh;
mod ui;
pub(crate) mod vulkan;
pub(crate) mod world_renderer;

mod assets;

pub struct Renderer {
    pub context: VkContext,
    pub swapchain: Swapchain,
    should_recreate: bool,
    width: u32,
    height: u32,

    wireframe_mode: bool,

    command_pool: vk::CommandPool,
    command_buffers: [vk::CommandBuffer; MAX_FRAMES_IN_FLIGHT],

    sync: FrameSync,

    world: WorldRenderer,

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

        let wireframe_enabled = context.features().fill_mode_non_solid;

        let world = WorldRenderer::new(
            Arc::new(assets),
            atlas_image,
            &context,
            &swapchain,
            WorldRendererOptions { wireframe_enabled },
        );

        let command_pool = create_command_pool(&context);
        let command_buffers = allocate_command_buffers(&context, command_pool);

        let sync = FrameSync::new(context.device(), swapchain.images.len());

        let camera = Camera::new(glam::vec3(0.0, 150.0, 2.0), -90.0, 0.0);
        let projection = Projection::new(size.width, size.height, 90.0, 0.1, 10000.0);
        let camera_controller = CameraController::new(4.0, 1.0);

        let egui = EguiVulkan::new(event_loop, &context, &swapchain, None)?;

        Ok(Self {
            context,
            swapchain,
            should_recreate: false,
            width: size.width,
            height: size.height,

            wireframe_mode: false,

            command_pool,
            command_buffers,

            sync,
            world,
            camera,
            projection,
            camera_controller,

            egui,
        })
    }

    /// Run the built-in debug UI.
    pub fn run_debug_ui(&mut self, window: &Window, frame_time_ms: f64) {
        let wireframe_available = self.context.features().fill_mode_non_solid;

        self.egui.run(window, |ctx| {
            egui::Window::new("Debug Info").show(ctx, |ui| {
                ui.label(format!("Frame time: {:.2}ms", frame_time_ms));
                ui.label("Azalea Graphics Renderer");

                ui.separator();

                ui.add_enabled(
                    wireframe_available,
                    egui::Checkbox::new(&mut self.wireframe_mode, "Wireframe mode (F3)"),
                );
            });
        });
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
        // For now, always allow toggling - the world renderer will handle whether
        // wireframe is available
        self.wireframe_mode = !self.wireframe_mode;
    }

    pub fn set_wireframe(&mut self, enabled: bool) {
        self.wireframe_mode = enabled;
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
        }

        self.world.render(
            device,
            cmd,
            image_index,
            self.swapchain.extent,
            self.projection.calc_proj() * self.camera.calc_view(),
            self.wireframe_mode,
            self.camera.position,
        );

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
            }
            self.swapchain
                .recreate(&self.context, self.width, self.height);

            // Let the world renderer handle its own swapchain recreation
            self.world
                .recreate_swapchain(&self.context, &self.swapchain);

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
