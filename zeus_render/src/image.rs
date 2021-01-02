use super::{
    adapter::AdapterState,
    buffer::BufferState,
    constants::{
        DEPTH_RANGE, FILE_EXT, IMAGE_FORMAT
    },
    desc::{
        DescSet,DescSetWrite
    },
    device::DeviceState,
    model::Dimensions
};

use gfx_hal::{Backend, buffer, command::{
        BufferImageCopy, CommandBuffer, CommandBufferFlags, ImageBlit, Level
    }, device::Device, format::{
        Aspects, Format, ImageFeature, Swizzle
    }, image::{
        Access, Extent, Filter, Kind, Layout, Offset, SamplerDesc, Size, SubresourceLayers, Tiling, Usage, ViewCapabilities, ViewKind, WrapMode, Lod, PackedColor, SubresourceRange
    }, memory::{
        Barrier, Dependencies, Properties
    }, pool::CommandPool, pso::{
        Descriptor, PipelineStage, Comparison
    }, queue::{CommandQueue, QueueFamilyId}};

use regex::Regex;

use std::{
    iter,
    rc::Rc,
    fs,
    io::Cursor,
};


#[derive(Debug)]
pub struct ImageState<B: Backend> {
    pub desc: DescSet<B>,
    buffer: Option<BufferState<B>>,
    sampler: Option<B::Sampler>,
    image_view: Option<B::ImageView>,
    image: Option<B::Image>,
    memory: Option<B::Memory>,
    transfered_image_fence: Option<B::Fence>,
    mip_levels: u8
}

impl<B: Backend> ImageState<B> {
    pub fn new_texture(
        mut desc: DescSet<B>,
        img_path: &str,
        adapter: &AdapterState<B>,
        usage: buffer::Usage,
        device_state: &mut DeviceState<B>,
        staging_pool: &mut B::CommandPool,
    ) -> Self {

        let re = Regex::new(FILE_EXT).unwrap();
        let mut file_ext = re.captures(img_path).unwrap().get(1).unwrap().as_str();

        //IMAGE
        let image_bytes = match fs::read(img_path) {
            Ok(img) => img,
            Err(err) => {
                error!("{:?}", err);
                file_ext = "png";
                fs::read("./data/textures/error.png").unwrap()
            }
        };

        let file_format = match file_ext {
            "png" => image::PNG,
            "jpg" | "jpeg" => image::JPEG,
            "gif" => image::GIF,
            "ico" => image::ICO,
            _ => image::PNG
        };

        let img = image::load(
            Cursor::new(&image_bytes[..]),
            file_format
        ).expect("Could not load Image")
        .to_rgba();

        let width = if img.width() > img.height() {
            img.width() as f32
        } else {
            img.height() as f32
        };

        let mip_levels = (width.log2().floor() + 1.0) as u8;

        //BUFFER
        let (buffer, dims, row_pitch, stride) = BufferState::new_texture(
            Rc::clone(&desc.layout.device),
            &device_state.device,
            &img,
            adapter,
            usage,
        );

        let buffer = Some(buffer);
        let device = &mut device_state.device;

        let mut image = unsafe {
            device.create_image(
                Kind::D2(dims.width as Size, dims.height as Size, 1, 1),
                mip_levels,
                IMAGE_FORMAT,
                Tiling::Optimal,
                Usage::TRANSFER_SRC | Usage::TRANSFER_DST | Usage::SAMPLED,
                ViewCapabilities::empty(),
            )
        }.expect("Could not create image");

        let req = unsafe { 
            device.get_image_requirements(&image) 
        };

        let device_type = adapter
            .memory_types.iter().enumerate()
            .position(|(id, memory_type)| {
                req.type_mask & (1 << id) != 0
                    && memory_type.properties.contains(Properties::DEVICE_LOCAL)
            }).unwrap()
            .into();

        let memory = unsafe { 
            device.allocate_memory(device_type, req.size) 
        }.expect("Could not allocate memory for image");

        unsafe { 
            device.bind_image_memory(&memory, 0, &mut image) 
        }.expect("Could not bind image memroy");

        //Create Image View and Sampler.
        let image_view = unsafe {
            device.create_image_view(
                &image,
                ViewKind::D2,
                IMAGE_FORMAT,
                Swizzle::NO,
                SubresourceRange {
                    aspects: Aspects::COLOR,
                    level_start: 0,
                    level_count: Some(mip_levels),
                    layer_start: 0,
                    layer_count: Some(1)
                },
            )
        }.expect("Could not create image view");
        
        let sampler = unsafe { 
            device.create_sampler(&SamplerDesc {
                mag_filter: Filter::Linear,
                min_filter: Filter::Linear,
                mip_filter: Filter::Nearest,
                wrap_mode: (WrapMode::Clamp, WrapMode::Clamp, WrapMode::Clamp),
                lod_bias: Lod(0.0_f32),
                lod_range: Lod(0.0) .. Lod(mip_levels as f32),
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

        let transfered_image_fence = device.create_fence(false)
            .expect("Can't create fence");

        let device_props = device_state
            .physical_device_format_properties(Some(IMAGE_FORMAT));

        //Copy buffer to texture
        unsafe {
            let mut cmd_buffer = staging_pool.allocate_one(Level::Primary);
            cmd_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

            cmd_buffer.pipeline_barrier(
                PipelineStage::TOP_OF_PIPE .. PipelineStage::TRANSFER,
                Dependencies::empty(),
                &[Barrier::Image {
                    states: (Access::empty(), Layout::Undefined)
                        ..(Access::TRANSFER_WRITE, Layout::TransferDstOptimal),
                    target: &image,
                    families: Some(QueueFamilyId(0) .. QueueFamilyId(0)),
                    range: SubresourceRange {
                        aspects: Aspects::COLOR,
                        level_start: 0,
                        level_count: Some(mip_levels),
                        layer_start: 0,
                        layer_count: Some(1)
                    }
                }],
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

            if device_props.optimal_tiling.contains(ImageFeature::SAMPLED_LINEAR) {
                let mut mip_width = img.width();
                let mut mip_height = img.height();

                for i in 1 .. mip_levels {
                    cmd_buffer.pipeline_barrier(
                        PipelineStage::TRANSFER .. PipelineStage::TRANSFER,
                        Dependencies::empty(),
                        &[Barrier::Image {
                            states: (Access::TRANSFER_WRITE, Layout::TransferDstOptimal) 
                            .. (Access::TRANSFER_READ, Layout::TransferSrcOptimal),
                            target: &image,
                            families: None,
                            range: SubresourceRange {
                                aspects: Aspects::COLOR,
                                level_start: i - 1,
                                level_count: Some(1),
                                layer_start: 0,
                                layer_count: Some(1)
                            }
                        }]
                    );

                    cmd_buffer.blit_image(
                        &image,
                        Layout::TransferSrcOptimal,
                        &image,
                        Layout::TransferDstOptimal,
                        Filter::Linear,
                        &[ImageBlit {
                            src_subresource: SubresourceLayers {
                                aspects: Aspects::COLOR,
                                level: i - 1,
                                layers: 0 .. 1
                            },
                            src_bounds: Offset::ZERO .. Offset {
                                x: mip_width as i32,
                                y: mip_height as i32,
                                z: 1
                            },
                            dst_subresource: SubresourceLayers {
                                aspects: Aspects::COLOR,
                                level: i,
                                layers: 0 .. 1
                            },
                            dst_bounds: Offset::ZERO .. Offset {
                                x: if mip_width > 1 {
                                    (mip_width / 2) as i32
                                } else { 1 },
                                y: if mip_height > 1 {
                                    (mip_height / 2) as i32
                                } else { 1 },
                                z: 1
                            }
                        }]
                    );

                    cmd_buffer.pipeline_barrier(
                        PipelineStage::TRANSFER .. PipelineStage::FRAGMENT_SHADER,
                        Dependencies::empty(),
                        &[Barrier::Image {
                            states: (Access::TRANSFER_READ, Layout::TransferSrcOptimal) 
                            .. (Access::SHADER_READ, Layout::ShaderReadOnlyOptimal),
                            target: &image,
                            families: None,
                            range: SubresourceRange {
                                aspects: Aspects::COLOR,
                                level_start: i - 1,
                                level_count: Some(1),
                                layer_start: 0,
                                layer_count: Some(1)
                            }
                        }]
                    );

                    if mip_width > 1 {
                        mip_width /= 2;
                    }
                    
                    if mip_height > 1 {
                        mip_height /= 2;
                    }
                    
                }
            }

            cmd_buffer.pipeline_barrier(
                PipelineStage::TRANSFER .. PipelineStage::FRAGMENT_SHADER,
                Dependencies::empty(),
                &[Barrier::Image {
                    states: (Access::TRANSFER_WRITE, Layout::TransferDstOptimal)
                        ..(Access::SHADER_READ, Layout::ShaderReadOnlyOptimal),
                    target: &image,
                    families: None,
                    range: SubresourceRange {
                        aspects: Aspects::COLOR,
                        level_start: mip_levels - 1,
                        level_count: Some(mip_levels),
                        layer_start: 0,
                        layer_count: Some(1)
                    }
                }]
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
            mip_levels
        }
    }

    pub fn new_depth_image(
        desc: DescSet<B>,
        dims: Dimensions<u32>,
        adapter: &AdapterState<B>,
        device_state: &mut DeviceState<B>,
        staging_pool: &mut B::CommandPool,
    ) -> Self {
        let device = &mut device_state.device;

        let mut depth_image = unsafe {
            device.create_image(
                Kind::D2(dims.width as Size, dims.height as Size, 1, 1),
                1,
                Format::D32SfloatS8Uint,
                Tiling::Optimal,
                Usage::DEPTH_STENCIL_ATTACHMENT,
                ViewCapabilities::empty()
            )
        }.expect("Could not create depth image");

        let req = unsafe {
            device.get_image_requirements(&depth_image)
        };

        let device_type = adapter.memory_types
            .iter().enumerate().position(|(id, memory_type)| {
                req.type_mask & (1 << id) != 0
                    && memory_type.properties.contains(Properties::DEVICE_LOCAL)
            }).unwrap().into();
        
        let memory = unsafe { 
            device.allocate_memory(device_type, req.size) 
        }.expect("Could not allocation memory for image");

        unsafe { 
            device.bind_image_memory(&memory, 0, &mut depth_image) 
        }.expect("Could not bind image memory");

        //Create Image View
        let depth_image_view = unsafe {
            device.create_image_view(
                &depth_image,
                ViewKind::D2,
                Format::D32SfloatS8Uint,
                Swizzle::NO,
                DEPTH_RANGE.clone()
            )
        }.expect("Could not create depth image view");

        let sampler = unsafe {
            device.create_sampler(&SamplerDesc {
                mag_filter: Filter::Linear,
                min_filter: Filter::Linear,
                mip_filter: Filter::Linear,
                wrap_mode: (WrapMode::Clamp, WrapMode::Clamp, WrapMode::Clamp),
                lod_bias: Lod(0.0_f32),
                lod_range: Lod::RANGE,
                comparison: Some(Comparison::Always),
                border: PackedColor(0_u32),
                normalized: true,
                anisotropy_clamp: None
            })
        }.expect("Could not create depth sampler");

        let transfered_image_fence = device.create_fence(false)
            .expect("Could not create depth fence");

        //Copy buffer to depth image
        unsafe {
            let mut cmd_buffer = staging_pool.allocate_one(Level::Primary);
            cmd_buffer.begin_primary (CommandBufferFlags::ONE_TIME_SUBMIT);

            cmd_buffer.pipeline_barrier(
                PipelineStage::TOP_OF_PIPE..PipelineStage::EARLY_FRAGMENT_TESTS,
                Dependencies::empty(),
                &[Barrier::Image {
                    states: (Access::empty(), Layout::Undefined) .. (Access::DEPTH_STENCIL_ATTACHMENT_READ | Access::DEPTH_STENCIL_ATTACHMENT_WRITE, Layout::DepthStencilAttachmentOptimal),
                    target: &depth_image,
                    families: None,
                    range: DEPTH_RANGE.clone()
                }]
            );

            cmd_buffer.finish();

            device_state.queues.queues[0]
                .submit_without_semaphores(iter::once(&cmd_buffer), Some(&transfered_image_fence));
        }

        ImageState {
            desc,
            buffer: None,
            sampler: Some(sampler),
            image_view: Some(depth_image_view),
            image: Some(depth_image),
            memory: Some(memory),
            transfered_image_fence: Some(transfered_image_fence),
            mip_levels: 0
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

    pub fn get_image_view(&self) -> Option<&B::ImageView> {
        if self.image_view.is_none() {
            None
        } else {
            self.image_view.as_ref()
        }
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

        if self.buffer.is_some() {
            self.buffer.take().expect("No Buffer found!");
        }
        
    }
}
