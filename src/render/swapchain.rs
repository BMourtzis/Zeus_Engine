use gfx_hal::{
    Backend,
    format::{
        Format,
        ChannelType
    },
    image::Extent,
    device::Device,
    window::{ 
        Surface,
        SwapchainConfig,
        PresentMode
    }
};

use std::{
    cell::RefCell,
    rc::Rc
};

use super::{
    device::DeviceState,
    backend::BackendState,
    constants::DIMS
};

pub struct SwapchainState<B: Backend>{
    device: Rc<RefCell<DeviceState<B>>>,
    pub swapchain: Option<B::Swapchain>,
    pub backbuffer: Option<Vec<B::Image>>,
    pub extent: Extent,
    pub format: Format
}

impl<B: Backend> SwapchainState<B> {
    pub unsafe fn new (backend: &mut BackendState<B>, device: Rc<RefCell<DeviceState<B>>>) -> Self {
        let caps = backend.surface.capabilities(&device.borrow().physical_device);
        let formats = backend.surface.supported_formats(&device.borrow().physical_device);

        // println!("caps: {:?}", caps);
        // println!("formats: {:?}", formats);

        let format = formats.map_or(Format::Rgba8Srgb, |formats| {
            formats.iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .map(|format| *format)
                .unwrap_or(formats[0])
        });

        // println!("Surface format: {:?}", format);
        
        //TODO: Should probably make my own config
        let swap_config = SwapchainConfig::from_caps(&caps, format, DIMS)
            .with_present_mode(if caps.present_modes.contains(PresentMode::MAILBOX) {
                PresentMode::MAILBOX
            } else {
                PresentMode::FIFO
            });

        // println!("{:?}", swap_config);

        let extent = swap_config.extent.to_extent();
        let (swapchain, backbuffer) = device.borrow()
            .device.create_swapchain(&mut backend.surface, swap_config, None)
            .expect("Could not create swapchain");

        SwapchainState {
            swapchain: Some(swapchain),
            backbuffer: Some(backbuffer),
            device,
            extent,
            format
        }
    }
}

impl<B: Backend> Drop for SwapchainState<B> {
    fn drop(&mut self) {
        unsafe {
            self.device
                .borrow()
                .device
                .destroy_swapchain(self.swapchain.take().unwrap());
        }
    }
}