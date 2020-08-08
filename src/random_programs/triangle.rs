use vk_playground::{
    utility,
    utility::constants::*,
    utility::debug::*,
    utility::share,
};

use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};

use std::ptr;

// constants
const WINDOW_TITLE: &'static str = "Basic Window";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const MAX_FRAMES_IN_FLIGHT: usize = 2;

struct SyncObjects {
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_sempahores: Vec<vk::Semaphore>,
    inflight_fences: Vec<vk::Fence>,
}

struct VulkanApp {
    window: winit::window::Window,

    _entry: ash::Entry,
    instance: ash::Instance,
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,

    _physical_device: vk::PhysicalDevice,
    device: ash::Device,

    graphics_queue: vk::Queue,
    present_queue: vk::Queue,

    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
    _swapchain_images: Vec<vk::Image>,
    _swapchain_format: vk::Format,
    _swapchain_extent: vk::Extent2D,
    swapchain_imageviews: Vec<vk::ImageView>,
    swapchain_framebuffers: Vec<vk::Framebuffer>,

    render_pass: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,

    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_sempahores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    current_frame: usize,
}

impl VulkanApp {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> VulkanApp {
        
        let window = utility::window::init_window(event_loop, WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);

        // init vulkan stuff
        let entry = ash::Entry::new().unwrap();

        let instance = share::CreateInstance
        
        VulkanApp {
            window,
            _entry: entry,

        }
    }
}

impl VulkanApp {

    pub fn main_loop(event_loop: EventLoop<()>) {

        event_loop.run(move |event, _, control_flow| {

            match event {
                // handle window events
                | Event::WindowEvent { event, .. } => {
                    // handle closing
                    match event {
                        | WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        },
                        // handle keyboard input
                        | WindowEvent::KeyboardInput { input, .. } => {
                            // handle different key cases
                            match input {
                                // various keys from the keyboard
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    // someone's pressed escape!
                                    match (virtual_keycode, state) {
                                        | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                            dbg!();
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                            // end of handling key cases
                        }, // end of handling window events
                        | _ => {},
                    }
                },
                _ => (),
            }

        })
    }

}

fn main() {

    let event_loop = EventLoop::new();    
    let vulkan_app= VulkanApp::new(&event_loop);

    VulkanApp::main_loop(event_loop);
}
