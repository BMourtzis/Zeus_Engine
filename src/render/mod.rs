extern crate gfx_hal;

//NOTE: For now we only use Vulkan until we can undestand the layout and design a architecture
extern crate gfx_backend_vulkan as back;
extern crate image as img;
extern crate glsl_to_spirv;

mod adapter;
mod backend;
mod constants;
mod buffer;
mod desc;
mod device;
mod image;
mod pass;
mod pipeline;
mod renderer;
mod swapchain;
mod utils;

use winit::{
    event_loop:: {
        EventLoop,
        ControlFlow
    },
    window::WindowBuilder,
    dpi::LogicalSize,
    event::{
        Event,
        WindowEvent,
        KeyboardInput,
        VirtualKeyCode,
        ElementState
    }
};

use self::{
    constants::DIMS,
    renderer::RendererState
};


pub fn render() {
    #[cfg(debug_assertions)]
    env_logger::init();

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_min_inner_size(LogicalSize::new(1.0, 1.0))
        .with_inner_size(LogicalSize::new(
            DIMS.width,
            DIMS.height
        ))
        .with_title("Zeus Engine V0.1.0".to_string());
    
    let backend = backend::create_backend(window_builder, &event_loop);

    let mut renderer_state = unsafe {
        RendererState::new(backend)
    };

    renderer_state.draw();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } | WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    },
                    WindowEvent::Resized(_dims) => {
                        println!("Resize Event");
                        renderer_state.recreate_swapchain = true;
                    },
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            virtual_keycode,
                            state: ElementState::Pressed,
                            ..
                        },
                        ..
                    } => {
                        // println!("Keyboard Input");
                        if let Some(virtual_keycode) = virtual_keycode {
                            renderer_state.input(virtual_keycode);
                        }
                    },
                    _ => ()
                }
            },
            Event::RedrawRequested(_) => {
                println!("RedrawRequested");
                renderer_state.draw();
            },
            Event::MainEventsCleared => {
                renderer_state.backend.window.request_redraw();
                // println!("EventsCleared");
            },
            _ => ()
        }
    });

}
