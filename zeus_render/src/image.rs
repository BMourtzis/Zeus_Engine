use super::{
    adapter::AdapterState,
    buffer::BufferState,
    constants::COLOR_RANGE,
    desc::{DescSet, DescSetWrite},
    device::DeviceState,
};

use gfx_hal::{
    buffer,
    command::{BufferImageCopy, CommandBuffer, CommandBufferFlags, Level},
    device::Device,
    format::{AsFormat, Aspects, Rgba8Srgb, Swizzle},
    image::{
        Access, Extent, Filter, Kind, Layout, Offset, SamplerDesc, Size, SubresourceLayers, Tiling,
        Usage, ViewCapabilities, ViewKind, WrapMode, Lod, PackedColor
    },
    memory::{Barrier, Dependencies, Properties},
    pool::CommandPool,
    pso::{Descriptor, PipelineStage, Comparison},
    queue::CommandQueue,
    Backend,
};

use image::{ImageBuffer, Rgba};

use std::{iter, rc::Rc};

pub struct ImageState<B: Backend> {
    pub desc: DescSet<B>,
    buffer: Option<BufferState<B>>,
    sampler: Option<B::Sampler>,
    image_view: Option<B::ImageView>,
    image: Option<B::Image>,
    memory: Option<B::Memory>,
    transfered_image_fence: Option<B::Fence>,
}

impl<B: Backend> ImageState<B> {
    pub fn new(
        mut desc: DescSet<B>,
        img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        adapter: &AdapterState<B>,
        usage: buffer::Usage,
        device_state: &mut DeviceState<B>,
        staging_pool: &mut B::CommandPool,
    ) -> Self {
        let (buffer, dims, row_pitch, stride) = BufferState::new_texture(
            Rc::clone(&desc.layout.device),
            &device_state.device,
            img,
            adapter,
            usage,
        );

        let buffer = Some(buffer);
        let device = &mut device_state.device;

        // let kind = Kind::D2(dims.width as Size, dims.height as Size, 1, 1);
        let mut image = unsafe {
            device.create_image(
                Kind::D2(dims.width as Size, dims.height as Size, 1, 1),
                1,
                Rgba8Srgb::SELF,
                Tiling::Optimal,
                Usage::TRANSFER_DST | Usage::SAMPLED,
                ViewCapabilities::empty(),
            )
        }.expect("Could not create image");
        let req = unsafe { device.get_image_requirements(&image) };

        let device_type = adapter
            .memory_types
            .iter()
            .enumerate()
            .position(|(id, memory_type)| {
                req.type_mask & (1 << id) != 0
                    && memory_type.properties.contains(Properties::DEVICE_LOCAL)
            })
            .unwrap()
            .into();

        let memory = unsafe { device.allocate_memory(device_type, req.size) }
            .expect("Could not allocate memory for image");

        unsafe { device.bind_image_memory(&memory, 0, &mut image) }
            .expect("Could not bind image memroy");

        //Create Image View and Sampler.

        let image_view = unsafe {
            device.create_image_view(
                &image,
                ViewKind::D2,
                Rgba8Srgb::SELF,
                Swizzle::NO,
                COLOR_RANGE.clone(),
            )
        }.expect("Could not create image view");
        
        let sampler = unsafe { 
            device.create_sampler(&SamplerDesc {
                mag_filter: Filter::Linear,
                min_filter: Filter::Linear,
                mip_filter: Filter::Linear,
                wrap_mode: (WrapMode::Clamp, WrapMode::Clamp, WrapMode::Clamp),
                lod_bias: Lod(0.0_f32),
                lod_range: Lod::RANGE,
                //od_range: Lod(0.0_f32)..Lod(0.0_f32),
                comparison: Some(Comparison::Always),
                border: PackedColor(0_u32),
                normalized: true,
                //Anisotropy is not enabled in the current feature list
                anisotropy_clamp: None,
            })
            }.expect("Can't create sampler");

        desc.write_to_state(
            vec![
                DescSetWrite {
                    binding: 0,
                    array_offset: 0,
                    descriptors: Some(Descriptor::Image(
                        &image_view,
                        Layout::ShaderReadOnlyOptimal,
                    )),
                },
                DescSetWrite {
                    binding: 1,
                    array_offset: 0,
                    descriptors: Some(Descriptor::Sampler(&sampler)),
                },
            ],
            device,
        );

        let transfered_image_fence = device.create_fence(false).expect("Can't create fence");

        //Copy buffer to texture
        unsafe {
            let mut cmd_buffer = staging_pool.allocate_one(Level::Primary);
            cmd_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

            let image_barrier = Barrier::Image {
                states: (Access::empty(), Layout::Undefined)
                    ..(Access::TRANSFER_WRITE, Layout::TransferDstOptimal),
                target: &image,
                families: None,
                range: COLOR_RANGE.clone(),
            };

            cmd_buffer.pipeline_barrier(
                PipelineStage::TOP_OF_PIPE..PipelineStage::TRANSFER,
                Dependencies::empty(),
                &[image_barrier],
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
                        layers: 0..1,
                    },
                    image_offset: Offset { x: 0, y: 0, z: 0 },
                    image_extent: Extent {
                        width: dims.width,
                        height: dims.height,
                        depth: 1,
                    },
                }],
            );

            let image_barrier = Barrier::Image {
                states: (Access::TRANSFER_WRITE, Layout::TransferDstOptimal)
                    ..(Access::SHADER_READ, Layout::ShaderReadOnlyOptimal),
                target: &image,
                families: None,
                range: COLOR_RANGE.clone(),
            };
            cmd_buffer.pipeline_barrier(
                PipelineStage::TRANSFER..PipelineStage::FRAGMENT_SHADER,
                Dependencies::empty(),
                &[image_barrier],
            );

            cmd_buffer.finish();

            device_state.queues.queues[0]
                .submit_without_semaphores(iter::once(&cmd_buffer), Some(&transfered_image_fence));
        }

        ImageState {
            desc,
            buffer,
            sampler: Some(sampler),
            image_view: Some(image_view),
            image: Some(image),
            memory: Some(memory),
            transfered_image_fence: Some(transfered_image_fence),
        }
    }

    pub fn wait_for_transfer_completion(&self) {
        let device = &self.desc.layout.device.borrow().device;
        unsafe {
            device
                .wait_for_fence(self.transfered_image_fence.as_ref().unwrap(), !0)
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
