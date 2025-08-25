use std::{
    ffi::{CStr, CString},
    mem::ManuallyDrop,
    os::raw::{c_char, c_void},
};

use ash::{
    Device, Entry, Instance,
    ext::debug_utils,
    khr::{surface, swapchain as khr_swapchain},
    vk,
};
use raw_window_handle::{DisplayHandle, WindowHandle};
use vk_mem::{Allocator, AllocatorCreateInfo};

#[derive(Clone, Copy)]
pub struct QueueFamiliesIndices {
    pub graphics_index: u32,
    pub present_index: u32,
}

pub struct VkContext {
    _entry: Entry,
    instance: Instance,
    debug_messenger: Option<(debug_utils::Instance, vk::DebugUtilsMessengerEXT)>,
    surface: surface::Instance,
    surface_khr: vk::SurfaceKHR,

    physical_device: vk::PhysicalDevice,
    device: Device,
    allocator: ManuallyDrop<Allocator>,

    queue_families: QueueFamiliesIndices,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
    command_pool: vk::CommandPool,
}

impl VkContext {
    pub fn new(window: &WindowHandle, display: &DisplayHandle) -> Self {
        let entry = unsafe { Entry::load().expect("Failed to load Vulkan entry.") };
        let instance = Self::create_instance(&entry, display);
        let surface = surface::Instance::new(&entry, &instance);
        let surface_khr = unsafe {
            ash_window::create_surface(&entry, &instance, display.as_raw(), window.as_raw(), None)
                .expect("Failed to create surface.")
        };
        let debug_messenger = setup_debug_messenger(&entry, &instance);

        let (physical_device, queue_families) =
            Self::pick_physical_device(&instance, &surface, surface_khr);
        let (device, graphics_queue, present_queue) =
            Self::create_logical_device(&instance, physical_device, queue_families);

        let allocator = ManuallyDrop::new(unsafe {
            Allocator::new(AllocatorCreateInfo::new(
                &instance,
                &device,
                physical_device,
            ))
            .expect("Failed to create VMA allocator.")
        });

        let command_pool = unsafe {
            device.create_command_pool(
                &vk::CommandPoolCreateInfo::default()
                    .queue_family_index(queue_families.graphics_index)
                    .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER),
                None,
            )
        }
        .expect("Failed to create command pool.");

        Self {
            _entry: entry,
            instance,
            debug_messenger,
            surface,
            surface_khr,
            physical_device,
            device,
            allocator,
            queue_families,
            graphics_queue,
            present_queue,
            command_pool,
        }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }
    pub fn allocator(&self) -> &Allocator {
        &self.allocator
    }
    pub fn instance(&self) -> &Instance {
        &self.instance
    }
    pub fn surface(&self) -> &surface::Instance {
        &self.surface
    }
    pub fn surface_khr(&self) -> vk::SurfaceKHR {
        self.surface_khr
    }
    pub fn physical_device(&self) -> vk::PhysicalDevice {
        self.physical_device
    }
    pub fn graphics_queue(&self) -> vk::Queue {
        self.graphics_queue
    }
    pub fn present_queue(&self) -> vk::Queue {
        self.present_queue
    }
    pub fn queue_families(&self) -> QueueFamiliesIndices {
        self.queue_families
    }

    pub fn begin_one_time_commands(&self) -> vk::CommandBuffer {
        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(self.command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let cmd_buf = unsafe { self.device().allocate_command_buffers(&alloc_info).unwrap()[0] };

        let begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe {
            self.device
                .begin_command_buffer(cmd_buf, &begin_info)
                .unwrap();
        }

        cmd_buf
    }

    pub fn end_one_time_commands(&self, cmd_buf: vk::CommandBuffer) {
        unsafe {
            self.device.end_command_buffer(cmd_buf).unwrap();

            let submit_info =
                vk::SubmitInfo::default().command_buffers(std::slice::from_ref(&cmd_buf));
            self.device
                .queue_submit(self.graphics_queue(), &[submit_info], vk::Fence::null())
                .unwrap();
            self.device.queue_wait_idle(self.graphics_queue()).unwrap();

            self.device
                .free_command_buffers(self.command_pool, &[cmd_buf]);
        }
    }

    fn create_instance(entry: &Entry, display: &DisplayHandle) -> Instance {
        let app_name = CString::new("Azalea Renderer").unwrap();
        let engine_name = CString::new("Custom").unwrap();

        let app_info = vk::ApplicationInfo::default()
            .application_name(app_name.as_c_str())
            .application_version(vk::make_api_version(0, 0, 1, 0))
            .engine_name(engine_name.as_c_str())
            .engine_version(vk::make_api_version(0, 0, 1, 0))
            .api_version(vk::make_api_version(0, 1, 3, 0));

        let mut extensions = ash_window::enumerate_required_extensions(display.as_raw())
            .unwrap()
            .to_vec();
        if cfg!(debug_assertions) {
            extensions.push(debug_utils::NAME.as_ptr());
        }

        let (_layer_names, layer_ptrs) = get_layer_names_and_pointers();
        let mut create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extensions);

        if cfg!(debug_assertions) {
            check_validation_layer_support(entry);
            create_info = create_info.enabled_layer_names(&layer_ptrs);
        }

        unsafe { entry.create_instance(&create_info, None).unwrap() }
    }

    fn pick_physical_device(
        instance: &Instance,
        surface: &surface::Instance,
        surface_khr: vk::SurfaceKHR,
    ) -> (vk::PhysicalDevice, QueueFamiliesIndices) {
        let devices =
            unsafe { instance.enumerate_physical_devices() }.expect("Failed to enumerate devices.");
        let device = devices
            .into_iter()
            .find(|&dev| {
                let (gfx, pres) = Self::find_queue_families(instance, surface, surface_khr, dev);
                gfx.is_some() && pres.is_some()
            })
            .expect("No suitable GPU found.");

        let (graphics, present) = Self::find_queue_families(instance, surface, surface_khr, device);
        let indices = QueueFamiliesIndices {
            graphics_index: graphics.unwrap(),
            present_index: present.unwrap(),
        };

        (device, indices)
    }

    fn find_queue_families(
        instance: &Instance,
        surface: &surface::Instance,
        surface_khr: vk::SurfaceKHR,
        device: vk::PhysicalDevice,
    ) -> (Option<u32>, Option<u32>) {
        let mut graphics = None;
        let mut present = None;

        let families = unsafe { instance.get_physical_device_queue_family_properties(device) };
        for (i, fam) in families.iter().enumerate() {
            let idx = i as u32;
            if fam.queue_flags.contains(vk::QueueFlags::GRAPHICS) && graphics.is_none() {
                graphics = Some(idx);
            }
            let supports_present = unsafe {
                surface
                    .get_physical_device_surface_support(device, idx, surface_khr)
                    .unwrap()
            };
            if supports_present && present.is_none() {
                present = Some(idx);
            }
        }
        (graphics, present)
    }

    fn create_logical_device(
        instance: &Instance,
        physical: vk::PhysicalDevice,
        families: QueueFamiliesIndices,
    ) -> (Device, vk::Queue, vk::Queue) {
        let priorities = [1.0f32];
        let mut unique_indices = vec![families.graphics_index, families.present_index];
        unique_indices.dedup();

        let queue_infos: Vec<_> = unique_indices
            .iter()
            .map(|&idx| {
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(idx)
                    .queue_priorities(&priorities)
            })
            .collect();

        // --- Query descriptor indexing support ---
        let mut descriptor_indexing_features =
            vk::PhysicalDeviceDescriptorIndexingFeatures::default();

        let mut features2 =
            vk::PhysicalDeviceFeatures2::default().push_next(&mut descriptor_indexing_features);

        unsafe { instance.get_physical_device_features2(physical, &mut features2) };

        if descriptor_indexing_features.shader_sampled_image_array_non_uniform_indexing == vk::TRUE
        {
            log::info!("Descriptor indexing supported, enabling non-uniform indexing");
        } else {
            panic!("Device does not support descriptor indexing (required for texture arrays)");
        }

        let extensions = [khr_swapchain::NAME.as_ptr()];

        let create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_infos)
            .enabled_extension_names(&extensions)
            .push_next(&mut descriptor_indexing_features);

        let device = unsafe {
            instance
                .create_device(physical, &create_info, None)
                .expect("Failed to create logical device.")
        };
        let graphics_queue = unsafe { device.get_device_queue(families.graphics_index, 0) };
        let present_queue = unsafe { device.get_device_queue(families.present_index, 0) };

        (device, graphics_queue, present_queue)
    }
}

impl Drop for VkContext {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_command_pool(self.command_pool, None);
            ManuallyDrop::drop(&mut self.allocator);
            self.device.destroy_device(None);
            self.surface.destroy_surface(self.surface_khr, None);
            if let Some((utils, messenger)) = self.debug_messenger.take() {
                utils.destroy_debug_utils_messenger(messenger, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}

#[cfg(debug_assertions)]
pub const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_VALIDATION_LAYERS: bool = false;

const REQUIRED_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

unsafe extern "system" fn vulkan_debug_callback(
    flag: vk::DebugUtilsMessageSeverityFlagsEXT,
    typ: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    use vk::DebugUtilsMessageSeverityFlagsEXT as Flag;

    let message = unsafe { CStr::from_ptr((*p_callback_data).p_message) };
    match flag {
        Flag::VERBOSE => log::debug!("{:?} - {:?}", typ, message),
        Flag::INFO => log::info!("{:?} - {:?}", typ, message),
        Flag::WARNING => log::warn!("{:?} - {:?}", typ, message),
        _ => log::error!("{:?} - {:?}", typ, message),
    }
    vk::FALSE
}

/// Get the pointers to the validation layers names.
/// Also return the corresponding `CString` to avoid dangling pointers.
pub fn get_layer_names_and_pointers() -> (Vec<CString>, Vec<*const c_char>) {
    let layer_names = REQUIRED_LAYERS
        .iter()
        .map(|name| CString::new(*name).unwrap())
        .collect::<Vec<_>>();
    let layer_names_ptrs = layer_names
        .iter()
        .map(|name| name.as_ptr())
        .collect::<Vec<_>>();
    (layer_names, layer_names_ptrs)
}

/// Check if the required validation set in `REQUIRED_LAYERS`
/// are supported by the Vulkan instance.
///
/// # Panics
///
/// Panic if at least one on the layer is not supported.
pub fn check_validation_layer_support(entry: &Entry) {
    let supported_layers = unsafe { entry.enumerate_instance_layer_properties().unwrap() };
    for required in REQUIRED_LAYERS.iter() {
        let found = supported_layers.iter().any(|layer| {
            let name = unsafe { CStr::from_ptr(layer.layer_name.as_ptr()) };
            let name = name.to_str().expect("Failed to get layer name pointer");
            required == &name
        });

        if !found {
            panic!("Validation layer not supported: {}", required);
        }
    }
}

/// Setup the debug message if validation layers are enabled.
pub fn setup_debug_messenger(
    entry: &Entry,
    instance: &Instance,
) -> Option<(debug_utils::Instance, vk::DebugUtilsMessengerEXT)> {
    if !ENABLE_VALIDATION_LAYERS {
        return None;
    }

    let create_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
        .flags(vk::DebugUtilsMessengerCreateFlagsEXT::empty())
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(vulkan_debug_callback));
    let debug_utils = debug_utils::Instance::new(entry, instance);
    let debug_utils_messenger = unsafe {
        debug_utils
            .create_debug_utils_messenger(&create_info, None)
            .unwrap()
    };

    Some((debug_utils, debug_utils_messenger))
}
