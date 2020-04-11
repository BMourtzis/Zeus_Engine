#[macro_use]
extern crate log;

extern crate zeus_core;

//NOTE: For now we only use Vulkan until we can undestand the layout and design a architecture
extern crate gfx_backend_vulkan as back;
extern crate gfx_hal;
extern crate glsl_to_spirv;
extern crate image as img;

mod adapter;
mod backend;
mod buffer;
mod camera;
mod constants;
mod desc;
mod device;
mod framebuffer;
mod image;
mod model;
mod obj;
mod pass;
mod pipeline;
mod renderer;
mod swapchain;

use winit::{
    dpi::LogicalSize,
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use self::{constants::DIMS, renderer::RendererState};

use zeus_core::input;

pub fn render() {
    info!("Starting up Zeus Engine V0.1.0");

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_min_inner_size(LogicalSize::new(1.0, 1.0))
        .with_inner_size(LogicalSize::new(DIMS.width, DIMS.height))
        .with_title("Zeus Engine V0.1.0".to_string());

    let backend = backend::create_backend(window_builder, &event_loop);

    let mut renderer_state = RendererState::new(backend);

    renderer_state.load_level();

    renderer_state.draw();

    event_loop.run(move |event, _, control_flow| {
        // *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    info!("Exiting Renderer!");
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::Resized(_dims) => {
                    debug!("Resizing window to {}x{}", _dims.width, _dims.height);
                    renderer_state.recreate_swapchain = true;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode,
                            state,
                            ..
                        },
                    ..
                } => {
                    if let Some(virtual_keycode) = virtual_keycode {
                        input::update_btn(virtual_keycode, state);
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                debug!("RedrawRequested");
                renderer_state.draw();
            }
            Event::MainEventsCleared => {
                renderer_state.backend.window.request_redraw();
                debug!("EventsCleared");
            }
            _ => (),
        }
    });
}


//region Tests
#[cfg(test)]
mod tests {
    #[test]
    fn simple_test() {
        let check = true;
        assert!(check);
    }
}

//endregion