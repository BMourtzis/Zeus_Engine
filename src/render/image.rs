use super::{
    adapter::AdapterState,
    buffer::BufferState,
    constants::COLOR_RANGE,
    desc::{
        DescSet,
        DescSetWrite
    },
    device::DeviceState
};

use gfx_hal::{
    Backend,
    device::Device,
    buffer,
    command::{
        Level,
        CommandBufferFlags,
        BufferImageCopy,
        CommandBuffer
    },
    format::{
        Aspects,
        Rgba8Srgb,
        AsFormat,
        Swizzle
    },
    image::{
        Access,
        Kind,
        Size,
        Tiling,
        Usage,
        ViewCapabilities,
        ViewKind,
        SamplerDesc,
        Filter,
        WrapMode,
        Layout,
        SubresourceLayers,
        Offset,
        Extent
    },
    memory::{
        Barrier,
        Properties,
        Dependencies
    },
    pool::CommandPool,
    pso::{
        Descriptor,
        PipelineStage
    },
    queue::CommandQueue
};

use image::{
    ImageBuffer,
    Rgba
};

use std::{
    rc::Rc,
    iter
};

pub struct ImageState<B: Backend> {
    pub desc: DescSet<B>,
    buffer: Option<BufferState<B>>,
    sampler: Option<B::Sampler>,
    image_view: Option<B::ImageView>,
    image: Option<B::Image>,
    memory: Option<B::Memory>,
    transfered_image_fence: Option<B::Fence>
}

impl<B: Backend> ImageState<B> {
    pub unsafe fn new(
        mut desc: DescSet<B>,
        img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        adapter: &AdapterState<B>,
        usage: buffer::Usage,
        device_state: &mut DeviceState<B>,
        staging_pool: &mut B::CommandPool
    ) -> Self {
        let (buffer, dims, row_pitch, stride) = BufferState::new_texture(
            Rc::clone(&desc.layout.device),
            &device_state.device,
            img,
            adapter,
            usage
        );

        let buffer = Some(buffer);
        let device = &mut device_state.device;

        let kind = Kind::D2(dims.width as Size, dims.height as Size, 1, 1);
        let mut image = device.create_image(
            kind,
            1,
            Rgba8Srgb::SELF,
            Tiling::Optimal,
            Usage::TRANSFER_DST | Usage::SAMPLED,
            ViewCapabilities::empty()
        ).unwrap();
        let req = device.get_image_requirements(&image);

        let device_type = adapter.memory_types
            .iter().enumerate().position(|(id, memory_type)| {
                req.type_mask & (1 << id) != 0 && memory_type.properties.contains(Properties::DEVICE_LOCAL)
            }).unwrap().into();

        let memory = device.allocate_memory(device_type, req.size).unwrap();

        device.bind_image_memory(&memory, 0, &mut image).unwrap();
        let image_view = device.create_image_view(
            &image,
            ViewKind::D2,
            Rgba8Srgb::SELF,
            Swizzle::NO,
            COLOR_RANGE.clone()
        ).unwrap();

        let sampler = device.create_sampler(&SamplerDesc::new(Filter::Linear, WrapMode::Clamp))
            .expect("Can't create sampler");

        desc.write_to_state(
            vec![
                DescSetWrite {
                    binding: 0,
                    array_offset: 0,
                    descriptors: Some(Descriptor::Image(
                        &image_view,
                        Layout::ShaderReadOnlyOptimal
                    ))
                },
                DescSetWrite {
                    binding: 1,
                    array_offset: 0,
                    descriptors: Some(Descriptor::Sampler(&sampler))
                }
            ],
            device
        );

        let transfered_image_fence = device.create_fence(false)
            .expect("Can't create fence");
        
        //copy buffer to texture
        {
            let mut cmd_buffer = staging_pool.allocate_one(Level::Primary);
            cmd_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

            let image_barrier = Barrier::Image {
                states: (Access::empty(), Layout::Undefined)
                    .. (Access::TRANSFER_WRITE, Layout::TransferDstOptimal),
                target: &image,
                families: None,
                range: COLOR_RANGE.clone()
            };

            cmd_buffer.pipeline_barrier(
                PipelineStage::TOP_OF_PIPE .. PipelineStage::TRANSFER,
                Dependencies::empty(),
                &[image_barrier]
            );

            cmd_buffer.copy_buffer_to_image(
                buffer.as_ref().unwrap().get_buffer(),
                &image,
                Layout::TransferDstOptimal,
                &[BufferImageCopy {
                    buffer_offset: 0,
                    buffer_width: row_pitch / (stride as u32),
                    buffer_height: dims.height as u32,
                    image_layers: SubresourceLayers {
                        aspects: Aspects::COLOR,
                        level: 0,
                        layers: 0 .. 1 
                    },
                    image_offset: Offset { x: 0, y: 0, z: 0},
                    image_extent: Extent {
                        width: dims.width,
                        height: dims.height,
                        depth: 1
                    }
                }]
            );

            let image_barrier = Barrier::Image {
                states: (Access::TRANSFER_WRITE, Layout::TransferDstOptimal)
                    .. (Access::SHADER_READ, Layout::ShaderReadOnlyOptimal),
                target: &image,
                families: None,
                range: COLOR_RANGE.clone()
            };
            cmd_buffer.pipeline_barrier(
                PipelineStage::TRANSFER .. PipelineStage::FRAGMENT_SHADER,
                Dependencies::empty(),
                &[image_barrier]
            );

            cmd_buffer.finish();

            device_state.queues.queues[0].submit_without_semaphores(
                iter::once(&cmd_buffer),
                Some(&transfered_image_fence)
            );
        }

        ImageState {
            desc,
            buffer,
            sampler: Some(sampler),
            image_view: Some(image_view),
            image: Some(image),
            memory: Some(memory),
            transfered_image_fence: Some(transfered_image_fence)
        }
    }

    pub fn wait_for_transfer_completion(&self) {
        let device = &self.desc.layout.device.borrow().device;
        unsafe {
            device.wait_for_fence(self.transfered_image_fence.as_ref().unwrap(), !0)
                .unwrap();
        }
    }

    pub fn get_layout(&self) -> &B::DescriptorSetLayout {
        self.desc.get_layout()
    }
}

impl<B: Backend> Drop for ImageState<B> {
    fn drop(&mut self) {
        unsafe {
            let device = &self.desc.layout.device.borrow().device;

            let fence = self.transfered_image_fence.take().unwrap();
            device.wait_for_fence(&fence, !0).unwrap();
            device.destroy_fence(fence);

            device.destroy_sampler(self.sampler.take().unwrap());
            device.destroy_image_view(self.image_view.take().unwrap());
            device.destroy_image(self.image.take().unwrap());
            device.free_memory(self.memory.take().unwrap());
        }

        self.buffer.take().unwrap();
    }
}