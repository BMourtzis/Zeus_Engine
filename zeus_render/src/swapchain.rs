use gfx_hal::{
    device::Device,
    format::{ChannelType, Format},
    image::Extent,
    window::{PresentMode, Surface, SwapchainConfig},
    Backend,
};

use std::{cell::RefCell, rc::Rc};

use super::{backend::BackendState, constants::DIMS, device::DeviceState};

pub struct SwapchainState<B: Backend> {
    device: Rc<RefCell<DeviceState<B>>>,
    pub swapchain: Option<B::Swapchain>,
    pub backbuffer: Option<Vec<B::Image>>,
    pub extent: Extent,
    pub format: Format,
    pub size: u32,
}

impl<B: Backend> SwapchainState<B> {
    pub fn new(
        backend: &mut BackendState<B>,
        device: Rc<RefCell<DeviceState<B>>>,
    ) -> Self {
        let caps = backend
            .surface
            .capabilities(&device.borrow().physical_device);
        let formats = backend
            .surface
            .supported_formats(&device.borrow().physical_device);

        let format = formats.map_or(Format::Rgba8Srgb, |formats| {
            formats
                .iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .copied()
                .unwrap_or(formats[0])
        });

        let swap_config = SwapchainConfig::new(
            DIMS.width,
            DIMS.height,
            format,
            3, //TODO: add a check
        )
        .with_present_mode(if caps.present_modes.contains(PresentMode::MAILBOX) {
            PresentMode::MAILBOX
        } else {
            PresentMode::FIFO
        });

        let size = swap_config.image_count;

        let extent = swap_config.extent.to_extent();
        let (swapchain, backbuffer) = unsafe {
            device
                .borrow()
                .device
                .create_swapchain(&mut backend.surface, swap_config, None)
        }
        .expect("Could not create swapchain");

        SwapchainState {
            swapchain: Some(swapchain),
            backbuffer: Some(backbuffer),
            device,
            extent,
            format,
            size,
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
