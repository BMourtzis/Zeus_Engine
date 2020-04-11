use super::{
    buffer::BufferState,
    desc::{DescSet, DescSetWrite},
    device::DeviceState,
};
use gfx_hal::{
    adapter::MemoryType,
    buffer::Usage,
    format::Format,
    memory::Properties,
    pso::{
        AttributeDesc, Descriptor, Element, GraphicsPipelineDesc, VertexBufferDesc, VertexInputRate,
    },
    Backend,
};
use std::{cell::RefCell, mem::size_of, rc::Rc};

use zeus_core::math::{matrix::Matrix4, vector::Vector2, vector::Vector3};

pub struct Dimensions<T> {
    pub width: T,
    pub height: T,
}

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Alpha,
}

pub struct Uniform<B: Backend> {
    pub buffer: Option<BufferState<B>>,
    pub desc: Option<DescSet<B>>,
}

impl<B: Backend> Uniform<B> {
    pub fn new<T>(
        device: Rc<RefCell<DeviceState<B>>>,
        memory_types: &[MemoryType],
        data: &[T],
        mut desc: DescSet<B>,
        binding: u32,
    ) -> Self
    where
        T: Copy,
    {
        let buffer = BufferState::new(
            Rc::clone(&device),
            &data,
            Usage::UNIFORM,
            memory_types,
            Properties::CPU_VISIBLE | Properties::COHERENT,
        );
        let buffer = Some(buffer);

        desc.write_to_state(
            vec![DescSetWrite {
                binding,
                array_offset: 0,
                descriptors: Some(Descriptor::Buffer(
                    buffer.as_ref().unwrap().get_buffer(),
                    None..None,
                )),
            }],
            &mut device.borrow_mut().device,
        );

        Uniform {
            buffer,
            desc: Some(desc),
        }
    }

    pub fn get_layout(&self) -> &B::DescriptorSetLayout {
        self.desc.as_ref().unwrap().get_layout()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub a_pos: Vector3,
    pub a_uv: Vector2,
}

impl Vertex {
    pub fn inject_desc<B: Backend>(pipeline_desc: &mut GraphicsPipelineDesc<B>) {
        pipeline_desc
            .vertex_buffers
            .push(Self::get_vertex_buffer_description());

        pipeline_desc
            .attributes
            .extend(Self::get_attribute_description().iter());
    }

    fn get_vertex_buffer_description() -> VertexBufferDesc {
        VertexBufferDesc {
            binding: 0,
            stride: size_of::<Self>() as u32,
            rate: VertexInputRate::Vertex,
        }
    }

    fn get_attribute_description() -> [AttributeDesc; 2] {
        [
            AttributeDesc {
                location: 0,
                binding: 0,
                element: Element {
                    format: Format::Rgb32Sfloat,
                    offset: 0,
                },
            },
            AttributeDesc {
                location: 1,
                binding: 0,
                element: Element {
                    format: Format::Rg32Sfloat,
                    offset: 12,
                },
            },
        ]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct UniformBufferObject {
    pub model: Matrix4,
    pub view: Matrix4,
    pub proj: Matrix4,
}

impl UniformBufferObject {
    pub fn new() -> UniformBufferObject {
        let mut model = Matrix4::new();
        model[5] *= -1.0;

        UniformBufferObject {
            model,
            view: Matrix4::new(),
            proj: Matrix4::new(),
        }
    }
}
