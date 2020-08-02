use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};


// constants
const IS_PAINT_FPS_COUNTER: bool = true;

pub fn init_window(
    event_loop: &EventLoop<()>,
    title: &str,
    width: u32,
    height: u32

) -> winit::window::Window {
    winit::window::WindowBuilder::new() 
    .with_title(WINDOW_TITLE)
    .with_inner_size(winit::dpi::LogicalSize::new(width, height))
    .build(event_loop)
    .expect("Failed to create window!")
}

pub struct ProgramProc {
    pub event_loop: EventLoop<()>,
}

impl ProgramProc {

    pub fn new() -> ProgramProc {
        // init the window
        let event_loop = EventLoop::new();

        ProgramProc { event_loop }
    }
    
    pub fn main_loop(event_loop: EventLoop<()>) {

        let mut tick_counter = super::fps_limiter::FPSLimiter::new();

        event_loop.run(move |event, _, control_flow| {

            match event {
                // handle window events
                | Event::WindowEvent { event, .. } => {
                    // handle closing
                    match event {
                        | WindowEvent::CloseRequested => {
                            vulkan_app.wait_device_idle();
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
                                            vulkan_app.wait_device_idle();
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                            // end of handling key cases
                        }, 
                         | WindowEvent::Resized(_new_size) = {
                            vulkan_app.wait_device_idle();
                            vulkan_app.resize_framebuffer();
                         },
                        | _ => {},
                    }
                },
                | Event::MainEventsCleared => {
                    vulkan_app.window_ref().request_redraw();
                },
                | Event::RedrawRequested(_window_id) => {
                    let delta+time = tick_counter.delta_time();
                    vulkan_app.draw_frame(delta_time);

                    is IS_PAINT_FPS_COUNTER {
                        print!("FPS: {}\r", tick_counter.fps());
                    }

                    tick_counter.tick_frame();
                },
                | Event::LoopDestroyed => {
                    vulkan_app.wait_device_idle();
                }
                _ => (),
            }
        })
    }

}