use super::{
    adapter::AdapterState,
    desc::DescSetLayout,
    device::DeviceState,
    image::ImageState,
    model::Dimensions
};

use gfx_hal::{
    adapter::MemoryType,
    buffer::Usage,
    command::{
        BufferCopy, CommandBuffer, CommandBufferFlags, Level
    },
    device::Device,
    format::{
        Format, ImageFeature
    },
    memory::{
        Properties, Segment
    },
    pool::{
        CommandPool, CommandPoolCreateFlags
    },
    pso::{
        DescriptorPoolCreateFlags, DescriptorRangeDesc, DescriptorSetLayoutBinding, DescriptorType, ImageDescriptorType, ShaderStageFlags
    },
    queue::CommandQueue,
    Backend,
};

use std::{
    cell::RefCell,
    iter,
    mem::size_of,
    ptr,
    rc::Rc
};

#[derive(Debug)]
pub struct BufferState<B: Backend> {
    memory: Option<B::Memory>,
    pub buffer: Option<B::Buffer>,
    device: Rc<RefCell<DeviceState<B>>>,
    size: u64,
}

impl<B: Backend> BufferState<B> {
    //region Constructors

    ///Creates a new buffer and maps the memory
    pub fn new<T>(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        data_source: &[T],
        usage: Usage,
        memory_types: &[MemoryType],
        memory_properties: Properties,
    ) -> Self
    where
        T: Copy,
    {
        let mut buffer_state = BufferState::new_unmapped::<T>(
            Rc::clone(&device_ptr),
            data_source.len(),
            usage,
            memory_types,
            memory_properties,
        );

        buffer_state.update_data(0, data_source);

        buffer_state
    }

    ///Creates new Buffer state without mapping the memory
    pub fn new_unmapped<T>(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        data_length: usize,
        usage: Usage,
        memory_types: &[MemoryType],
        memory_properties: Properties,
    ) -> Self {
        let memory: B::Memory;
        let mut buffer: B::Buffer;
        let size: u64;

        let stride = size_of::<T>();
        let upload_size = data_length * stride;

        unsafe {
            let device = &device_ptr.borrow().device;

            //TODO: Can we set sharing mode?
            buffer = device.create_buffer(upload_size as u64, usage)
                .unwrap();
            let mem_req = device.get_buffer_requirements(&buffer);

            let upload_type = memory_types
                .iter().enumerate()
                .position(|(id, mem_type)| {
                    mem_req.type_mask & (1 << id) != 0
                        && mem_type.properties.contains(memory_properties)
                }).unwrap()
                .into();

            memory = device.allocate_memory(upload_type, mem_req.size)
                .unwrap();
            device.bind_buffer_memory(&memory, 0, &mut buffer)
                .unwrap();
            size = mem_req.size;
        }

        BufferState {
            memory: Some(memory),
            buffer: Some(buffer),
            device: device_ptr,
            size,
        }
    }

    /// Creates a new buffer to save textures
    pub fn new_texture(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        device: &B::Device,
        img: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        adapter: &AdapterState<B>,
        usage: Usage,
    ) -> (Self, Dimensions<u32>, u32, usize) {
        let (width, height) = img.dimensions();

        let row_alignment_mask = adapter.limits.optimal_buffer_copy_pitch_alignment as u32 - 1;
        let stride = 4usize;

        let row_pitch = (width * stride as u32 + row_alignment_mask) & !row_alignment_mask;
        let upload_size = (height * row_pitch) as u64;

        let memory: B::Memory;
        let mut buffer: B::Buffer;
        let size: u64;

        unsafe {
            buffer = device.create_buffer(upload_size, usage)
                .unwrap();
            let mem_reqs = device.get_buffer_requirements(&buffer);

            let upload_type = adapter
                .memory_types.iter().enumerate()
                .position(|(id, mem_type)| {
                    mem_reqs.type_mask & (1 << id) != 0
                        && mem_type
                            .properties
                            .contains(Properties::CPU_VISIBLE | Properties::COHERENT)
                }).unwrap()
                .into();

            memory = device.allocate_memory(upload_type, mem_reqs.size)
                .unwrap();
            device.bind_buffer_memory(&memory, 0, &mut buffer)
                .unwrap();
            size = mem_reqs.size;

            //copy image data into staging buffer
            let mapping = device.map_memory(&memory, Segment {
                offset: 0,
                size: Some(size)
            }).expect("Unable to map texture memory");
            
            for y in 0..height as usize {
                let data_source_slice =
                    &(**img)[y * (width as usize) * stride..(y + 1) * (width as usize) * stride];
                ptr::copy_nonoverlapping(
                    data_source_slice.as_ptr(),
                    mapping.offset(y as isize * row_pitch as isize),
                    data_source_slice.len(),
                );
            }
            device.unmap_memory(&memory);
        }

        (
            BufferState {
                memory: Some(memory),
                buffer: Some(buffer),
                device: device_ptr,
                size,
            },
            Dimensions { width, height },
            row_pitch,
            stride,
        )
    }

    //TODO: Should add a separate struct for vertex buffers to do all the staging and expose the fence,
    //like image state struct

    /// Creates new buffer for vertex data and copies the data with a staging buffer
    pub fn new_vertex_buffer<T>(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        data_source: &[T],
        memory_types: &[MemoryType],
        staging_pool: &mut B::CommandPool,
    ) -> Self
    where
        T: Copy,
    {
        let staging_buffer = BufferState::new(
            Rc::clone(&device_ptr),
            data_source,
            Usage::TRANSFER_SRC,
            memory_types,
            Properties::CPU_VISIBLE | Properties::COHERENT,
        );

        let vertex_buffer = BufferState::new_unmapped::<T>(
            Rc::clone(&device_ptr),
            data_source.len(),
            Usage::TRANSFER_DST | Usage::VERTEX,
            memory_types,
            Properties::DEVICE_LOCAL,
        );

        let upload_size = (data_source.len() * size_of::<T>()) as u64;
        let mut device = device_ptr.borrow_mut();

        unsafe {
            Self::copy_buffer(
                &mut device,
                &staging_buffer,
                &vertex_buffer,
                upload_size,
                staging_pool,
            );
        }

        vertex_buffer
    }

    //Creates a new buffer for index data and copies the data in, it uses a staging buffer
    pub fn new_index_buffer<T>(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        data_source: &[T],
        memory_types: &[MemoryType],
        staging_pool: &mut B::CommandPool,
    ) -> Self
    where
        T: Copy,
    {
        let staging_buffer = BufferState::new(
            Rc::clone(&device_ptr),
            data_source,
            Usage::TRANSFER_SRC,
            memory_types,
            Properties::CPU_VISIBLE | Properties::COHERENT,
        );

        let index_buffer = BufferState::new_unmapped::<T>(
            Rc::clone(&device_ptr),
            data_source.len(),
            Usage::TRANSFER_DST | Usage::INDEX,
            memory_types,
            Properties::DEVICE_LOCAL,
        );

        let upload_size = (data_source.len() * size_of::<T>()) as u64;
        let mut device = device_ptr.borrow_mut();

        unsafe {
            Self::copy_buffer(
                &mut device,
                &staging_buffer,
                &index_buffer,
                upload_size,
                staging_pool,
            );
        }

        index_buffer
    }

    //Creates a new uniform buffer. Doesn't map the memory
    pub fn new_uniform_buffer<T>(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        data_length: usize,
        memory_types: &[MemoryType],
    ) -> Self {
        BufferState::new_unmapped::<T>(
            Rc::clone(&device_ptr),
            data_length,
            Usage::UNIFORM,
            memory_types,
            Properties::CPU_VISIBLE | Properties::COHERENT,
        )
    }

    //endregion

    pub fn get_buffer(&self) -> &B::Buffer {
        self.buffer.as_ref().unwrap()
    }

    pub fn update_data<T>(
        &mut self,
        offset: u64,
        data_source: &[T],
    ) where
        T: Copy,
    {
        let device = &self.device.borrow().device;

        let stride = size_of::<T>();
        let upload_size = data_source.len() * stride;

        assert!(offset + upload_size as u64 <= self.size);
        let memory = self.memory.as_ref()
            .unwrap();

        unsafe {
            let mapping = device.map_memory(memory, Segment {
                offset,
                size: Some(self.size)
            }).unwrap();
            ptr::copy_nonoverlapping(data_source.as_ptr() as *const u8, mapping, upload_size);
            device.unmap_memory(memory);
        }
    }

    unsafe fn copy_buffer(
        device: &mut DeviceState<B>,
        src_buffer: &BufferState<B>,
        dst_buffer: &BufferState<B>,
        upload_size: u64,
        staging_pool: &mut B::CommandPool,
    ) {
        let transfered_buffer_fence = device
            .device.create_fence(false)
            .expect("Can't create fence");

        {
            let mut cmd_buffer = staging_pool.allocate_one(Level::Primary);
            cmd_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

            cmd_buffer.copy_buffer(
                &src_buffer.get_buffer(),
                &dst_buffer.get_buffer(),
                &[BufferCopy {
                    src: 0,
                    dst: 0,
                    size: upload_size,
                }],
            );

            cmd_buffer.finish();

            device.queues.queues[0]
                .submit_without_semaphores(iter::once(&cmd_buffer), Some(&transfered_buffer_fence));
        }

        device.device
            .wait_for_fence(&transfered_buffer_fence, !0)
            .unwrap();
    }
}

impl<B: Backend> Drop for BufferState<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        unsafe {
            device.destroy_buffer(self.buffer.take().unwrap());
            device.free_memory(self.memory.take().unwrap());
        }
    }
}

pub struct DepthBuffer<B: Backend> {
    pub depth_buffer: ImageState<B>,
    #[allow(dead_code)]
    depth_desc_pool: Option<B::DescriptorPool>,
}

impl<B: Backend> DepthBuffer<B> {
    pub fn new(
        device: Rc<RefCell<DeviceState<B>>>,
        adapter: &AdapterState<B>,
        dims: Dimensions<u32>
    ) -> Self {
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

        let mut depth_desc_pool = unsafe {
            device.borrow().device.create_descriptor_pool(
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
            )
        }.ok();

        let depth_desc = depth_desc.create_desc_set(depth_desc_pool.as_mut().unwrap());

        let mut staging_pool = unsafe {
            device.borrow().device.create_command_pool(
                device.borrow().queues.family,
                CommandPoolCreateFlags::empty(),
            )
        }.expect("Can't create Command Pool");

        let depth_buffer = ImageState::new_depth_image(
            depth_desc,
            dims,
            &adapter,
            &mut device.borrow_mut(),
            &mut staging_pool
        );

        depth_buffer.wait_for_transfer_completion();

        DepthBuffer {
            depth_buffer,
            depth_desc_pool
        }
    }

    pub fn stencil_support(device: Rc<RefCell<DeviceState<B>>>, format: Format) -> bool {
        let properties = device.borrow()
        .physical_device_format_properties(Some(format));
        // Format::D32SfloatS8Uint

        properties.linear_tiling.contains(ImageFeature::DEPTH_STENCIL_ATTACHMENT) || properties.optimal_tiling.contains(ImageFeature::DEPTH_STENCIL_ATTACHMENT)
    }
}
