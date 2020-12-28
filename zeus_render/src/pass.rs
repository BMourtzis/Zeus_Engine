use super::{
    device::DeviceState,
    swapchain::SwapchainState
};

use gfx_hal::{
    Backend,
    device::Device,
    format::Format,
    image::{
        Access, Layout,
    },
    memory::Dependencies,
    pass::{
        Attachment,AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, SubpassDependency, SubpassDesc
    },
    pso::PipelineStage
};

use std::{
    cell::RefCell,
    rc::Rc
};

pub struct RenderPassState<B: Backend> {
    device: Rc<RefCell<DeviceState<B>>>,
    pub render_pass: Option<B::RenderPass>,
}

impl<B: Backend> RenderPassState<B> {
    pub fn new(
        swapchain: &SwapchainState<B>,
        device: Rc<RefCell<DeviceState<B>>>,
    ) -> Self {
        let render_pass = {
            let color_attachment = Attachment {
                format: Some(swapchain.format),
                samples: 1,
                ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: Layout::Undefined..Layout::Present,
            };

            let depth_attachment = Attachment {
                format: Some(Format::D32SfloatS8Uint),
                samples: 1,
                ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::DontCare),
                stencil_ops: AttachmentOps::new(AttachmentLoadOp::DontCare, AttachmentStoreOp::DontCare),
                layouts: Layout::Undefined..Layout::DepthStencilAttachmentOptimal
            };

            let subpass = SubpassDesc {
                colors: &[(0, Layout::ColorAttachmentOptimal)],
                depth_stencil: Some(&(1, Layout::DepthStencilAttachmentOptimal)),
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };

            let dependency = SubpassDependency {
                passes: None .. None,
                stages: (PipelineStage::COLOR_ATTACHMENT_OUTPUT | PipelineStage::EARLY_FRAGMENT_TESTS) .. (PipelineStage::COLOR_ATTACHMENT_OUTPUT | PipelineStage::EARLY_FRAGMENT_TESTS),
                accesses: Access::empty() .. (Access::COLOR_ATTACHMENT_WRITE | Access::DEPTH_STENCIL_ATTACHMENT_WRITE),
                flags: Dependencies::empty()
            };

            unsafe {
                device.borrow()
                    .device.create_render_pass(&[color_attachment, depth_attachment], &[subpass], &[dependency])
            }.ok()
        };

        RenderPassState {
            render_pass,
            device,
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
