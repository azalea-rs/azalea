use std::{collections::HashMap, mem::offset_of, sync::Arc};

use ash::{Device, vk};
use azalea::core::position::ChunkSectionPos;
use image::GenericImageView;
use vk_mem::{Alloc, Allocation, AllocationCreateInfo, MemoryUsage};

use crate::renderer::{
    assets::{Assets, processed::atlas::TextureEntry},
    chunk::LocalSection,
    mesh::Mesh,
    vulkan::{
        buffer::Buffer, context::VkContext, frame_sync::MAX_FRAMES_IN_FLIGHT, swapchain::Swapchain,
        texture::Texture,
    },
    world_renderer::{
        animation::AnimationManager,
        mesher::{MeshResult, Mesher},
    },
};

mod animation;
pub mod mesher;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BlockVertex {
    pub position: [f32; 3],
    pub ao: f32,
    pub uv: [f32; 2],
    pub tint: [f32; 3],
}

const TRIANGLE_VERT: &[u8] = include_bytes!(env!("BLOCK_VERT"));
const TRIANGLE_FRAG: &[u8] = include_bytes!(env!("BLOCK_FRAG"));

const WATER_VERT: &[u8] = include_bytes!(env!("WATER_VERT"));
const WATER_FRAG: &[u8] = include_bytes!(env!("WATER_FRAG"));

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

    animation_manager: AnimationManager,
    pub block_meshes: HashMap<ChunkSectionPos, Mesh<BlockVertex>>,
    pub water_meshes: HashMap<ChunkSectionPos, Mesh<BlockVertex>>,

    // Vulkan resources owned by this renderer
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    wireframe_pipeline: Option<vk::Pipeline>,
    water_pipeline: vk::Pipeline,
    descriptor_set_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    descriptor_set: vk::DescriptorSet,

    // Depth buffer resources
    depth_images: Vec<(vk::Image, Allocation, vk::ImageView)>,

    // Framebuffers for world rendering
    framebuffers: Vec<vk::Framebuffer>,

    // Texture resources
    blocks_texture: Texture,

    staging_buffers: [Option<Buffer>; MAX_FRAMES_IN_FLIGHT],

    // Cached extent for recreation
    extent: vk::Extent2D,

    assets: Arc<Assets>,
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
        assets: Arc<Assets>,
        ctx: &VkContext,
        swapchain: &Swapchain,
        options: WorldRendererOptions,
    ) -> Self {
        let atlas_image = animation::create_initial_atlas(&assets.block_atlas, &assets.textures);
        let blocks_texture = Texture::new(ctx, atlas_image);

        let descriptor_set_layout = create_world_descriptor_set_layout(ctx.device());

        let descriptor_pool = create_world_descriptor_pool(ctx.device());
        let descriptor_set =
            allocate_world_descriptor_set(ctx.device(), descriptor_pool, descriptor_set_layout);
        update_world_texture_descriptor(ctx.device(), descriptor_set, &blocks_texture);

        let render_pass = create_world_render_pass(ctx, swapchain);

        let depth_images = create_world_depth_resources(ctx, swapchain);

        let framebuffers = create_world_framebuffers(ctx, swapchain, render_pass, &depth_images);

        let pipeline_layout = create_world_pipeline_layout(ctx.device(), descriptor_set_layout);

        let pipeline = create_world_pipeline(
            ctx,
            render_pass,
            pipeline_layout,
            TRIANGLE_VERT,
            TRIANGLE_FRAG,
        );
        let wireframe_pipeline = if options.wireframe_enabled {
            create_world_wireframe_pipeline(
                ctx,
                render_pass,
                pipeline_layout,
                TRIANGLE_VERT,
                TRIANGLE_FRAG,
            )
        } else {
            None
        };
        let water_pipeline =
            create_water_pipeline(ctx, render_pass, pipeline_layout, WATER_VERT, WATER_FRAG);

        Self {
            mesher: Mesher::new(assets.clone()),
            animation_manager: AnimationManager::from_textures(&assets.textures),
            staging_buffers: Default::default(),
            block_meshes: HashMap::new(),
            water_meshes: HashMap::new(),
            render_pass,
            pipeline_layout,
            pipeline,
            wireframe_pipeline,
            water_pipeline,
            descriptor_set_layout,
            descriptor_pool,
            descriptor_set,
            depth_images,
            framebuffers,
            blocks_texture,
            extent: swapchain.extent,
            assets: assets.clone(),
        }
    }

    pub fn tick(&mut self) {
        self.animation_manager.tick(&self.assets.textures);
    }

    pub fn update_section(&self, section: LocalSection) {
        self.mesher.submit(section);
    }

    pub fn process_meshing_results(&mut self, ctx: &VkContext) {
        while let Some(MeshResult { blocks, water }) = self.mesher.poll() {
            if !blocks.vertices.is_empty() {
                let mut staging = Mesh::new_staging(ctx, &blocks.vertices, &blocks.indices);

                let mesh = staging.upload(ctx);
                staging.destroy(ctx);

                if let Some(mut old_mesh) = self.block_meshes.insert(blocks.section_pos, mesh) {
                    old_mesh.destroy(ctx);
                }
            }

            if !water.vertices.is_empty() {
                let mut staging = Mesh::new_staging(ctx, &water.vertices, &water.indices);

                let mesh = staging.upload(ctx);
                staging.destroy(ctx);

                if let Some(mut old_mesh) = self.water_meshes.insert(water.section_pos, mesh) {
                    old_mesh.destroy(ctx);
                }
            }
        }
    }

    pub fn draw(
        &mut self,
        ctx: &VkContext,
        cmd: vk::CommandBuffer,
        view_proj: glam::Mat4,
        wireframe_mode: bool,
        camera_pos: glam::Vec3,
        frame_index: usize
    ) {

        let device = ctx.device();
        let push = PushConstants { view_proj };

        let current_pipeline = if wireframe_mode {
            self.wireframe_pipeline.unwrap_or(self.pipeline)
        } else {
            self.pipeline
        };

        unsafe {
            device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, current_pipeline);

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

        for (_, mesh) in &self.block_meshes {
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

        unsafe {
            device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, self.water_pipeline);

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

        let mut water_meshes: Vec<_> = self.water_meshes.iter().collect();
        water_meshes.sort_by(|(pos_a, _), (pos_b, _)| {
            let center_a = glam::Vec3::new(
                pos_a.x as f32 * 16.0 + 8.0,
                pos_a.y as f32 * 16.0 + 8.0,
                pos_a.z as f32 * 16.0 + 8.0,
            );
            let center_b = glam::Vec3::new(
                pos_b.x as f32 * 16.0 + 8.0,
                pos_b.y as f32 * 16.0 + 8.0,
                pos_b.z as f32 * 16.0 + 8.0,
            );

            let dist_a = camera_pos.distance_squared(center_a);
            let dist_b = camera_pos.distance_squared(center_b);

            dist_b
                .partial_cmp(&dist_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for (_, mesh) in water_meshes {
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

    pub fn end_render_pass(&self, device: &ash::Device, cmd: vk::CommandBuffer) {
        unsafe {
            device.cmd_end_render_pass(cmd);
        }
    }

pub fn upload_dirty_textures(
    &mut self,
    ctx: &VkContext,
    cmd: vk::CommandBuffer,
    frame_index: usize,
) {
    let dirty = self.animation_manager.dirty_textures(&self.assets.textures);
    if dirty.is_empty() {
        return;
    }

    let mut buffer_data = Vec::new();
    let mut regions = Vec::new();

    for (name, tex, (fw, fh), frame_idx) in dirty {
        if let Some(placed) = self.assets.block_atlas.sprites.get(name) {
            let (fx, fy) = tex
                .animation
                .as_ref()
                .unwrap()
                .get_frame(frame_idx, tex.size());

            let frame_img = tex.data.view(fx, fy, fw, fh).to_image();
            let bytes = frame_img.as_raw();

            let offset = buffer_data.len() as vk::DeviceSize;
            buffer_data.extend_from_slice(bytes);

            regions.push(
                vk::BufferImageCopy::default()
                    .buffer_offset(offset)
                    .buffer_row_length(0)
                    .buffer_image_height(0)
                    .image_subresource(
                        vk::ImageSubresourceLayers::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .mip_level(0)
                            .base_array_layer(0)
                            .layer_count(1),
                    )
                    .image_offset(vk::Offset3D {
                        x: placed.x as i32,
                        y: placed.y as i32,
                        z: 0,
                    })
                    .image_extent(vk::Extent3D {
                        width: fw,
                        height: fh,
                        depth: 1,
                    }),
            );
        }
    }

    let needed_size = buffer_data.len() as vk::DeviceSize;

    let staging = if let Some(ref mut buf) = self.staging_buffers[frame_index] {
        if buf.size >= needed_size {
            buf.upload_data(ctx, 0, &buffer_data);
            buf
        } else {
            buf.destroy(ctx);
            let mut new_buf = Buffer::new(
                ctx,
                needed_size,
                vk::BufferUsageFlags::TRANSFER_SRC,
                MemoryUsage::AutoPreferHost,
                true,
            );
            new_buf.upload_data(ctx, 0, &buffer_data);
            self.staging_buffers[frame_index] = Some(new_buf);
            self.staging_buffers[frame_index].as_mut().unwrap()
        }
    } else {
        let mut new_buf = Buffer::new(
            ctx,
            needed_size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            MemoryUsage::AutoPreferHost,
            true,
        );
        new_buf.upload_data(ctx, 0, &buffer_data);
        self.staging_buffers[frame_index] = Some(new_buf);
        self.staging_buffers[frame_index].as_mut().unwrap()
    };

    unsafe {
        let subresource = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };

        ctx.device().cmd_pipeline_barrier(
            cmd,
            vk::PipelineStageFlags::FRAGMENT_SHADER,
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[vk::ImageMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::SHADER_READ)
                .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .old_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .image(self.blocks_texture.image)
                .subresource_range(subresource)],
        );

        ctx.device().cmd_copy_buffer_to_image(
            cmd,
            staging.buffer,
            self.blocks_texture.image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &regions,
        );

        ctx.device().cmd_pipeline_barrier(
            cmd,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::FRAGMENT_SHADER,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[vk::ImageMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::SHADER_READ)
                .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .image(self.blocks_texture.image)
                .subresource_range(subresource)],
        );
    }
}

    pub fn render(
        &mut self,
        ctx: &VkContext,
        cmd: vk::CommandBuffer,
        image_index: u32,
        extent: vk::Extent2D,
        view_proj: glam::Mat4,
        wireframe_mode: bool,
        camera_pos: glam::Vec3,
        frame_index: usize,
    ) {

        self.upload_dirty_textures(ctx, cmd, frame_index);
        self.begin_render_pass(ctx.device(), cmd, image_index, extent);

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
            ctx.device().cmd_set_viewport(cmd, 0, &[viewport]);
            ctx.device().cmd_set_scissor(cmd, 0, &[scissor]);
        }

        self.draw(ctx, cmd, view_proj, wireframe_mode, camera_pos, frame_index);

        self.end_render_pass(ctx.device(), cmd);
    }

    pub fn recreate_swapchain(&mut self, ctx: &VkContext, swapchain: &Swapchain) {
        let device = ctx.device();

        // Destroy old framebuffers
        for framebuffer in self.framebuffers.drain(..) {
            unsafe { device.destroy_framebuffer(framebuffer, None) };
        }

        // Destroy old depth resources
        for (image, mut alloc, view) in self.depth_images.drain(..) {
            unsafe {
                device.destroy_image_view(view, None);
                ctx.allocator().destroy_image(image, &mut alloc);
            }
        }

        // Recreate depth resources
        self.depth_images = create_world_depth_resources(ctx, swapchain);

        // Recreate framebuffers
        self.framebuffers =
            create_world_framebuffers(ctx, swapchain, self.render_pass, &self.depth_images);

        self.extent = swapchain.extent;
    }

    pub fn destroy(&mut self, ctx: &VkContext) {
        let device = ctx.device();

        for (_pos, mut mesh) in self.block_meshes.drain() {
            mesh.destroy(ctx);
        }

        for (_pos, mut mesh) in self.water_meshes.drain() {
            mesh.destroy(ctx);
        }

        for framebuffer in self.framebuffers.drain(..) {
            unsafe { device.destroy_framebuffer(framebuffer, None) };
        }

        for (image, mut alloc, view) in self.depth_images.drain(..) {
            unsafe {
                device.destroy_image_view(view, None);
                ctx.allocator().destroy_image(image, &mut alloc);
            }
        }

        self.blocks_texture.destroy(ctx);

        unsafe {
            for buffer in &mut self.staging_buffers{
                if let Some(buffer) = buffer{
                    buffer.destroy(ctx);
                }
            }
            device.destroy_descriptor_pool(self.descriptor_pool, None);
            device.destroy_pipeline(self.pipeline, None);
            if let Some(wireframe_pipeline) = self.wireframe_pipeline.take() {
                device.destroy_pipeline(wireframe_pipeline, None);
            }
            device.destroy_pipeline(self.water_pipeline, None);
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

pub fn create_water_pipeline(
    ctx: &VkContext,
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    vert_spv: &[u8],
    frag_spv: &[u8],
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
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::BACK)
        .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
        .line_width(1.0);

    let multisampling = vk::PipelineMultisampleStateCreateInfo::default()
        .rasterization_samples(vk::SampleCountFlags::TYPE_1);

    // Enable alpha blending for water transparency
    let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
        .color_write_mask(
            vk::ColorComponentFlags::R
                | vk::ColorComponentFlags::G
                | vk::ColorComponentFlags::B
                | vk::ColorComponentFlags::A,
        )
        .blend_enable(true)
        .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
        .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
        .color_blend_op(vk::BlendOp::ADD)
        .src_alpha_blend_factor(vk::BlendFactor::ONE)
        .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
        .alpha_blend_op(vk::BlendOp::ADD);

    let depth_stencil = vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(true)
        .depth_write_enable(false) // Don't write to depth buffer for transparency
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
            .expect("Failed to create water pipeline")
    };
    let pipeline = pipelines[0];

    unsafe {
        device.destroy_shader_module(vert_module, None);
        device.destroy_shader_module(frag_module, None);
    }

    pipeline
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

    let dependencies = [
        vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE),
        vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(
                vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
                    | vk::PipelineStageFlags::LATE_FRAGMENT_TESTS,
            )
            .src_access_mask(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS)
            .dst_access_mask(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE),
    ];

    let subpass = vk::SubpassDescription::default()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(std::slice::from_ref(&color_ref))
        .depth_stencil_attachment(&depth_ref);

    let attachments = [color_attachment, depth_attachment];

    let render_pass_info = vk::RenderPassCreateInfo::default()
        .attachments(&attachments)
        .subpasses(std::slice::from_ref(&subpass))
        .dependencies(&dependencies);

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
    swapchain: &Swapchain,
) -> Vec<(vk::Image, Allocation, vk::ImageView)> {
    let format = vk::Format::D32_SFLOAT;

    swapchain
        .image_views
        .iter()
        .map(|_| {
            let image_info = vk::ImageCreateInfo::default()
                .image_type(vk::ImageType::TYPE_2D)
                .format(format)
                .extent(vk::Extent3D {
                    width: swapchain.extent.width,
                    height: swapchain.extent.height,
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
        })
        .collect()
}

pub fn create_world_framebuffers(
    ctx: &VkContext,
    swapchain: &Swapchain,
    render_pass: vk::RenderPass,
    depth_resources: &[(vk::Image, Allocation, vk::ImageView)],
) -> Vec<vk::Framebuffer> {
    let device = ctx.device();
    let mut framebuffers = Vec::with_capacity(swapchain.image_views.len());

    for (i, &view) in swapchain.image_views.iter().enumerate() {
        let depth_view = depth_resources[i].2;
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

fn calc_dirty_size(textures: &HashMap<String, TextureEntry>, dirty: &[&str]) -> vk::DeviceSize {
    dirty
        .iter()
        .filter_map(|name| textures.get(*name))
        .map(|tex| {
            let (fw, fh) = tex.size();
            (fw * fh * 4) as vk::DeviceSize
        })
        .sum()
}
