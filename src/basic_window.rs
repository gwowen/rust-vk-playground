use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};


// constants
const WINDOW_TITLE: &'static str = "Basic Window";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

struct VulkanApp;

impl VulkanApp {
    fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window {
        winit::window::WindowBuilder::new() 
        .with_title(WINDOW_TITLE)
        .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH,WINDOW_HEIGHT))
        .build(event_loop)
        .expect("Failed to create window!")
    }

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
    let _window = VulkanApp::init_window(&event_loop);

    VulkanApp::main_loop(event_loop);
}
