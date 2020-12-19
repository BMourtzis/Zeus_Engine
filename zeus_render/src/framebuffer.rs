use gfx_hal::{
    device::Device,
    format::Swizzle,
    image::{Extent, ViewKind},
    pool::{CommandPool, CommandPoolCreateFlags},
    Backend,
};

use std::{cell::RefCell, rc::Rc};

use super::{
    constants::COLOR_RANGE, device::DeviceState, pass::RenderPassState, swapchain::SwapchainState,
};

pub struct FramebufferState<B: Backend> {
    framebuffers: Option<Vec<B::Framebuffer>>,
    framebuffer_fences: Option<Vec<B::Fence>>,
    command_pools: Option<Vec<B::CommandPool>>,
    command_buffer_lists: Vec<Vec<B::CommandBuffer>>,
    frame_images: Option<Vec<(B::Image, B::ImageView)>>,
    acquire_semaphores: Option<Vec<B::Semaphore>>,
    present_semaphores: Option<Vec<B::Semaphore>>,
    last_ref: usize,
    device: Rc<RefCell<DeviceState<B>>>,
}

impl<B: Backend> FramebufferState<B> {
    pub unsafe fn new(
        device: Rc<RefCell<DeviceState<B>>>,
        render_pass: &RenderPassState<B>,
        swapchain: &mut SwapchainState<B>,
    ) -> Self {
        let (frame_images, framebuffers) = {
            let extent = Extent {
                width: swapchain.extent.width as _,
                height: swapchain.extent.height as _,
                depth: 1,
            };

            let pairs = swapchain
                .backbuffer
                .take()
                .unwrap()
                .into_iter()
                .map(|image| {
                    let rtv = device
                        .borrow()
                        .device
                        .create_image_view(
                            &image,
                            ViewKind::D2,
                            swapchain.format,
                            Swizzle::NO,
                            COLOR_RANGE.clone(),
                        )
                        .unwrap();
                    (image, rtv)
                })
                .collect::<Vec<_>>();

            let fbos = pairs
                .iter()
                .map(|&(_, ref rtv)| {
                    device
                        .borrow()
                        .device
                        .create_framebuffer(
                            render_pass.render_pass.as_ref().unwrap(),
                            Some(rtv),
                            extent,
                        )
                        .unwrap()
                })
                .collect();
            (pairs, fbos)
        };

        let iter_count = if !frame_images.is_empty() {
            frame_images.len()
        } else {
            1
        };

        let mut fences: Vec<B::Fence> = vec![];
        let mut command_pools: Vec<_> = vec![];
        let mut command_buffer_lists = Vec::new();
        let mut acquire_semaphores: Vec<B::Semaphore> = vec![];
        let mut present_semaphores: Vec<B::Semaphore> = vec![];

        for _ in 0..iter_count {
            fences.push(device.borrow().device.create_fence(true).unwrap());
            command_pools.push(
                device
                    .borrow()
                    .device
                    .create_command_pool(
                        device.borrow().queues.family,
                        CommandPoolCreateFlags::empty(),
                    )
                    .expect("Can't create command pool"),
            );
            command_buffer_lists.push(Vec::new());

            acquire_semaphores.push(device.borrow().device.create_semaphore().unwrap());
            present_semaphores.push(device.borrow().device.create_semaphore().unwrap());
        }

        FramebufferState {
            frame_images: Some(frame_images),
            framebuffers: Some(framebuffers),
            framebuffer_fences: Some(fences),
            command_pools: Some(command_pools),
            command_buffer_lists,
            present_semaphores: Some(present_semaphores),
            acquire_semaphores: Some(acquire_semaphores),
            device,
            last_ref: 0,
        }
    }

    pub fn next_acq_pre_pair_index(&mut self) -> usize {
        if self.last_ref >= self.acquire_semaphores.as_ref().unwrap().len() {
            self.last_ref = 0
        }

        let ret = self.last_ref;
        self.last_ref += 1;
        ret
    }

    pub fn get_frame_data(
        &mut self,
        frame_id: Option<usize>,
        sem_index: Option<usize>,
    ) -> FrameData<B> {
        FrameData {
            fid: if let Some(fid) = frame_id {
                Some((
                    &mut self.framebuffer_fences.as_mut().unwrap()[fid],
                    &mut self.framebuffers.as_mut().unwrap()[fid],
                    &mut self.command_pools.as_mut().unwrap()[fid],
                    &mut self.command_buffer_lists[fid],
                ))
            } else {
                None
            },
            sid: if let Some(sid) = sem_index {
                Some((
                    &mut self.acquire_semaphores.as_mut().unwrap()[sid],
                    &mut self.present_semaphores.as_mut().unwrap()[sid],
                ))
            } else {
                None
            },
        }
    }
}

impl<B: Backend> Drop for FramebufferState<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;

        unsafe {
            for fence in self.framebuffer_fences.take().unwrap() {
                device.wait_for_fence(&fence, !0).unwrap();
                device.destroy_fence(fence);
            }

            for (mut command_pool, command_buffer_list) in self
                .command_pools
                .take()
                .unwrap()
                .into_iter()
                .zip(self.command_buffer_lists.drain(..))
            {
                command_pool.free(command_buffer_list);
                device.destroy_command_pool(command_pool);
            }

            for acquire_semaphore in self.acquire_semaphores.take().unwrap() {
                device.destroy_semaphore(acquire_semaphore);
            }

            for present_semaphore in self.present_semaphores.take().unwrap() {
                device.destroy_semaphore(present_semaphore);
            }

            for framebuffer in self.framebuffers.take().unwrap() {
                device.destroy_framebuffer(framebuffer);
            }

            for (_, rtv) in self.frame_images.take().unwrap() {
                device.destroy_image_view(rtv);
            }
        }
    }
}

#[derive(Debug)]
pub struct FrameData<'a, B: Backend> {
    /// Frame Id
    pub fid: Option<(
        &'a mut B::Fence,
        &'a mut B::Framebuffer,
        &'a mut B::CommandPool,
        &'a mut Vec<B::CommandBuffer>
    )>,
    /// Semaphore Id
    pub sid: Option<(
        &'a mut B::Semaphore,
        &'a mut B::Semaphore
    )>
}
