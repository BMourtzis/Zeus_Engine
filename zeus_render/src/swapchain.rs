use gfx_hal::{
    format::{ ChannelType, Format },
    image::Extent,
    window::{PresentMode, PresentationSurface, Surface, SwapchainConfig, Extent2D},
    Backend,
};

use std::{cell::RefCell, rc::Rc};

use super::{backend::BackendState, device::DeviceState};

pub struct SwapchainState<B: Backend> {
    #[allow(dead_code)]
    device: Rc<RefCell<DeviceState<B>>>,
    pub extent: Extent,
    pub format: Format,
    pub size: u32,
    pub frame_index: u32
}

impl<B: Backend> SwapchainState<B> {
    pub fn new(
        backend: &mut BackendState<B>,
        device: Rc<RefCell<DeviceState<B>>>,
        window_dimensions: Extent2D,
    ) -> Self {
        let caps = backend.surface
            .capabilities(&device.borrow().physical_device);
        let formats = backend.surface
            .supported_formats(&device.borrow().physical_device);

        let format = formats.map_or(Format::Rgba8Srgb, |formats| {
            formats.iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .copied().unwrap_or(formats[0])
        });

        let swap_config = SwapchainConfig::new(
            window_dimensions.width,
            window_dimensions.height,
            format,
            3, //TODO: add a check
        ).with_present_mode(if caps.present_modes.contains(PresentMode::MAILBOX) {
            PresentMode::MAILBOX
        } else {
            PresentMode::FIFO
        });

        let size = swap_config.image_count;

        let extent = swap_config.extent.to_extent();
        let frame_index = swap_config.image_count;
        
        unsafe {
            backend.surface.configure_swapchain(
                &device.borrow().device,
                swap_config
            )
        }.expect("Could not create swapchain");

        SwapchainState {
            device,
            extent,
            format,
            size,
            frame_index
        }
    }
}

impl<B: Backend> Drop for SwapchainState<B> {
    fn drop(&mut self) {
        // unsafe {
        //     self.device
        //         .borrow().device
        //         .destroy_swapchain(self.swapchain.take().unwrap());
        // }
    }
}
