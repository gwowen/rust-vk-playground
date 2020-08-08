use ash::version::DeviceV1_0;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;

use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

use crate::utility::constants::*;
use crate::utility::debug;
use crate::utility::structures::*;
use crate::utility::platforms;

pub fn create_instance(
    entry: &ash::Entry,
    window_title: &str,
    is_enable_debug: bool,
    required_validation_layers: &Vec<&str>,
) -> ash::Instance {
    if is_enable_debug
        && debug::check_validation_layer_support(entry, required_validation_layers) == false
    {
        panic!("Validation layers requested, but not available!");
    }

    let app_name = CString::new(window_title).unwrap();
    let engine_name = CString::new("Vulkan Engine").unwrap();
    let app_info = vk::ApplicationInfo {
        p_application_name: app_name.as_ptr(),
        s_type: vk::StructureType::APPLICATION_INFO,
        p_next: ptr::null(),
        application_version: APPLICATION_VERSION,
        p_engine_name: engine_name.as_ptr(),
        engine_version: ENGINE_VERSION,
        api_version: API_VERSION,
    };

    // this creates info used to debug issues in vk::createInstance
    let debug_utils_create_info = debug::populate_debug_messenger_create_info();

    // VK_EXT debug report has been requested here
    let extension_names = platforms::required_extension_names();

    let required_validation_layer_raw_names: Vec<CString> = required_validation_layers
        .iter()
        .map(|layer_name| CString::new(*layer_name).unwrap())
        .collect();
    let layer_names: Vec<*const i8> = required_validation_layer_raw_names
        .iter()
        .map(|layer_name| layer_name.as_ptr())
        .collect(); 

    // fill out instance info
    let create_info = vk::InstanceCreateInfo {
        s_type: vk::StructureType::INSTANCE_CREATE_INFO,
        p_next: if VALIDATION.is_enable {
            &debug_utils_create_info as *const vk::DebugUtilsMessengerCreateInfoEXT
                    as *const c_void
        } else {
            ptr::null()
        },
        flags: vk::InstanceCreateFlags::empty(),
        p_application_info: &app_info,
        pp_enabled_layer_names: if is_enable_debug {
            layer_names.as_ptr()
        } else {
            ptr::null()
        },
        enabled_layer_count: if is_enable_debug {
            layer_names.len()
        } else {
            0
        } as u32,
        pp_enabled_extension_names: extension_names.as_ptr(),
        enabled_extension_count: extension_names.len() as u32,
    };

    let instance: ash::Instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("Failed to create instance!")
    };

    instance
}

pub fn create_surface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    window: &winit::window::Window,
    screen_width: u32,
    screen_height: u32,
)   -> SurfaceStuff {

    let surface = unsafe {
        platforms::create_surface(entry, instance, window).expect("Failed to create surface!");
    };

    let surface_loader = ash::extensions::khr::Surface::new(entry, instance);

    SurfaceStuff {
        surface_loader,
        surface,
        screen_width,
        screen_height,
    }
}


pub fn pick_physical_device(
    instance: &ash::Instance,
    surface_stuff: &SurfaceStuff,
    required_device_extensions: &DeviceExtension
) -> vk::PhysicalDevice {
    let physical_devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate Physical Devices!")
    };

    let result = physical_devices.iter().find(|physical_device| {
        let is_suitable = is_physical_device_suitable(
            instance,
            **physical_device,
            surface_stuff,
            required_device_extensions,
        );

        is_suitable
    });

    match result {
        Some(p_physical_device) => *p_physical_device,
        None => panic!("Failed to find a suitable GPU"),
    }
}

pub fn is_physical_device_suitable(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    surface_stuff: &SurfaceStuff,
    required_device_extensions: &DeviceExtension,
) -> bool {

    let device_features = unsafe {instance.get_physical_device_features(physical_device)};

    let indices = find_queue_family(instance, physical_device, surface_stuff);

    let is_queue_family_supported = indices.is_complete();

    let is_device_extension_supported = 
        check_device_extension_support(instance, physical_device, required_device_extensions);

    let is_swapchain_supported = if is_device_extension_supported {
        let swapchain_support = query_swapchain_support(physical_device, surface_stuff);
        !swapchain_support.formats.is_empty() && !swapchain_support.present_modes.is_empty()
    } else {
        false
    };

    let is_support_sampler_anisotropy = device_features.sampler_anisotropy == 1;

    return is_queue_family_supported
        && is_device_extension_supported
        && is_swapchain_supported
        && is_support_sampler_anisotropy;
    
}

pub fn create_logical_device(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    validation: &super::debug::ValidationInfo,
    device_extensions: &DeviceExtension,
    surface_stuff: &SurfaceStuff,
) -> (ash::Device, QueueFamilyIndices) {
    let indices = find_queue_family(instance, physical_device, surface_stuff);

    use std::collections::HashSet;
    let mut unique_queue_families = HashSet::new();
    unique_queue_families.insert(indices.graphics_family.unwrap());
    unique_queue_families.insert(indices.present_family.unwrap());

    let queue_priorities = [1.0_f32];
    let mut queue_create_infos = vec![];
    for &queue_family in unique_queue_families.iter() {
        let queue_create_info = vk::DeviceQueueCreateInfo {
            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DeviceQueueCreateFlags::empty(),
            queue_family_index: queue_family,
            p_queue_priorities: queue_priorities.as_ptr(),
            queue_count: queue_priorities.len() as u32,
        };
        queue_create_infos.push(queue_create_info);
    }

    let physical_device_features = vk::PhysicalDeviceFeatures {
        sampler_anisotropy: vk::TRUE,
        ..Default::default()
    };

    let required_validation_layer_raw_names: Vec<CString> = validation
        .required_validation_layers
        .iter()
        .map(|layer_name| CString::new(*layer_name).unwrap())
        .collect();
    let enable_layer_names: Vec<*const c_char> = required_validation_layer_raw_names
        .iter()
        .map(|layer_name| layer_name.as_ptr())
        .collect();
    
    let enable_extension_names = device_extensions.get_extensions_raw_names();

    let device_create_info = vk::DeviceCreateInfo {
        s_type: vk::StructureType::DEVICE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::DeviceCreateFlags::empty(),
        queue_create_info_count: queue_create_infos.len() as u32,
        p_queue_create_infos: queue_create_infos.as_ptr(),
        enabled_layer_count: if validation.is_enable {
            enable_layer_names.len()
        } else {
            0
        } as u32,
        pp_enabled_layer_names: if validation.is_enable {
            enable_layer_names.as_ptr()
        } else {
            ptr::null()
        },
        enabled_extension_count: enable_extension_names.len() as u32,
        pp_enabled_extension_names: enable_extension_names.as_ptr(),
        p_enabled_features: &physical_device_features,
    };

    let device: ash::Device = unsafe {
        instance
            .create_device((physical_device, &device_create_info, None)
            .expect("Failed to create logical device!")
    };

    (device, indices)
}


pub fn find_queue_family(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    surface_stuff: &SurfaceStuff
) -> QueueFamilyIndices {

}

pub fn check_device_extension_support(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    device_extensions: &DeviceExtension,
) -> bool {

}

pub fn query_swapchain_support(
    physical_device: vk::PhysicalDevice,
    surface_stuff: &SurfaceStuff
) -> SwapChainSupportDetail {

}

pub fn create_swapchain(
    instance: &ash::Instance,
    device: &ash::Device,
    physical_device: vk::PhysicalDevice,
    window: &winit::window::Window,
    queue_family: &QueueFamilyIndices,
) -> SwapChainStuff {

}

pub fn choose_swapchain_format(
    available_formats: &Vec<vk::SurfaceFormatKHR>,
) -> vk::SurfaceFormatKHR {

}

pub fn choose_swapchain_present_mode(
    available_present_modes: &Vec<vk::PresentModeKHR>,
) -> vk::PresentModeKHR {

}

pub fn choose_swapchain_extent(
    capabilities: &vk::SurfaceCapabilitiesKHR,
    window: &winit::window::Window,
) -> vk::Extent2D {

}

pub fn create_shader_module(device: &ash::Device, code: Vec<u8>) -> vk::ShaderModule {

}

pub fn create_buffer(
    device: &ash::Device,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    required_memory_properties: vk::MemoryPropertyFlags,
    device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
) -> (vk::Buffer, vk::DeviceMemory) {

}

pub fn copy_buffer(
    device: &ash::Device,
    submit_queue: vk::Queue,
    command_pool: vk::CommandPool,
    src_buffer: vk::Buffer,
    dst_buffer: vk::Buffer,
    size: vk::DeviceSize,
) {

}
pub fn begin_single_time_command(
    device: &ash::Device,
    command_pool: vk::CommandPool,
) -> vkCommandBuffer {

}

pub fn end_single_time_command(
    device: &ash::Device,
    command_pool: vk::CommandPool,
    submit_queue: vk::Queue,
    command_buffer: vk::CommandBuffer,
) {

}

pub fn find_memory_type(
    type_filter: u32,
    required_properties: vk::MemoryPropertyFlags,
    mem_properties: &vk::PhysicalDevice
) -> u32 {

}

pub fn has_stencil_component(format: vk::Format) -> bool {

}

pub fn copy_buffer_to_image(
    device: &ash::Device,
    command_pool: vk::CommandPool,
    submit_queue: vk::Queue,
    buffer: vk::Buffer,
    image: vk::Image,
    width: u32,
    height: u32,
) {

}

pub fn find_depth_format(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> vk::Format {

}

pub fn find_supported_format(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    candidate_formats: &[vk::Format],
    tiling: vk::ImageTiling,
    features: vk::FormatFeatureFlags,
) -> vk::Format {

}

pub fn load_model(model_path: &Path) -> (Vec<VertexV3>, Vec<u32>) {

}

pub fn check_mipmap_support(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    image_format: vk::Format,
) {

}