use ash::{Device, vk};

use crate::renderer::{mesh::Vertex, render_world::PushConstants, vulkan::context::VkContext};

fn create_shader_module(device: &Device, code: &[u8]) -> vk::ShaderModule {
    let code_aligned = ash::util::read_spv(&mut std::io::Cursor::new(code)).unwrap();
    let info = vk::ShaderModuleCreateInfo::default().code(&code_aligned);
    unsafe { device.create_shader_module(&info, None).unwrap() }
}

pub fn create_pipeline_layout(
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

pub fn create_pipeline_with_mode(
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

pub fn create_pipeline(
    ctx: &VkContext,
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    vert_spv: &[u8],
    frag_spv: &[u8],
) -> vk::Pipeline {
    create_pipeline_with_mode(
        ctx,
        render_pass,
        pipeline_layout,
        vert_spv,
        frag_spv,
        vk::PolygonMode::FILL,
    )
}

pub fn create_wireframe_pipeline(
    ctx: &VkContext,
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    vert_spv: &[u8],
    frag_spv: &[u8],
) -> Option<vk::Pipeline> {
    if ctx.features().fill_mode_non_solid {
        Some(create_pipeline_with_mode(
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

pub fn create_egui_pipeline(
    device: &Device,
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
) -> anyhow::Result<vk::Pipeline> {
    // Vertex input state for egui vertices
    let binding_desc = vk::VertexInputBindingDescription::default()
        .binding(0)
        .stride(std::mem::size_of::<egui::epaint::Vertex>() as u32)
        .input_rate(vk::VertexInputRate::VERTEX);

    let attribute_descs = [
        // Position
        vk::VertexInputAttributeDescription::default()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(0),
        // UV
        vk::VertexInputAttributeDescription::default()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(8),
        // Color
        vk::VertexInputAttributeDescription::default()
            .binding(0)
            .location(2)
            .format(vk::Format::R8G8B8A8_UNORM)
            .offset(16),
    ];

    let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::default()
        .vertex_binding_descriptions(std::slice::from_ref(&binding_desc))
        .vertex_attribute_descriptions(&attribute_descs);

    let input_assembly_info = vk::PipelineInputAssemblyStateCreateInfo::default()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false);

    let viewport_state_info = vk::PipelineViewportStateCreateInfo::default()
        .viewport_count(1)
        .scissor_count(1);

    let rasterizer_info = vk::PipelineRasterizationStateCreateInfo::default()
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::NONE)
        .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
        .line_width(1.0);

    let multisampling_info = vk::PipelineMultisampleStateCreateInfo::default()
        .rasterization_samples(vk::SampleCountFlags::TYPE_1);

    let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
        .color_write_mask(
            vk::ColorComponentFlags::R
                | vk::ColorComponentFlags::G
                | vk::ColorComponentFlags::B
                | vk::ColorComponentFlags::A,
        )
        .blend_enable(true)
        .src_color_blend_factor(vk::BlendFactor::ONE)
        .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
        .color_blend_op(vk::BlendOp::ADD)
        .src_alpha_blend_factor(vk::BlendFactor::ONE_MINUS_DST_ALPHA)
        .dst_alpha_blend_factor(vk::BlendFactor::ONE)
        .alpha_blend_op(vk::BlendOp::ADD);

    let color_blending_info = vk::PipelineColorBlendStateCreateInfo::default()
        .attachments(std::slice::from_ref(&color_blend_attachment));

    let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let dynamic_state_info =
        vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);

    // Load egui shaders
    let vert_spv = include_bytes!(env!("EGUI_VERT"));
    let frag_spv = include_bytes!(env!("EGUI_FRAG"));

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

    let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
        .stages(&shader_stages)
        .vertex_input_state(&vertex_input_info)
        .input_assembly_state(&input_assembly_info)
        .viewport_state(&viewport_state_info)
        .rasterization_state(&rasterizer_info)
        .multisample_state(&multisampling_info)
        .color_blend_state(&color_blending_info)
        .dynamic_state(&dynamic_state_info)
        .layout(pipeline_layout)
        .render_pass(render_pass)
        .subpass(0);

    let pipelines = unsafe {
        device
            .create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
            .map_err(|e| anyhow::anyhow!("Failed to create egui pipeline: {:?}", e))?
    };

    // Clean up shader modules
    unsafe {
        device.destroy_shader_module(vert_module, None);
        device.destroy_shader_module(frag_module, None);
    }

    Ok(pipelines[0])
}
