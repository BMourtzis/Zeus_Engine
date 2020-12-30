use super::{
    buffer::BufferState,
    desc::{DescSet, DescSetWrite},
    device::DeviceState,
};
use gfx_hal::{
    adapter::MemoryType,
    buffer::{ SubRange, Usage},
    format::Format,
    memory::Properties,
    pso::{
        AttributeDesc, Descriptor, Element, VertexBufferDesc, VertexInputRate,
    },
    Backend,
};
use std::{
    cell::RefCell,
    cmp::Ordering,
    mem::size_of,
    rc::Rc
};

use zeus_core::math::{
    Matrix4,
    Vector2,
    Vector3,
    Vector4
};

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
                    SubRange {
                        offset: 0,
                        size: None
                    }
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
        self.desc.as_ref()
            .unwrap().get_layout()
    }
}

//TODO: make a_uv a vector3. why? 3d models?
//TODO: add texCoord
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub a_pos: Vector3,
    pub a_color: Vector4,
    pub a_uv: Vector2,
}

impl Vertex {
    pub fn get_vertex_buffer_description() -> [VertexBufferDesc; 1] {
        [
            VertexBufferDesc {
                binding: 0,
                stride: size_of::<Self>() as u32,
                rate: VertexInputRate::Vertex,
            }
        ]
    }

    pub fn get_attribute_description() -> [AttributeDesc; 3] {
        [
            AttributeDesc {
                binding: 0,
                location: 0,
                element: Element {
                    format: Format::Rgb32Sfloat,
                    offset: 0,
                }
            },
            AttributeDesc {
                binding: 0,
                location: 1,
                element: Element {
                    format: Format::Rgba32Sfloat,
                    offset: 12,
                }
            },
            AttributeDesc {
                binding: 0,
                location: 2,
                element: Element {
                    format: Format::Rg32Sfloat,
                    offset: 28,
                }
            },
        ]
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            return Ordering::Equal;
        }

        if self.a_pos.magn() > other.a_pos.magn() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Vertex {}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.a_pos == other.a_pos && self.a_color == other.a_color && self.a_uv == other.a_uv
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
