use vk_playground::{
    utility,
    utility::constants::*,
    utility::debug::*,
    utility::share,
    utility::structures::*,
    utility::window::{ProgramProc,},
};

use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use cgmath::{Deg, Matrix4, Point3, Vector3};
use memoffset::offset_of;


use std::ffi::CString;
use std::path::Path;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VertexV2 {
    pub pos: [f32; 2],
    pub color: [f32; 3],
    pub tex_coord: [f32; 2],
}

impl VertexV2 {
    pub fn get_binding_description() -> [vk::VertexInputBindingDescription; 1] {
        [vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<VertexV2>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }]
    }

    pub fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 3] {
        [
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 0, 
                format: vk::Format::R32G32_SFLOAT,
                offset: offset_of!(VertexV2, pos) as u32,
            },

            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 1, 
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(VertexV2, color) as u32,
            },

            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 2, 
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(VertexV2, tex_coord) as u32,
            },
        ]
    }
}

// constants
const WINDOW_TITLE: &'static str = "Texture Mapping!";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

struct VulkanAppTextureMapping {
    window: winit::window::Window,

    _entry: ash::Entry,
    instance: ash::Instance,
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,

    physical_device: vk::PhysicalDevice,
    device: ash::Device,

    queue_family: QueueFamilyIndices,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,

    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<vk::Image>,
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    swapchain_imageviews: Vec<vk::ImageView>,
    swapchain_framebuffers: Vec<vk::Framebuffer>,

    render_pass: vk::RenderPass,
    ubo_layout: vk::DescriptorSetLayout,
    pipeline_layout: vk::PipelineLayout,
    graphics_pipeline: vk::Pipeline,

    texture_image: vk::Image,
    texture_image_view: vk::ImageView,
    texture_sampler: vk::Sampler,
    texture_image_memory: vk::DeviceMemory,

    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    index_buffer: vk::Buffer,
    index_buffer_memory: vk::DeviceMemory,

    uniform_transform: UniformBufferObject,
    uniform_buffers: Vec<vk::Buffer>,
    uniform_buffers_memory: Vec<vk::DeviceMemory>,

    desciptor_pool: vk::DescriptorPool,
    descriptor_sets: Vec<vk::DescriptorSet>,

    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,

    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_sempahores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    current_frame: usize,

    is_framebuffer_resized: bool,
}

impl VulkanAppTextureMapping {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> VulkanAppTextureMapping {
       let window =
            utility::window::init_window(&event_loop, WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);
            
            VulkanAppTextureMapping {
                window,
            }
    }

}

fn main() {

    let program_proc = ProgramProc::new();   
    let vulkan_app = VulkanAppTextureMapping::new(&program_proc.event_loop);

    program_proc.main_loop(vulkan_app);
}