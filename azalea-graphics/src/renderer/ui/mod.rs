use egui::{ViewportId, ViewportOutput};
use egui_winit::winit;
pub use egui_winit::{self, EventResponse};

use crate::renderer::{
    ui::painter::Painter,
    vulkan::{context::VkContext, swapchain::Swapchain},
};

mod painter;
mod pipelines;
mod passes;

/// Use [`egui`] from a Vulkan app based on [`winit`].
pub struct EguiVulkan {
    egui_ctx: egui::Context,
    egui_winit: egui_winit::State,
    painter: Painter,

    viewport_info: egui::ViewportInfo,

    // output from the last update:
    shapes: Vec<egui::epaint::ClippedShape>,
    pixels_per_point: f32,
    textures_delta: egui::TexturesDelta,
}

impl EguiVulkan {
    pub fn new(
        event_loop: &winit::event_loop::ActiveEventLoop,
        ctx: &VkContext,
        swapchain: &Swapchain,
        native_pixels_per_point: Option<f32>,
    ) -> anyhow::Result<Self> {
        let painter = Painter::new(ctx, swapchain)?;

        let egui_ctx = egui::Context::default();

        let egui_winit = egui_winit::State::new(
            egui_ctx.clone(),
            ViewportId::ROOT,
            event_loop,
            native_pixels_per_point,
            event_loop.system_theme(),
            None,
        );

        Ok(Self {
            egui_ctx,
            egui_winit,
            painter,
            viewport_info: Default::default(),
            shapes: Default::default(),
            pixels_per_point: native_pixels_per_point.unwrap_or(1.0),
            textures_delta: Default::default(),
        })
    }

    pub fn on_window_event(
        &mut self,
        window: &winit::window::Window,
        event: &winit::event::WindowEvent,
    ) -> EventResponse {
        self.egui_winit.on_window_event(window, event)
    }

    /// Call [`Self::paint`] later to paint.
    pub fn run(&mut self, window: &winit::window::Window, run_ui: impl FnMut(&egui::Context)) {
        let raw_input = self.egui_winit.take_egui_input(window);

        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output,
        } = self.egui_ctx.run(raw_input, run_ui);

        if viewport_output.len() > 1 {
            log::warn!("Multiple viewports not yet supported by EguiVulkan");
        }
        for (_, ViewportOutput { commands, .. }) in viewport_output {
            let mut actions_requested = Default::default();
            egui_winit::process_viewport_commands(
                &self.egui_ctx,
                &mut self.viewport_info,
                commands,
                window,
                &mut actions_requested,
            );
            for action in actions_requested {
                log::warn!("{:?} not yet supported by EguiVulkan", action);
            }
        }

        self.egui_winit
            .handle_platform_output(window, platform_output);

        self.shapes = shapes;
        self.pixels_per_point = pixels_per_point;
        self.textures_delta.append(textures_delta);
    }

    /// Paint the results of the last call to [`Self::run`].
    pub fn paint(
        &mut self,
        ctx: &VkContext,
        cmd: ash::vk::CommandBuffer,
        dimensions: [u32; 2],
        image_index: u32,
        frame_index: usize,
    ) -> anyhow::Result<()> {
        let shapes = std::mem::take(&mut self.shapes);
        let mut textures_delta = std::mem::take(&mut self.textures_delta);

        for (id, image_delta) in textures_delta.set {
            self.painter.set_texture(ctx, id, &image_delta, frame_index)?;
        }

        let pixels_per_point = self.pixels_per_point;
        let clipped_primitives = self.egui_ctx.tessellate(shapes, pixels_per_point);

        self.painter.paint_primitives(
            ctx,
            cmd,
            dimensions,
            pixels_per_point,
            &clipped_primitives,
            image_index,
            frame_index,
        )?;

        for id in textures_delta.free.drain(..) {
            self.painter.free_texture(id, frame_index);
        }

        Ok(())
    }

    /// Resize egui when swapchain is recreated.
    pub fn resize(&mut self, ctx: &VkContext, swapchain: &Swapchain) {
        self.painter.resize(ctx, swapchain);
    }

    /// Call to release the allocated graphics resources.
    pub fn destroy(&mut self, ctx: &VkContext) {
        self.painter.destroy(ctx);
    }
}
