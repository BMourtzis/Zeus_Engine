use gfx_hal::{
    adapter::MemoryType,
    Backend,
    buffer::Usage,
    pso::Descriptor
};
use super::{
    buffer::BufferState,
    device::DeviceState,
    desc::{DescSet, DescSetWrite}
};
use std::{
    rc::Rc,
    cell::RefCell
};

pub struct Dimensions<T> {
    pub width: T,
    pub height: T
}

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Alpha
}

pub struct Uniform<B: Backend> {
    pub buffer: Option<BufferState<B>>,
    pub desc: Option<DescSet<B>>
}

impl<B: Backend> Uniform<B> {
    pub unsafe fn new<T>(
        device: Rc<RefCell<DeviceState<B>>>,
        memory_types: &[MemoryType],
        data: &[T],
        mut desc: DescSet<B>,
        binding: u32
    ) -> Self
    where T: Copy
    {
        let buffer = BufferState::new(
            Rc::clone(&device),
            &data,
            Usage::UNIFORM,
            memory_types
        );
        let buffer = Some(buffer);

        desc.write_to_state(
            vec![DescSetWrite {
                binding,
                array_offset: 0,
                descriptors: Some(Descriptor::Buffer(
                    buffer.as_ref().unwrap().get_buffer(),
                    None .. None
                ))
            }],
            &mut device.borrow_mut().device
        );

        Uniform {
            buffer,
            desc: Some(desc)
        }
    }

    pub fn get_layout(&self) -> &B::DescriptorSetLayout {
        self.desc.as_ref().unwrap().get_layout()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub a_pos: [f32; 2],
    pub a_uv: [f32; 2]
}