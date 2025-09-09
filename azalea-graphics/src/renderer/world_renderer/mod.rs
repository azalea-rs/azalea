use std::{collections::HashMap, mem::offset_of, sync::Arc};

use ash::{Device, vk};
use azalea::core::position::ChunkSectionPos;
use vk_mem::{Alloc, Allocation, AllocationCreateInfo, MemoryUsage};

use crate::renderer::{
    assets::MeshAssets, chunk::LocalSection, mesh::Mesh, vulkan::{context::VkContext, swapchain::Swapchain, texture::Texture}, world_renderer::mesher::Mesher
};

pub mod mesher;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BlockVertex {
    pub position: [f32; 3],
    pub ao: f32,
    pub uv: [f32; 2],
    pub tint: [f32; 3],
}

impl BlockVertex {
    fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(std::mem::size_of::<BlockVertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
    }

    fn attribute_descriptions() -> &'static [vk::VertexInputAttributeDescription] {
        &[
            // position
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(BlockVertex, position) as u32,
            },
            // ao
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 1,
                format: vk::Format::R32_SFLOAT,
                offset: offset_of!(BlockVertex, ao) as u32,
            },
            // uv
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 2,
                format: vk::Format::R32G32_SFLOAT,
                offset: offset_of!(BlockVertex, uv) as u32,
            },
            // tint
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 3,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(BlockVertex, tint) as u32,
            },
        ]
    }
}

pub struct WorldRenderer {
    pub mesher: Mesher,
    pub meshes: HashMap<ChunkSectionPos, Mesh<BlockVertex>>,

    // Vulkan resources owned by this renderer
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    wireframe_pipeline: Option<vk::Pipeline>,
    descriptor_set_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    descriptor_set: vk::DescriptorSet,

    // Depth buffer resources
    depth_image: vk::Image,
    depth_allocation: Allocation,
    depth_view: vk::ImageView,

    // Framebuffers for world rendering
    framebuffers: Vec<vk::Framebuffer>,

    // Texture resources
    blocks_texture: Texture,

    // Cached extent for recreation
    extent: vk::Extent2D,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PushConstants {
    pub view_proj: glam::Mat4,
}

pub struct WorldRendererOptions {
    pub wireframe_enabled: bool,
}

impl Default for WorldRendererOptions {
    fn default() -> Self {
        Self {
            wireframe_enabled: false,
        }
    }
}

impl WorldRenderer {
    pub fn new(
        assets: Arc<MeshAssets>,
        atlas_image: image::RgbaImage,
        ctx: &VkContext,
        swapchain: &Swapchain,
        vert_spv: &[u8],
        frag_spv: &[u8],
        options: WorldRendererOptions,
    ) -> Self {
        // Create texture from atlas
        let blocks_texture = Texture::new(ctx, atlas_image);

        // Create descriptor set layout
        let descriptor_set_layout = create_world_descriptor_set_layout(ctx.device());

        // Create descriptor pool and set
        let descriptor_pool = create_world_descriptor_pool(ctx.device());
        let descriptor_set =
            allocate_world_descriptor_set(ctx.device(), descriptor_pool, descriptor_set_layout);
        update_world_texture_descriptor(ctx.device(), descriptor_set, &blocks_texture);

        // Create render pass
        let render_pass = create_world_render_pass(ctx, swapchain);

        // Create depth resources
        let (depth_image, depth_allocation, depth_view) =
            create_world_depth_resources(ctx, swapchain.extent);

        // Create framebuffers
        let framebuffers = create_world_framebuffers(ctx, swapchain, render_pass, depth_view);

        // Create pipeline layout
        let pipeline_layout = create_world_pipeline_layout(ctx.device(), descriptor_set_layout);

        // Create pipelines
        let pipeline = create_world_pipeline(ctx, render_pass, pipeline_layout, vert_spv, frag_spv);
        let wireframe_pipeline = if options.wireframe_enabled {
            create_world_wireframe_pipeline(ctx, render_pass, pipeline_layout, vert_spv, frag_spv)
        } else {
            None
        };

        Self {
            mesher: Mesher::new(assets),
            meshes: HashMap::new(),
            render_pass,
            pipeline_layout,
            pipeline,
            wireframe_pipeline,
            descriptor_set_layout,
            descriptor_pool,
            descriptor_set,
            depth_image,
            depth_allocation,
            depth_view,
            framebuffers,
            blocks_texture,
            extent: swapchain.extent,
        }
    }

    /// Get the descriptor set layout for this world renderer
    pub fn descriptor_set_layout(&self) -> vk::DescriptorSetLayout {
        self.descriptor_set_layout
    }

    /// Submit a chunk for meshing (background thread will handle it)
    pub fn update_section(&self, section: LocalSection) {
        self.mesher.submit(section);
    }

    /// Poll mesher results and upload to GPU
    pub fn process_meshing_results(&mut self, ctx: &VkContext) {
        while let Some(mesh_data) = self.mesher.poll() {
            if mesh_data.vertices.is_empty() || mesh_data.indices.is_empty() {
                continue;
            }
            let mesh = Mesh::new_staging(ctx, &mesh_data.vertices, &mesh_data.indices).upload(ctx);

            self.meshes.insert(mesh_data.section_pos, mesh);
        }
    }

    /// Draw all chunk meshes
    pub fn draw(
        &self,
        device: &ash::Device,
        cmd: vk::CommandBuffer,
        view_proj: glam::Mat4,
        wireframe_mode: bool,
    ) {
        let current_pipeline = if wireframe_mode {
            self.wireframe_pipeline.unwrap_or(self.pipeline)
        } else {
            self.pipeline
        };
        unsafe {
            device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, current_pipeline);
            let push = PushConstants { view_proj };

            device.cmd_push_constants(
                cmd,
                self.pipeline_layout,
                vk::ShaderStageFlags::VERTEX,
                0,
                std::slice::from_raw_parts(
                    &push as *const PushConstants as *const u8,
                    std::mem::size_of::<PushConstants>(),
                ),
            );

            device.cmd_bind_descriptor_sets(
                cmd,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout,
                0,
                &[self.descriptor_set],
                &[],
            );
        }

        for (_, mesh) in &self.meshes {
            let vertex_buffers = [mesh.buffer.buffer];
            let offsets = [mesh.vertex_offset];
            unsafe {
                device.cmd_bind_vertex_buffers(cmd, 0, &vertex_buffers, &offsets);
                device.cmd_bind_index_buffer(
                    cmd,
                    mesh.buffer.buffer,
                    mesh.index_offset,
                    vk::IndexType::UINT32,
                );
                device.cmd_draw_indexed(cmd, mesh.index_count, 1, 0, 0, 0);
            }
        }
    }

    /// Begin the world render pass
    pub fn begin_render_pass(
        &self,
        device: &ash::Device,
        cmd: vk::CommandBuffer,
        image_index: u32,
        extent: vk::Extent2D,
    ) {
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
                extent,
            })
            .clear_values(&clear_values);

        unsafe {
            device.cmd_begin_render_pass(cmd, &rp_info, vk::SubpassContents::INLINE);
        }
    }

    /// End the world render pass
    pub fn end_render_pass(&self, device: &ash::Device, cmd: vk::CommandBuffer) {
        unsafe {
            device.cmd_end_render_pass(cmd);
        }
    }

    /// Render the world (handles full render pass lifecycle)
    pub fn render(
        &self,
        device: &ash::Device,
        cmd: vk::CommandBuffer,
        image_index: u32,
        extent: vk::Extent2D,
        view_proj: glam::Mat4,
        wireframe_mode: bool,
    ) {
        // Begin render pass
        self.begin_render_pass(device, cmd, image_index, extent);

        // Set viewport and scissor
        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: extent.width as f32,
            height: extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };
        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent,
        };
        unsafe {
            device.cmd_set_viewport(cmd, 0, &[viewport]);
            device.cmd_set_scissor(cmd, 0, &[scissor]);
        }

        // Draw world geometry
        self.draw(device, cmd, view_proj, wireframe_mode);

        // End render pass
        self.end_render_pass(device, cmd);
    }

    /// Recreate swapchain-dependent resources
    pub fn recreate_swapchain(&mut self, ctx: &VkContext, swapchain: &Swapchain) {
        let device = ctx.device();

        // Destroy old framebuffers
        for framebuffer in &self.framebuffers {
            unsafe {
                device.destroy_framebuffer(*framebuffer, None);
            }
        }

        // Destroy old depth resources
        unsafe {
            device.destroy_image_view(self.depth_view, None);
            ctx.allocator()
                .destroy_image(self.depth_image, &mut self.depth_allocation);
        }

        // Recreate depth resources
        let (depth_image, depth_allocation, depth_view) =
            create_world_depth_resources(ctx, swapchain.extent);
        self.depth_image = depth_image;
        self.depth_allocation = depth_allocation;
        self.depth_view = depth_view;

        // Recreate framebuffers
        self.framebuffers =
            create_world_framebuffers(ctx, swapchain, self.render_pass, self.depth_view);

        self.extent = swapchain.extent;
    }

    /// Clean up GPU resources
    pub fn destroy(&mut self, ctx: &VkContext) {
        let device = ctx.device();

        // Clean up meshes
        for (_pos, mut mesh) in self.meshes.drain() {
            mesh.destroy(ctx);
        }

        // Clean up Vulkan resources
        unsafe {
            device.destroy_pipeline(self.pipeline, None);
            if let Some(wireframe_pipeline) = self.wireframe_pipeline {
                device.destroy_pipeline(wireframe_pipeline, None);
            }
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            device.destroy_render_pass(self.render_pass, None);
            device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);
        }
    }
}

// World rendering pipeline and pass creation functions

fn create_shader_module(device: &Device, code: &[u8]) -> vk::ShaderModule {
    let code_aligned = ash::util::read_spv(&mut std::io::Cursor::new(code)).unwrap();
    let info = vk::ShaderModuleCreateInfo::default().code(&code_aligned);
    unsafe { device.create_shader_module(&info, None).unwrap() }
}

pub fn create_world_pipeline_layout(
    device: &Device,
    descriptor_set_layout: vk::DescriptorSetLayout,
) -> vk::PipelineLayout {
    let layouts = [descriptor_set_layout];
    let push_constant_range = vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::VERTEX)
        .offset(0)
        .size(std::mem::size_of::<PushConstants>() as u32);

    let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default()
        .set_layouts(&layouts)
        .push_constant_ranges(std::slice::from_ref(&push_constant_range));

    unsafe {
        device
            .create_pipeline_layout(&pipeline_layout_info, None)
            .unwrap()
    }
}

fn create_world_pipeline_with_mode(
    ctx: &VkContext,
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    vert_spv: &[u8],
    frag_spv: &[u8],
    polygon_mode: vk::PolygonMode,
) -> vk::Pipeline {
    let device = ctx.device();

    let vert_module = create_shader_module(device, vert_spv);
    let frag_module = create_shader_module(device, frag_spv);

    let entry_point = std::ffi::CString::new("main").unwrap();

    let shader_stages = [
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_module)
            .name(&entry_point),
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_module)
            .name(&entry_point),
    ];

    let binding_desc = [BlockVertex::binding_description()];
    let attribute_desc = BlockVertex::attribute_descriptions();

    let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
        .vertex_binding_descriptions(&binding_desc)
        .vertex_attribute_descriptions(&attribute_desc);
    let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false);

    let viewport_state = vk::PipelineViewportStateCreateInfo::default()
        .viewport_count(1)
        .scissor_count(1);

    let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
        .polygon_mode(polygon_mode)
        .cull_mode(vk::CullModeFlags::BACK)
        .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
        .line_width(1.0);

    let multisampling = vk::PipelineMultisampleStateCreateInfo::default()
        .rasterization_samples(vk::SampleCountFlags::TYPE_1);

    let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
        .color_write_mask(
            vk::ColorComponentFlags::R
                | vk::ColorComponentFlags::G
                | vk::ColorComponentFlags::B
                | vk::ColorComponentFlags::A,
        )
        .blend_enable(false);

    let depth_stencil = vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(true)
        .depth_write_enable(true)
        .depth_compare_op(vk::CompareOp::LESS)
        .depth_bounds_test_enable(false)
        .stencil_test_enable(false);

    let attachments = [color_blend_attachment];
    let color_blending = vk::PipelineColorBlendStateCreateInfo::default().attachments(&attachments);

    let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let dynamic_state =
        vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);

    let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
        .stages(&shader_stages)
        .vertex_input_state(&vertex_input)
        .input_assembly_state(&input_assembly)
        .viewport_state(&viewport_state)
        .rasterization_state(&rasterizer)
        .multisample_state(&multisampling)
        .depth_stencil_state(&depth_stencil)
        .color_blend_state(&color_blending)
        .dynamic_state(&dynamic_state)
        .layout(pipeline_layout)
        .render_pass(render_pass)
        .subpass(0);

    let pipelines = unsafe {
        device
            .create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
            .expect("Failed to create graphics pipeline")
    };
    let pipeline = pipelines[0];

    unsafe {
        device.destroy_shader_module(vert_module, None);
        device.destroy_shader_module(frag_module, None);
    }

    pipeline
}

pub fn create_world_pipeline(
    ctx: &VkContext,
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    vert_spv: &[u8],
    frag_spv: &[u8],
) -> vk::Pipeline {
    create_world_pipeline_with_mode(
        ctx,
        render_pass,
        pipeline_layout,
        vert_spv,
        frag_spv,
        vk::PolygonMode::FILL,
    )
}

pub fn create_world_wireframe_pipeline(
    ctx: &VkContext,
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    vert_spv: &[u8],
    frag_spv: &[u8],
) -> Option<vk::Pipeline> {
    if ctx.features().fill_mode_non_solid {
        Some(create_world_pipeline_with_mode(
            ctx,
            render_pass,
            pipeline_layout,
            vert_spv,
            frag_spv,
            vk::PolygonMode::LINE,
        ))
    } else {
        None
    }
}

pub fn create_world_render_pass(ctx: &VkContext, swapchain: &Swapchain) -> vk::RenderPass {
    let color_attachment = vk::AttachmentDescription::default()
        .format(swapchain.format)
        .samples(vk::SampleCountFlags::TYPE_1)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

    let depth_attachment = vk::AttachmentDescription::default()
        .format(vk::Format::D32_SFLOAT)
        .samples(vk::SampleCountFlags::TYPE_1)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::DONT_CARE)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

    let color_ref = vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    };
    let depth_ref = vk::AttachmentReference {
        attachment: 1,
        layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
    };

    let subpass = vk::SubpassDescription::default()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(std::slice::from_ref(&color_ref))
        .depth_stencil_attachment(&depth_ref);

    let attachments = [color_attachment, depth_attachment];

    let render_pass_info = vk::RenderPassCreateInfo::default()
        .attachments(&attachments)
        .subpasses(std::slice::from_ref(&subpass));

    unsafe {
        ctx.device()
            .create_render_pass(&render_pass_info, None)
            .expect("Failed to create render pass")
    }
}

pub fn create_world_descriptor_set_layout(device: &Device) -> vk::DescriptorSetLayout {
    let sampler_binding = vk::DescriptorSetLayoutBinding::default()
        .binding(0)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT);

    let info = vk::DescriptorSetLayoutCreateInfo::default()
        .bindings(std::slice::from_ref(&sampler_binding));

    unsafe { device.create_descriptor_set_layout(&info, None).unwrap() }
}

pub fn create_world_descriptor_pool(device: &Device) -> vk::DescriptorPool {
    let pool_size = vk::DescriptorPoolSize::default()
        .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1);

    let info = vk::DescriptorPoolCreateInfo::default()
        .pool_sizes(std::slice::from_ref(&pool_size))
        .max_sets(1);

    unsafe { device.create_descriptor_pool(&info, None).unwrap() }
}

pub fn allocate_world_descriptor_set(
    device: &Device,
    pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
) -> vk::DescriptorSet {
    let alloc_info = vk::DescriptorSetAllocateInfo::default()
        .descriptor_pool(pool)
        .set_layouts(std::slice::from_ref(&layout));

    unsafe { device.allocate_descriptor_sets(&alloc_info).unwrap()[0] }
}

pub fn update_world_texture_descriptor(
    device: &Device,
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

pub fn create_world_depth_resources(
    ctx: &VkContext,
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
        ctx.allocator()
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

pub fn create_world_framebuffers(
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
