//NOTE: For now we only use Vulkan until we can undestand the layout and design a architecture
extern crate gfx_backend_vulkan as back;

use gfx_hal::{
    Backend,
    Instance
};
use std::{
    mem::ManuallyDrop,
    ptr
};
use super::adapter::AdapterState;

pub struct BackendState<B: Backend> {
    instance: Option<B::Instance>,
    pub surface: ManuallyDrop<B::Surface>,
    adapter: AdapterState<B>,
    #[allow(dead_code)]
    window: winit::window::Window
}

impl<B: Backend> Drop for BackendState<B> {
    fn drop(&mut self) {
        if let Some(instance) = self.instance {
            unsafe {
                let surface = ManuallyDrop::into_inner(ptr::read(&self.surface));
                instance.destroy_surface(surface);
            }
        }
    }
}


pub fn create_backend(
    wb: winit::window::WindowBuilder, 
    event_loop: &winit::event_loop::EventLoop<()>
) -> BackendState<back::Backend> {
    let window = wb.build(event_loop).unwrap();
    let instance = back::Instance::create("Zeus Engine V0.0.1", 1)
        .expect("Could not creat instance");
    let surface = unsafe {
        instance.create_surface(&window).expect("Could not create Surface")
    };
    let mut adapters = instance.enumerate_adapters();

    BackendState {
        instance: Some(instance),
        adapter: AdapterState::new(&mut adapters),
        surface: ManuallyDrop::new(surface),
        window
    }
}

//TODO: add gl version