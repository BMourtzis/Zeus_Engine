use gfx_hal::{
    adapter::MemoryType,
    buffer::SubRange,
    device::Device,
    pso::{
        Descriptor, DescriptorPoolCreateFlags, DescriptorRangeDesc, DescriptorSetLayoutBinding,DescriptorType, ShaderStageFlags, BufferDescriptorType, BufferDescriptorFormat
    },
    Backend,
};

use super::{
    buffer::BufferState,
    desc::{
        DescSet, DescSetLayout, DescSetWrite
    },
    device::DeviceState,
    model::UniformBufferObject,
};

use zeus_core::math::Matrix4;

use std::{
    cell::RefCell,
    mem::size_of,
    rc::Rc
};

pub struct CameraState<B: Backend> {
    pub buffers: Vec<Option<BufferState<B>>>,
    pub desc: Option<DescSet<B>>,
    device: Rc<RefCell<DeviceState<B>>>,
    camera_desc_pool: Option<B::DescriptorPool>,
    ubo: UniformBufferObject,
    has_updated_ubo: bool
}

impl<B: Backend> CameraState<B> {
    pub fn new(
        size: usize,
        device: Rc<RefCell<DeviceState<B>>>,
        memory_types: &[MemoryType],
    ) -> Self {
        let binding = 0;

        //Create descriptors
        let camera_desc = DescSetLayout::new(
            Rc::clone(&device),
            vec![DescriptorSetLayoutBinding {
                binding,
                ty: DescriptorType::Buffer {
                    ty: BufferDescriptorType::Uniform,
                    format: BufferDescriptorFormat::Structured {
                        dynamic_offset: false
                    }
                },
                count: 1,
                stage_flags: ShaderStageFlags::VERTEX,
                immutable_samplers: false,
            }],
        );

        let mut camera_desc_pool = unsafe {
            device.borrow().device.create_descriptor_pool(
                size,
                &[DescriptorRangeDesc {
                    ty: DescriptorType::Buffer {
                        ty: BufferDescriptorType::Uniform,
                        format: BufferDescriptorFormat::Structured {
                            dynamic_offset: false
                        }
                    },
                    count: 1,
                }],
                DescriptorPoolCreateFlags::empty(),
            )
        }.ok();

        let mut camera_desc = camera_desc.create_desc_set(
            camera_desc_pool.as_mut().unwrap()
        );

        //Create buffers
        let mut buffers = Vec::default();
        for _i in 0..size {
            buffers.push(Some(
                BufferState::new_uniform_buffer::<UniformBufferObject>(
                    Rc::clone(&device),
                    1,
                    memory_types,
                ),
            ));
        }

        //Create desc set
        let mut desc_set_write = Vec::default();
        for buf in &buffers {
            desc_set_write.push(DescSetWrite {
                binding,
                array_offset: 0,
                descriptors: Some(Descriptor::Buffer(
                    buf.as_ref().unwrap().get_buffer(),
                    SubRange {
                        offset: 0,
                        size: Some(size_of::<UniformBufferObject>() as u64)
                    }
                )),
            });
        }

        camera_desc.write_to_state(desc_set_write, &mut device.borrow_mut().device);

        let ubo = UniformBufferObject::new();

        CameraState {
            buffers,
            desc: Some(camera_desc),
            device,
            camera_desc_pool,
            ubo,
            has_updated_ubo: false
        }
    }

    //Methods
    #[allow(dead_code)]
    pub fn get_ubo(&self) -> UniformBufferObject {
        self.ubo
    }

    pub fn update_ubo(
        &mut self,
        model: Matrix4,
        view: Matrix4,
        proj: Matrix4,
    ) {
        self.ubo.model = model;
        self.ubo.view = view;
        self.ubo.proj = proj;

        self.has_updated_ubo = true;
    }

    #[allow(dead_code)]
    pub fn get_model(&self) -> Matrix4 {
        self.ubo.model
    }

    pub fn update_model(
        &mut self,
        update: Matrix4,
    ) {
        self.ubo.model = update * self.ubo.model;
        self.has_updated_ubo = true;
    }

    #[allow(dead_code)]
    pub fn get_view(&self) -> Matrix4 {
        self.ubo.view
    }

    #[allow(dead_code)]
    pub fn update_view(
        &mut self,
        update: Matrix4,
    ) {
        self.ubo.view = update * self.ubo.view;
        self.has_updated_ubo = true;
    }

    #[allow(dead_code)]
    pub fn get_proj(&self) -> Matrix4 {
        self.ubo.proj
    }

    #[allow(dead_code)]
    pub fn update_proj(&mut self, update: Matrix4) {
        self.ubo.proj = update * self.ubo.proj;
        self.has_updated_ubo = true;
    }

    pub fn set_proj(&mut self, update: Matrix4) {
        self.ubo.proj = update;
        self.has_updated_ubo = true;
    }

    //Updates specific buffer with the new data.
    pub fn update_buffer(&mut self, idx: usize,) {
        if self.has_updated_ubo {
            debug!("ubo: {}", self.ubo.model);

            self.buffers[idx].as_mut().unwrap()
                .update_data(0, &[self.ubo]);
        }
    }

    pub fn update_all_buffers(&mut self) {
        if self.has_updated_ubo {
            for buffer in self.buffers.iter_mut() {
                buffer.as_mut().unwrap().update_data(0, &[self.ubo]);
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_desc_set(&self) -> &B::DescriptorSet {
        self.desc.as_ref().unwrap().set.as_ref().unwrap()
    }

    pub fn append_desc_set<'a>(
        &'a self,
        vec: &mut Vec<&'a B::DescriptorSet>,
    ) {
        vec.push(self.desc.as_ref().unwrap().set.as_ref().unwrap())
    }

    #[allow(dead_code)]
    pub fn get_layout(&self) -> &B::DescriptorSetLayout {
        self.desc.as_ref().unwrap().get_layout()
    }

    pub fn append_layout<'a>(
        &'a self,
        vec: &mut Vec<&'a B::DescriptorSetLayout>,
    ) {
        vec.push(self.desc.as_ref().unwrap().get_layout())
    }
}

impl<B: Backend> Drop for CameraState<B> {
    fn drop(&mut self) {
        self.device.borrow().device.wait_idle().unwrap();
        unsafe {
            self.device
                .borrow()
                .device
                .destroy_descriptor_pool(self.camera_desc_pool.take().unwrap());
        }
    }
}
