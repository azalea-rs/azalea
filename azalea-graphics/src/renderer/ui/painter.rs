use std::collections::HashMap;

use ash::vk;
use egui::{
    TextureId,
    emath::Rect,
    epaint::{Mesh, PaintCallbackInfo, Primitive, Vertex},
};

use crate::renderer::{
    mesh::Mesh as GpuMesh, ui::{passes::create_egui_render_pass, pipelines::create_egui_pipeline}, vulkan::{
        context::VkContext, frame_sync::MAX_FRAMES_IN_FLIGHT, swapchain::Swapchain,
        texture::Texture, 
    }
};

/// Per-frame data for egui rendering.
#[derive(Default)]
struct FrameData {
    meshes: Vec<GpuMesh<Vertex>>,
}

impl FrameData {
    fn destroy(&mut self, ctx: &VkContext) {
        for mut mesh in &mut self.meshes.drain(..) {
            mesh.destroy(ctx);
        }
    }

    fn clear_buffers(&mut self, ctx: &VkContext) {
        self.destroy(ctx);
        self.meshes.clear();
    }
}

/// A Vulkan-specific callback function that can be used to compose an
/// [`egui::PaintCallback`] for custom rendering.
pub struct CallbackFn {
    f: Box<dyn Fn(PaintCallbackInfo, &Painter) + Sync + Send>,
}

impl CallbackFn {
    pub fn new<F: Fn(PaintCallbackInfo, &Painter) + Sync + Send + 'static>(callback: F) -> Self {
        let f = Box::new(callback);
        Self { f }
    }
}

/// A Vulkan painter for egui using ash and vk-mem.
pub struct Painter {
    // Vulkan objects
    render_pass: vk::RenderPass,
    pipeline: vk::Pipeline,
    pipeline_layout: vk::PipelineLayout,
    descriptor_set_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,

    // Framebuffers for egui rendering
    framebuffers: Vec<vk::Framebuffer>,
    swapchain_format: vk::Format,
    extent: vk::Extent2D,

    // Per-frame data
    frame_data: [FrameData; MAX_FRAMES_IN_FLIGHT],

    // Textures
    textures: HashMap<TextureId, Texture>,
    texture_descriptor_sets: HashMap<TextureId, vk::DescriptorSet>,

    next_native_tex_id: u64,

    /// Stores outdated Vulkan textures that are yet to be deleted
    textures_to_destroy: Vec<Texture>,

    /// Used to make sure we are destroyed correctly.
    destroyed: bool,
}

impl Painter {
    /// Create a new Vulkan egui painter.
    pub fn new(ctx: &VkContext, swapchain: &Swapchain) -> anyhow::Result<Self> {
        let device = ctx.device();

        // Create egui render pass
        let render_pass = create_egui_render_pass(ctx, swapchain.format);

        // Create framebuffers for egui
        let framebuffers = Self::create_framebuffers(ctx, swapchain, render_pass);

        // Create descriptor set layout for textures
        let sampler_binding = vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT);

        let descriptor_set_layout_info = vk::DescriptorSetLayoutCreateInfo::default()
            .bindings(std::slice::from_ref(&sampler_binding));

        let descriptor_set_layout = unsafe {
            device
                .create_descriptor_set_layout(&descriptor_set_layout_info, None)
                .map_err(|e| anyhow::anyhow!("Failed to create descriptor set layout: {:?}", e))?
        };

        // Create descriptor pool
        let pool_size = vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1000); // Support many textures

        let descriptor_pool_info = vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(std::slice::from_ref(&pool_size))
            .max_sets(1000)
            .flags(vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET);

        let descriptor_pool = unsafe {
            device
                .create_descriptor_pool(&descriptor_pool_info, None)
                .map_err(|e| anyhow::anyhow!("Failed to create descriptor pool: {:?}", e))?
        };

        // Create pipeline layout
        let push_constant_range = vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::VERTEX)
            .offset(0)
            .size(std::mem::size_of::<EguiPushConstants>() as u32);

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(std::slice::from_ref(&descriptor_set_layout))
            .push_constant_ranges(std::slice::from_ref(&push_constant_range));

        let pipeline_layout = unsafe {
            device
                .create_pipeline_layout(&pipeline_layout_info, None)
                .map_err(|e| anyhow::anyhow!("Failed to create pipeline layout: {:?}", e))?
        };

        // Create pipeline (simplified - you'll need actual shaders)
        let pipeline = create_egui_pipeline(device, render_pass, pipeline_layout)?;

        Ok(Self {
            render_pass,
            pipeline,
            pipeline_layout,
            descriptor_set_layout,
            descriptor_pool,
            framebuffers,
            swapchain_format: swapchain.format,
            extent: swapchain.extent,
            frame_data: Default::default(),
            textures: HashMap::new(),
            texture_descriptor_sets: HashMap::new(),
            next_native_tex_id: 1 << 32,
            textures_to_destroy: Vec::new(),
            destroyed: false,
        })
    }

    fn create_framebuffers(
        ctx: &VkContext,
        swapchain: &Swapchain,
        render_pass: vk::RenderPass,
    ) -> Vec<vk::Framebuffer> {
        let device = ctx.device();
        let mut framebuffers = Vec::with_capacity(swapchain.image_views.len());

        for &image_view in &swapchain.image_views {
            let attachments = [image_view];
            let framebuffer_info = vk::FramebufferCreateInfo::default()
                .render_pass(render_pass)
                .attachments(&attachments)
                .width(swapchain.extent.width)
                .height(swapchain.extent.height)
                .layers(1);

            let framebuffer = unsafe {
                device
                    .create_framebuffer(&framebuffer_info, None)
                    .expect("Failed to create egui framebuffer")
            };
            framebuffers.push(framebuffer);
        }

        framebuffers
    }

    /// Resize the painter's framebuffers when the swapchain is recreated.
    pub fn resize(&mut self, ctx: &VkContext, swapchain: &Swapchain) {
        // Clear all frame data first
        for frame_data in &mut self.frame_data {
            frame_data.destroy(ctx);
        }

        // Destroy old framebuffers
        for framebuffer in &self.framebuffers {
            unsafe {
                ctx.device().destroy_framebuffer(*framebuffer, None);
            }
        }

        // Update stored values
        self.extent = swapchain.extent;
        self.swapchain_format = swapchain.format;

        // Recreate framebuffers
        self.framebuffers = Self::create_framebuffers(ctx, swapchain, self.render_pass);
    }

    /// Paint egui primitives to the current command buffer with its own render
    /// pass.
    pub fn paint_primitives(
        &mut self,
        ctx: &VkContext,
        cmd: vk::CommandBuffer,
        screen_size_px: [u32; 2],
        pixels_per_point: f32,
        clipped_primitives: &[egui::ClippedPrimitive],
        image_index: u32,
        frame_index: usize,
    ) -> anyhow::Result<()> {
        self.assert_not_destroyed();

        // Clear previous frame's buffers
        self.frame_data[frame_index].clear_buffers(ctx);

        // Begin egui render pass
        let render_pass_info = vk::RenderPassBeginInfo::default()
            .render_pass(self.render_pass)
            .framebuffer(self.framebuffers[image_index as usize])
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.extent,
            })
            .clear_values(&[]); // No clear values for egui overlay

        unsafe {
            ctx.device()
                .cmd_begin_render_pass(cmd, &render_pass_info, vk::SubpassContents::INLINE);

            ctx.device()
                .cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline);
        }

        let push_constants = EguiPushConstants {
            screen_size: [screen_size_px[0] as f32, screen_size_px[1] as f32],
        };

        unsafe {
            ctx.device().cmd_push_constants(
                cmd,
                self.pipeline_layout,
                vk::ShaderStageFlags::VERTEX,
                0,
                std::slice::from_raw_parts(
                    &push_constants as *const _ as *const u8,
                    std::mem::size_of::<EguiPushConstants>(),
                ),
            );
        }

        for egui::ClippedPrimitive {
            clip_rect,
            primitive,
        } in clipped_primitives
        {
            match primitive {
                Primitive::Mesh(mesh) => {
                    if let Some(texture_id) = self.texture_descriptor_sets.get(&mesh.texture_id) {
                        unsafe {
                            ctx.device().cmd_bind_descriptor_sets(
                                cmd,
                                vk::PipelineBindPoint::GRAPHICS,
                                self.pipeline_layout,
                                0,
                                std::slice::from_ref(texture_id),
                                &[],
                            );
                        }
                    }

                    self.set_clip_rect(ctx, cmd, screen_size_px, pixels_per_point, *clip_rect);
                    self.paint_mesh(ctx, cmd, mesh, frame_index)?;
                }
                Primitive::Callback(cb) => {
                    if let Some(callback) = cb.callback.downcast_ref::<CallbackFn>() {
                        let info = PaintCallbackInfo {
                            viewport: cb.rect,
                            clip_rect: *clip_rect,
                            pixels_per_point,
                            screen_size_px,
                        };
                        (callback.f)(info, self);
                    }
                }
            }
        }

        // End egui render pass
        unsafe {
            ctx.device().cmd_end_render_pass(cmd);
        }

        Ok(())
    }

    fn paint_mesh(
        &mut self,
        ctx: &VkContext,
        cmd: vk::CommandBuffer,
        mesh: &Mesh,
        frame_index: usize,
    ) -> anyhow::Result<()> {
        if mesh.vertices.is_empty() || mesh.indices.is_empty() {
            return Ok(());
        }

        // Create buffers for this mesh
        let mesh = GpuMesh::new_host(ctx, &mesh.vertices, &mesh.indices);




        unsafe {
            ctx.device()
                .cmd_bind_vertex_buffers(cmd, 0, &[mesh.buffer.buffer], &[mesh.vertex_offset]);
            ctx.device()
                .cmd_bind_index_buffer(cmd, mesh.buffer.buffer, mesh.index_offset, vk::IndexType::UINT32);
            ctx.device()
                .cmd_draw_indexed(cmd, mesh.index_count, 1, 0, 0, 0);
        }
        self.frame_data[frame_index]
            .meshes
            .push(mesh);

        Ok(())
    }

    fn set_clip_rect(
        &self,
        ctx: &VkContext,
        cmd: vk::CommandBuffer,
        [width_px, height_px]: [u32; 2],
        pixels_per_point: f32,
        clip_rect: Rect,
    ) {
        // Convert egui coordinates to Vulkan viewport coordinates
        let clip_min_x = (pixels_per_point * clip_rect.min.x).max(0.0);
        let clip_min_y = (pixels_per_point * clip_rect.min.y).max(0.0);
        let clip_max_x = (pixels_per_point * clip_rect.max.x).min(width_px as f32);
        let clip_max_y = (pixels_per_point * clip_rect.max.y).min(height_px as f32);

        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: width_px as f32,
            height: height_px as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        let scissor = vk::Rect2D {
            offset: vk::Offset2D {
                x: clip_min_x as i32,
                y: clip_min_y as i32,
            },
            extent: vk::Extent2D {
                width: ((clip_max_x - clip_min_x) as u32).max(1),
                height: ((clip_max_y - clip_min_y) as u32).max(1),
            },
        };

        unsafe {
            ctx.device().cmd_set_viewport(cmd, 0, &[viewport]);
            ctx.device().cmd_set_scissor(cmd, 0, &[scissor]);
        }
    }

    /// Set the texture to use for the given texture ID.
    pub fn set_texture(
        &mut self,
        ctx: &VkContext,
        tex_id: TextureId,
        delta: &egui::epaint::ImageDelta,
    ) -> anyhow::Result<()> {
        self.assert_not_destroyed();

        let texture = match &delta.image {
            egui::epaint::ImageData::Color(color_image) => {
                Texture::from_egui_image(ctx, color_image, delta.options)
            }
        };

        let descriptor_set = self.create_descriptor_set_for_texture(ctx, &texture)?;
        self.texture_descriptor_sets.insert(tex_id, descriptor_set);
        self.textures.insert(tex_id, texture);

        Ok(())
    }

    fn create_descriptor_set_for_texture(
        &mut self,
        ctx: &VkContext,
        texture: &Texture,
    ) -> anyhow::Result<vk::DescriptorSet> {
        let allocate_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(std::slice::from_ref(&self.descriptor_set_layout));

        let descriptor_sets = unsafe {
            ctx.device()
                .allocate_descriptor_sets(&allocate_info)
                .map_err(|e| anyhow::anyhow!("Failed to allocate descriptor set: {:?}", e))?
        };

        let descriptor_set = descriptor_sets[0];

        let image_info = vk::DescriptorImageInfo::default()
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .image_view(texture.view)
            .sampler(texture.sampler);

        let descriptor_write = vk::WriteDescriptorSet::default()
            .dst_set(descriptor_set)
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info(std::slice::from_ref(&image_info));

        unsafe {
            ctx.device()
                .update_descriptor_sets(&[descriptor_write], &[]);
        }

        Ok(descriptor_set)
    }

    /// Free the texture with the given ID.
    pub fn free_texture(&mut self, tex_id: TextureId) {
        self.assert_not_destroyed();

        if let Some(texture) = self.textures.remove(&tex_id) {
            self.textures_to_destroy.push(texture);
        }
        self.texture_descriptor_sets.remove(&tex_id);
    }

    /// Register a native texture and return a texture ID.
    pub fn register_native_texture(&mut self, texture: Texture) -> TextureId {
        self.assert_not_destroyed();
        let tex_id = TextureId::User(self.next_native_tex_id);
        self.next_native_tex_id += 1;
        self.textures.insert(tex_id, texture);
        tex_id
    }

    /// Destroy the painter and clean up Vulkan resources.
    pub fn destroy(&mut self, ctx: &VkContext) {
        if !self.destroyed {
            unsafe {
                let device = ctx.device();

                // Clean up buffers handled in frame_data cleanup above

                // Clean up textures
                for texture in self.textures.values_mut() {
                    texture.destroy(ctx);
                }

                for texture in &mut self.textures_to_destroy {
                    texture.destroy(ctx);
                }

                // Clean up per-frame data
                for frame_data in &mut self.frame_data {
                    frame_data.destroy(ctx);
                }

                // Clean up framebuffers
                for framebuffer in &self.framebuffers {
                    device.destroy_framebuffer(*framebuffer, None);
                }

                // Clean up Vulkan objects
                device.destroy_pipeline(self.pipeline, None);
                device.destroy_pipeline_layout(self.pipeline_layout, None);
                device.destroy_descriptor_pool(self.descriptor_pool, None);
                device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);
                device.destroy_render_pass(self.render_pass, None);
            }
            self.destroyed = true;
        }
    }

    fn assert_not_destroyed(&self) {
        assert!(
            !self.destroyed,
            "The egui painter has already been destroyed!"
        );
    }
}

impl Drop for Painter {
    fn drop(&mut self) {
        if !self.destroyed {
            log::warn!("You forgot to call destroy() on the egui painter. Resources will leak!");
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct EguiPushConstants {
    screen_size: [f32; 2],
}
