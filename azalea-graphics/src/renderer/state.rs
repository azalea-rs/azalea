use crate::renderer::mesh::{Mesh, Vertex};
use crate::vulkan::frame_sync::MAX_FRAMES_IN_FLIGHT;
use crate::vulkan::{context::VkContext, frame_sync::FrameSync};
use crate::vulkan::swapchain::Swapchain;
use ash::vk::{self, Handle};
use ash::Device;
use raw_window_handle::{DisplayHandle, WindowHandle};
use vk_mem::{Alloc, Allocation, AllocationCreateFlags, AllocationCreateInfo, Allocator, MemoryUsage};
use winit::dpi::PhysicalSize;

const TRIANGLE_VERT: &[u8] = include_bytes!(env!("TRIANGLE_VERT"));
const TRIANGLE_FRAG: &[u8] = include_bytes!(env!("TRIANGLE_FRAG"));

const VERTICES: [Vertex; 3] = [
    Vertex { pos: [ 0.0, -0.5], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [ 0.5,  0.5], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [-0.5,  0.5], color: [0.0, 0.0, 1.0] },
];

const INDICES: [u32; 3] = [0, 1, 2];

pub struct RenderState {
    pub context: VkContext,
    pub swapchain: Swapchain,
    should_recreate: bool,
    width: u32,
    height: u32,

    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    framebuffers: Vec<vk::Framebuffer>,

    command_pool: vk::CommandPool,
    command_buffers: [vk::CommandBuffer; MAX_FRAMES_IN_FLIGHT], 

    sync: FrameSync,

    mesh: Mesh 
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

        let framebuffers = create_framebuffers(&context, &swapchain, render_pass);

        let command_pool = create_command_pool(&context);
        let command_buffers =
            allocate_command_buffers(&context, command_pool);

        let sync = FrameSync::new(context.device(), swapchain.images.len());

        let mesh = Mesh::new(&context, &VERTICES, &INDICES);

        Self {
            context,
            swapchain,
            should_recreate: false,
            width: size.width,
            height: size.height,
            render_pass,
            pipeline_layout,
            pipeline,
            framebuffers,
            command_pool,
            command_buffers,
            sync,
            mesh
        }
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
        let rp_info = vk::RenderPassBeginInfo::default()
            .render_pass(self.render_pass)
            .framebuffer(self.framebuffers[image_index as usize])
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.swapchain.extent,
            })
            .clear_values(std::slice::from_ref(&clear_color));

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


        device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline);

        let vertex_buffers = [self.mesh.buffer];
        let vertex_offsets = [self.mesh.vertex_offset];
        device.cmd_bind_vertex_buffers(cmd, 0, &vertex_buffers, &vertex_offsets);
        
        device.cmd_bind_index_buffer(
            cmd,
            self.mesh.buffer,
            self.mesh.index_offset,
            vk::IndexType::UINT32,
        );
        
        device.cmd_draw_indexed(cmd, self.mesh.index_count, 1, 0, 0, 0);

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
    pub fn mark_recreate(&mut self, size: PhysicalSize<u32>) {
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
        self.framebuffers =
            create_framebuffers(&self.context, &self.swapchain, self.render_pass);
            
            self.should_recreate = false;
        }
    }

    pub fn destroy(&mut self) {
        let device = self.context.device();


        unsafe {
            device.device_wait_idle().unwrap();

        self.mesh.destroy(&self.context);

            for fb in &self.framebuffers {
                device.destroy_framebuffer(*fb, None);
            }
            self.framebuffers.clear();

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
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::PRESENT_SRC_KHR); 

    let color_attachment_ref = vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    };

    let subpass = vk::SubpassDescription::default()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(std::slice::from_ref(&color_attachment_ref));

    let dependency = vk::SubpassDependency::default()
        .src_subpass(vk::SUBPASS_EXTERNAL)
        .dst_subpass(0)
        .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .src_access_mask(vk::AccessFlags::empty())
        .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);

    let render_pass_info = vk::RenderPassCreateInfo::default()
        .attachments(std::slice::from_ref(&color_attachment))
        .subpasses(std::slice::from_ref(&subpass))
        .dependencies(std::slice::from_ref(&dependency));

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
        .front_face(vk::FrontFace::CLOCKWISE)
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

    let attachments = [color_blend_attachment];
    let color_blending =
        vk::PipelineColorBlendStateCreateInfo::default().attachments(&attachments);

    let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let dynamic_state =
        vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);

    let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default();
    let pipeline_layout =
        unsafe { device.create_pipeline_layout(&pipeline_layout_info, None).unwrap() };

    let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
        .stages(&shader_stages)
        .vertex_input_state(&vertex_input)
        .input_assembly_state(&input_assembly)
        .viewport_state(&viewport_state)
        .rasterization_state(&rasterizer)
        .multisample_state(&multisampling)
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
) -> Vec<vk::Framebuffer> {
    let device = ctx.device();
    let mut framebuffers = Vec::with_capacity(swapchain.image_views.len());

    for &view in &swapchain.image_views {
        let attachments = [view];

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

