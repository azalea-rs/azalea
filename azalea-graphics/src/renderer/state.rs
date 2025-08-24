use std::time::Duration;
use std::sync::Arc;

use crate::renderer::camera::{Camera, CameraController, Projection};
use crate::renderer::mesh::{Vertex};
use crate::renderer::mesher::LocalChunk;
use crate::renderer::render_world::{PushConstants, RenderWorld};
use crate::vulkan::frame_sync::MAX_FRAMES_IN_FLIGHT;
use crate::vulkan::{context::VkContext, frame_sync::FrameSync};
use crate::vulkan::swapchain::Swapchain;
use ash::vk::{self};
use ash::Device;
use azalea::core::position::ChunkPos;
use azalea::world::Chunk;
use parking_lot::RwLock;
use raw_window_handle::{DisplayHandle, WindowHandle};
use vk_mem::{Alloc, Allocation, AllocationCreateFlags, AllocationCreateInfo, Allocator, MemoryUsage};
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseScrollDelta};
use winit::keyboard::KeyCode;

const TRIANGLE_VERT: &[u8] = include_bytes!(env!("TRIANGLE_VERT"));
const TRIANGLE_FRAG: &[u8] = include_bytes!(env!("TRIANGLE_FRAG"));


pub struct RenderState {
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
    framebuffers: Vec<vk::Framebuffer>,



    command_pool: vk::CommandPool,
    command_buffers: [vk::CommandBuffer; MAX_FRAMES_IN_FLIGHT], 

    sync: FrameSync,

    world: RenderWorld,

    camera: Camera,
    projection: Projection,
    camera_controller: CameraController,
}


impl RenderState {
    pub fn new(
        window_handle: &WindowHandle,
        display_handle: &DisplayHandle,
        size: PhysicalSize<u32>,
    ) -> Self {
        let context = VkContext::new(window_handle, display_handle);
        let swapchain = Swapchain::new(&context, size.width, size.height);

        let render_pass = create_render_pass(&context, &swapchain);
        let (pipeline_layout, pipeline) =
            create_pipeline(&context, render_pass, TRIANGLE_VERT, TRIANGLE_FRAG);

        let (depth_image, depth_allocation, depth_view) =
            create_depth_resources(&context, context.allocator(), swapchain.extent);
        
        let framebuffers = create_framebuffers(&context, &swapchain, render_pass, depth_view);

        let command_pool = create_command_pool(&context);
        let command_buffers =
            allocate_command_buffers(&context, command_pool);

        let sync = FrameSync::new(context.device(), swapchain.images.len());

        let world = RenderWorld::new();

        let camera = Camera::new(glam::vec3(0.0, 150.0, 2.0), -90.0, 0.0);
        let projection = Projection::new(size.width, size.height, 45.0, 1.0, 10000.0);
        let camera_controller = CameraController::new(4.0, 1.0);

        Self {
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
            framebuffers,
            command_pool,
            command_buffers,
            sync,
            world,
            camera,
            projection,
            camera_controller,
        }
    }

    pub fn update_chunk(&self, pos: ChunkPos, chunk: &LocalChunk) {
        self.world.update_chunk(pos, chunk);
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
            let clear_values = [clear_color, vk::ClearValue{
               depth_stencil: vk::ClearDepthStencilValue{
                   depth: 1.0,
                   stencil: 0,
               }
            }];
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
    
    
            self.world.draw(device, cmd, self.pipeline, self.pipeline_layout, self.projection.calc_proj() * self.camera.calc_view());

            device.cmd_end_render_pass(cmd);
            device.end_command_buffer(cmd).unwrap();
        }
    
        let wait_semaphores = [self.sync.image_available[frame]];
        let signal_semaphores = [self.sync.render_finished[image_index as usize]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
    
    
        let submit_info = vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(std::slice::from_ref(&cmd))
            .signal_semaphores(&signal_semaphores)
            ;
    
        unsafe {
            device
                .queue_submit(
                    self.context.graphics_queue(),
                    &[submit_info],
                    self.sync.in_flight[frame],
                )
                .unwrap();
        }
    
        match self.swapchain.present(
            self.context.present_queue(),
            &self.sync,
            image_index,
        ) {
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

        self.context.device().queue_wait_idle(self.context.present_queue()).unwrap();
        self.context.device().queue_wait_idle(self.context.graphics_queue()).unwrap();
            for fb in &self.framebuffers {
                self.context.device().destroy_framebuffer(*fb, None);
            }
        }
        self.framebuffers.clear();
            self.swapchain
                .recreate(&self.context, self.width, self.height);

            unsafe{
        self.context.device().destroy_image_view(self.depth_view, None);
        self.context.allocator().destroy_image(self.depth_image, &mut self.depth_allocation);
            }

        let (depth_image, depth_allocation, depth_view) =
            create_depth_resources(&self.context, self.context.allocator(), self.swapchain.extent);

        self.depth_image = depth_image;
        self.depth_allocation = depth_allocation;
        self.depth_view = depth_view;

        self.framebuffers =
            create_framebuffers(&self.context, &self.swapchain, self.render_pass, self.depth_view);


            
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
            self.context.allocator().destroy_image(self.depth_image, &mut self.depth_allocation);

            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);

            device.destroy_render_pass(self.render_pass, None);

            device.destroy_command_pool(self.command_pool, None);
        }

        self.swapchain.destroy(device);
        self.sync.destroy(device);
    }
}



pub fn create_render_pass(ctx: &VkContext, swapchain: &Swapchain) -> vk::RenderPass {
    let color_attachment = vk::AttachmentDescription::default()
        .format(swapchain.format)
        .samples(vk::SampleCountFlags::TYPE_1)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

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

fn create_shader_module(device: &Device, code: &[u8]) -> vk::ShaderModule {
    let code_aligned = ash::util::read_spv(&mut std::io::Cursor::new(code)).unwrap();
    let info = vk::ShaderModuleCreateInfo::default().code(&code_aligned);
    unsafe { device.create_shader_module(&info, None).unwrap() }
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
        usage: MemoryUsage::GpuOnly,
        ..Default::default()
    };

    let (image, allocation) = unsafe {
        allocator.create_image(&image_info, &alloc_info)
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

    let depth_view = unsafe {
        ctx.device().create_image_view(&view_info, None).unwrap()
    };

    (image, allocation, depth_view)
}

pub fn create_pipeline(
    ctx: &VkContext,
    render_pass: vk::RenderPass,
    vert_spv: &[u8],
    frag_spv: &[u8],
) -> (vk::PipelineLayout, vk::Pipeline) {
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

    let binding_desc = [Vertex::binding_description()];
    let attribute_desc = Vertex::attribute_descriptions();
    
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
    let color_blending =
        vk::PipelineColorBlendStateCreateInfo::default().attachments(&attachments);

    let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let dynamic_state =
        vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);
    let push_constant_range = vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::VERTEX)
        .offset(0)
        .size(std::mem::size_of::<PushConstants>() as u32);

    let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default().push_constant_ranges(std::slice::from_ref(&push_constant_range));

    let pipeline_layout =
        unsafe { device.create_pipeline_layout(&pipeline_layout_info, None).unwrap() };

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

    (pipeline_layout, pipeline)
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

    unsafe { (device.fp_v1_0().allocate_command_buffers)(device.handle(), &alloc_info, buffers.as_mut_ptr()).result().unwrap() };

    buffers
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

