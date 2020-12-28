use gfx_hal::{
    device::Device,
    format::{
        Format, Swizzle, ImageFeature
    },
    image::{Extent, ViewKind},
    pool::{CommandPool, CommandPoolCreateFlags},
    pso::{
        DescriptorSetLayoutBinding, DescriptorType, ImageDescriptorType, ShaderStageFlags, DescriptorRangeDesc, DescriptorPoolCreateFlags
    },
    Backend,
};

use std::{
    cell::RefCell,
    rc::Rc
};

use super::{
    adapter::AdapterState,
    constants::COLOR_RANGE,
    desc::DescSetLayout,
    device::DeviceState,
    image::ImageState,
    model::Dimensions,
    pass::RenderPassState,
    swapchain::SwapchainState,
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
    //Depth Buffer
    #[allow(dead_code)]
    depth_buffer: Option<ImageState<B>>,
    depth_desc_pool: Option<B::DescriptorPool>,
}

impl<B: Backend> FramebufferState<B> {
    pub unsafe fn new(
        device: Rc<RefCell<DeviceState<B>>>,
        render_pass: &RenderPassState<B>,
        swapchain: &mut SwapchainState<B>,
        adapter: &AdapterState<B>
    ) -> Self {
        
        //DEPTH BUFFER
        let depth_desc = DescSetLayout::new(
            Rc::clone(&device),
            vec![
                DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: DescriptorType::Image {
                        ty:ImageDescriptorType::Sampled {
                            with_sampler: false
                        }
                    },
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
                DescriptorSetLayoutBinding {
                    binding: 1,
                    ty: DescriptorType::Sampler,
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false
                }
            ]
        );

        let mut depth_desc_pool = device.borrow().device.create_descriptor_pool(
            1,
            &[
                DescriptorRangeDesc {
                    ty: DescriptorType::Image {
                        ty: ImageDescriptorType::Sampled {
                            with_sampler: false
                        }
                    },
                    count: 1
                },
                DescriptorRangeDesc {
                    ty: DescriptorType::Sampler,
                    count: 1
                }
            ],
            DescriptorPoolCreateFlags::empty()
        ).ok();

        let depth_desc = depth_desc.create_desc_set(depth_desc_pool.as_mut().unwrap());

        let properties = device.borrow()
            .physical_device_format_properties(Some(Format::D32SfloatS8Uint));
    
        let stencil_support = properties.linear_tiling.contains(ImageFeature::DEPTH_STENCIL_ATTACHMENT) || properties.optimal_tiling.contains(ImageFeature::DEPTH_STENCIL_ATTACHMENT);

        let dims = Dimensions { 
            width: swapchain.extent.width as _,
            height: swapchain.extent.height as _
        };

        let mut staging_pool = device.borrow().device.create_command_pool(
            device.borrow().queues.family,
            CommandPoolCreateFlags::empty(),
        ).expect("Can't create Command Pool");

        let depth_buffer = if stencil_support {
            Some(ImageState::new_depth_image(
                depth_desc,
                dims,
                &adapter,
                &mut device.borrow_mut(),
                &mut staging_pool
            ))
        } else {
            None
        };

        if let Some(depth_buf) = depth_buffer.as_ref() {
            depth_buf.wait_for_transfer_completion();
        }
        

        let (frame_images, framebuffers) = {
            let extent = Extent {
                width: swapchain.extent.width as _,
                height: swapchain.extent.height as _,
                depth: 1,
            };

            let pairs = swapchain
                .backbuffer.take().unwrap()
                .into_iter().map(|image| {
                    let rtv = device.borrow()
                        .device.create_image_view(
                            &image,
                            ViewKind::D2,
                            swapchain.format,
                            Swizzle::NO,
                            COLOR_RANGE.clone(),
                        ).unwrap();
                    (image, rtv)
                }).collect::<Vec<_>>();

            let fbos = pairs.iter().map(|&(_, ref rtv)| {
                    let attachments = if let Some(depth_img_v) = depth_buffer.as_ref().unwrap()
                        .get_image_view() {
                        vec![rtv, depth_img_v]
                    } else {
                        vec![rtv]
                    };

                    device.borrow().device
                        .create_framebuffer(
                            render_pass.render_pass.as_ref().unwrap(),
                            attachments,
                            extent,
                        ).unwrap()
                }).collect();

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
                device.borrow()
                    .device.create_command_pool(
                        device.borrow().queues.family,
                        CommandPoolCreateFlags::empty(),
                    ).expect("Can't create command pool"),
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
            depth_buffer,
            depth_desc_pool
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
            self.device.borrow()
                .device.destroy_descriptor_pool(self.depth_desc_pool.take().unwrap());

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
