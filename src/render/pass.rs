use gfx_hal::{
    Backend,
    device::Device,
    pass::{Attachment, SubpassDesc, AttachmentOps, AttachmentLoadOp::Clear, AttachmentStoreOp::Store},
    image::Layout
};
use std::{
    cell::RefCell,
    rc::Rc,
};
use super::{
    swapchain::SwapchainState,
    device::DeviceState
};

pub struct RenderPassState<B: Backend> {
    pub render_pass: Option<B::RenderPass>,
    device: Rc<RefCell<DeviceState<B>>>
}

impl<B: Backend> RenderPassState<B> {
    pub unsafe fn new(swapchain: &SwapchainState<B>, device: Rc<RefCell<DeviceState<B>>>) -> Self {
        let render_pass = {
            let attachment = Attachment {
                format: Some(swapchain.format.clone()),
                samples: 1,
                ops: AttachmentOps::new(Clear, Store),
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: Layout::Undefined .. Layout::Present
            };

            let subpass = SubpassDesc {
                colors: &[(0, Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[]
            };

            device.borrow()
                .device.create_render_pass(&[attachment], &[subpass], &[])
                .ok()
        };

        RenderPassState {
            render_pass,
            device
        }
    }
}

impl<B: Backend> Drop for RenderPassState<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        unsafe {
            device.destroy_render_pass(self.render_pass.take().unwrap());
        }
    }
}