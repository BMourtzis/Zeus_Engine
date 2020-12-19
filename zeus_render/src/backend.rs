extern crate gfx_backend_vulkan as back;

use super::adapter::AdapterState;
use gfx_hal::{Backend, Instance};
use std::{mem::ManuallyDrop, ptr};
use winit::{dpi::PhysicalPosition, window::Window};

pub struct BackendState<B: Backend> {
    instance: Option<B::Instance>,
    pub surface: ManuallyDrop<B::Surface>,
    pub adapter: AdapterState<B>,
    #[allow(dead_code)]
    pub window: Window,
}

impl<B: Backend> Drop for BackendState<B> {
    fn drop(&mut self) {
        if let Some(instance) = &self.instance {
            unsafe {
                let surface = ManuallyDrop::into_inner(ptr::read(&self.surface));
                instance.destroy_surface(surface);
            }
        }
    }
}

pub fn create_backend(
    wb: winit::window::WindowBuilder,
    event_loop: &winit::event_loop::EventLoop<()>,
) -> BackendState<back::Backend> {
    let window = wb.build(event_loop).unwrap();

    window.set_outer_position(PhysicalPosition::new(1_300.0, 200.0));

    let instance =
        back::Instance::create("Zeus Engine V0.0.1", 1).expect("Could not create instance");

    let surface = unsafe {
        instance
            .create_surface(&window)
            .expect("Could not create Surface")
    };
    let mut adapters = instance.enumerate_adapters();

    BackendState {
        instance: Some(instance),
        adapter: AdapterState::new(&mut adapters),
        surface: ManuallyDrop::new(surface),
        window,
    }
}
