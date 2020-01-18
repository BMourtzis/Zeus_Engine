use gfx_hal::{
    adapter::MemoryType,
    Backend,
    buffer::Usage,
    device::Device,
    image::{ Extent, ViewKind },
    memory::Properties,
    format::Swizzle,
    pool::{CommandPool, CommandPoolCreateFlags},
    command::{
        Level,
        CommandBufferFlags,
        BufferCopy,
        CommandBuffer
    },
    queue::CommandQueue
};
use std::{
    cell::RefCell,
    rc::Rc,
    mem::size_of,
    ptr,
    iter
};
use super::{
    adapter::AdapterState,
    constants::COLOR_RANGE,
    device::DeviceState,
    pass::RenderPassState,
    swapchain::SwapchainState,
    utils::Dimensions
};

pub struct BufferState<B: Backend> {
    memory: Option<B::Memory>,
    pub buffer: Option<B::Buffer>,
    device: Rc<RefCell<DeviceState<B>>>,
    size: u64
}

impl<B: Backend> BufferState<B> {
    ///Creates a new buffer and maps the memory
    pub unsafe fn new<T>(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        data_source: &[T],
        usage: Usage,
        memory_types: &[MemoryType],
        memory_properties: Properties
    ) -> Self {
        let buffer_state = BufferState::new_unmapped::<T>(Rc::clone(&device_ptr), data_source.len(), usage, memory_types, memory_properties);

        let device = &device_ptr.borrow().device;

        let stride = size_of::<T>();
        let upload_size = data_source.len() * stride;

        if let Some(mem) = &buffer_state.memory {
            let mapping = device.map_memory(mem, 0 .. buffer_state.size).unwrap();
            ptr::copy_nonoverlapping(data_source.as_ptr() as *const u8, mapping, upload_size);
            device.unmap_memory(mem);
        }

        buffer_state
    }

    ///Creates new Buffer state without mapping the memory
    pub unsafe fn new_unmapped<T>(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        data_length: usize,
        usage: Usage,
        memory_types: &[MemoryType],
        memory_properties: Properties
    ) -> Self {
        let memory: B::Memory;
        let mut buffer: B::Buffer;
        let size: u64;

        let stride = size_of::<T>();
        let upload_size = data_length * stride;

        {
            let device = &device_ptr.borrow().device;

            //TODO: Can we set sharing mode?
            buffer = device.create_buffer(upload_size as u64, usage).unwrap();
            // println!("{:?}", buffer);
            let mem_req = device.get_buffer_requirements(&buffer);

            let upload_type = memory_types.iter()
                .enumerate().position(|(id, mem_type)| {
                    mem_req.type_mask & (1 << id) != 0 
                        && mem_type.properties.contains(memory_properties)
                }).unwrap().into();
            
            memory = device.allocate_memory(upload_type, mem_req.size).unwrap();
            device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();
            size = mem_req.size;
        }

        BufferState {
            memory: Some(memory),
            buffer: Some(buffer),
            device: device_ptr,
            size
        }
    }

    pub fn get_buffer(&self) -> &B::Buffer {
        self.buffer.as_ref().unwrap()
    }

    pub fn update_data<T>(&mut self, offset: u64, data_source: &[T])
    where T: Copy
    {
        let device = &self.device.borrow().device;

        let stride = size_of::<T>();
        let upload_size = data_source.len() * stride;

        assert!(offset + upload_size  as u64 <= self.size);
        let memory = self.memory.as_ref().unwrap();

        unsafe {
            let mapping = device.map_memory(memory, offset .. self.size).unwrap();
            ptr::copy_nonoverlapping(data_source.as_ptr() as *const u8, mapping, upload_size);
            device.unmap_memory(memory);
        }
    }

    pub unsafe fn new_texture(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        device: &B::Device,
        img: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        adapter: &AdapterState<B>,
        usage: Usage
    ) -> (Self, Dimensions<u32>, u32, usize) {
        let (width, height) = img.dimensions();

        let row_alignment_mask = adapter.limits.optimal_buffer_copy_pitch_alignment as u32 - 1;
        let stride = 4usize;

        let row_pitch = (width * stride as u32 + row_alignment_mask) & !row_alignment_mask;
        let upload_size = (height * row_pitch) as u64;

        let memory: B::Memory;
        let mut buffer: B::Buffer;
        let size: u64;

        {
            buffer = device.create_buffer(upload_size, usage).unwrap();
            let mem_reqs = device.get_buffer_requirements(&buffer);

            let upload_type = adapter.memory_types
                .iter().enumerate()
                .position(|(id, mem_type)| {
                    mem_reqs.type_mask & (1 << id) != 0
                        && mem_type.properties.contains(Properties::CPU_VISIBLE | Properties::COHERENT)
                }).unwrap().into();

            memory = device.allocate_memory(upload_type, mem_reqs.size).unwrap();
            device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();
            size = mem_reqs.size;

            //copy image data into staging buffer
            let mapping = device.map_memory(&memory, 0 .. size).unwrap();
            for y in 0 .. height as usize {
                let data_source_slice = &(**img)
                    [y * (width as usize) * stride .. (y + 1) * (width as usize) * stride];
                ptr::copy_nonoverlapping(
                    data_source_slice.as_ptr(),
                    mapping.offset(y as isize * row_pitch as isize),
                    data_source_slice.len()
                );
            }
            device.unmap_memory(&memory);
        }

        (
            BufferState {
                memory: Some(memory),
                buffer: Some(buffer),
                device: device_ptr,
                size
            },
            Dimensions { width, height },
            row_pitch,
            stride
        )
    }

    //TODO: Should add a separate struct for vertex buffers to do all the staging and expose the fence,
    //like image struct
    pub unsafe fn new_vertex_buffer<T>(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        data_source: &[T],
        memory_types: &[MemoryType],
        staging_pool: &mut B::CommandPool
    ) -> Self {
        let staging_buffer = BufferState::new(
            Rc::clone(&device_ptr),
            data_source,
            Usage::TRANSFER_SRC,
            memory_types,
            Properties::CPU_VISIBLE | Properties::COHERENT
        );

        let vertex_buffer = BufferState::new_unmapped::<T>(
            Rc::clone(&device_ptr),
            data_source.len(),
            Usage::TRANSFER_DST | Usage::VERTEX,
            memory_types,
            Properties::DEVICE_LOCAL
        );

        let upload_size = (data_source.len() * size_of::<T>()) as u64;
        let mut device = device_ptr.borrow_mut();

        Self::copy_buffer(&mut device, &staging_buffer, &vertex_buffer, upload_size, staging_pool);

        vertex_buffer
    }

    pub unsafe fn new_index_buffer<T> (
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        data_source: &[T],
        memory_types: &[MemoryType],
        staging_pool: &mut B::CommandPool
    ) -> Self {
        let staging_buffer = BufferState::new(
            Rc::clone(&device_ptr),
            data_source,
            Usage::TRANSFER_SRC,
            memory_types,
            Properties::CPU_VISIBLE | Properties::COHERENT
        );

        let index_buffer = BufferState::new_unmapped::<T>(
            Rc::clone(&device_ptr),
            data_source.len(),
            Usage::TRANSFER_DST | Usage::INDEX,
            memory_types,
            Properties::DEVICE_LOCAL
        );

        let upload_size = (data_source.len() * size_of::<T>()) as u64;
        let mut device = device_ptr.borrow_mut();

        Self::copy_buffer(&mut device, &staging_buffer, &index_buffer, upload_size, staging_pool);

        index_buffer
    }

    unsafe fn copy_buffer(
        device: &mut DeviceState<B>,
        src_buffer: &BufferState<B>,
        dst_buffer: &BufferState<B>,
        upload_size: u64,
        staging_pool: &mut B::CommandPool
    ) {
        let transfered_buffer_fence = device.device.create_fence(false)
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
                    size: upload_size
                }]
            );

            cmd_buffer.finish();

            device.queues.queues[0].submit_without_semaphores(
                iter::once(&cmd_buffer),
                Some(&transfered_buffer_fence)
            );
        }

        device.device.wait_for_fence(&transfered_buffer_fence, !0)
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

pub struct FramebufferState<B: Backend> {
    framebuffers: Option<Vec<B::Framebuffer>>,
    framebuffer_fences: Option<Vec<B::Fence>>,
    command_pools: Option<Vec<B::CommandPool>>,
    command_buffer_lists: Vec<Vec<B::CommandBuffer>>,
    frame_images: Option<Vec<(B::Image, B::ImageView)>>,
    acquire_semaphores: Option<Vec<B::Semaphore>>,
    present_semaphores: Option<Vec<B::Semaphore>>,
    last_ref: usize,
    device: Rc<RefCell<DeviceState<B>>>
}

impl<B: Backend> FramebufferState<B> {
    pub unsafe fn new(
        device: Rc<RefCell<DeviceState<B>>>,
        render_pass: &RenderPassState<B>,
        swapchain: &mut SwapchainState<B>
    ) -> Self {
        let (frame_images, framebuffers) = {
            let extent = Extent {
                width: swapchain.extent.width as _,
                height: swapchain.extent.height as _,
                depth: 1
            };

            let pairs = swapchain.backbuffer.take()
                .unwrap().into_iter()
                .map(|image| {
                    let rtv = device.borrow()
                        .device.create_image_view(
                            &image,
                            ViewKind::D2,
                            swapchain.format,
                            Swizzle::NO,
                            COLOR_RANGE.clone()
                        ).unwrap();
                    (image, rtv)
                }).collect::<Vec<_>>();
            
            let fbos = pairs.iter()
                .map(|&(_, ref rtv)| {
                    device.borrow()
                        .device.create_framebuffer(
                            render_pass.render_pass.as_ref().unwrap(),
                            Some(rtv),
                            extent
                        ).unwrap()
                }).collect();
            (pairs, fbos)
        };

        let iter_count = if !frame_images.is_empty() {
            frame_images.len()
        }
        else {
            1
        };

        let mut fences: Vec<B::Fence> = vec![];
        let mut command_pools: Vec<_> = vec![];
        let mut command_buffer_lists = Vec::new();
        let mut acquire_semaphores: Vec<B::Semaphore> = vec![];
        let mut present_semaphores: Vec<B::Semaphore> = vec![];

        for _ in 0 .. iter_count {
            fences.push(device.borrow().device.create_fence(true).unwrap());
            command_pools.push(
                device.borrow()
                    .device.create_command_pool(
                        device.borrow().queues.family,
                        CommandPoolCreateFlags::empty()
                    ).expect("Can't create command pool")
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
            last_ref: 0
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
        sem_index: Option<usize>
    ) -> (
        Option<(
            &mut B::Fence,
            &mut B::Framebuffer,
            &mut B::CommandPool,
            &mut Vec<B::CommandBuffer>
        )>,
        Option<(&mut B::Semaphore, &mut B::Semaphore)> 
    ) {
        (
            if let Some(fid) = frame_id {
                Some((
                    &mut self.framebuffer_fences.as_mut().unwrap()[fid],
                    &mut self.framebuffers.as_mut().unwrap()[fid],
                    &mut self.command_pools.as_mut().unwrap()[fid],
                    &mut self.command_buffer_lists[fid]
                ))
            }
            else {
                None
            },
            if let Some(sid) = sem_index {
                Some((
                    &mut self.acquire_semaphores.as_mut().unwrap()[sid],
                    &mut self.present_semaphores.as_mut().unwrap()[sid]
                ))
            }
            else {
                None
            }
        )
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

            for (mut command_pool, command_buffer_list) in self.command_pools.take()
                .unwrap().into_iter()
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


//TODO: move the frame data into this struct
//TODO: timeline set up
// pub struct FrameData<B> {
//     sid: Option<(
//         &mut B::Fence,
//         &mut B::Framebuffer,
//         &mut B::CommandPool,
//         &mut Vec<B::CommandBuffer>
//     )>,
//     pid: Option<(
//         &mut B::Semaphore, 
//         &mut B::Semaphore
//     )> 
// }