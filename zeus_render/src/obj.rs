use gfx_hal::{
    device::Device,
    Backend,
    command::CommandBuffer,
    pso::{
        ColorValue,
        DescriptorType,
        ShaderStageFlags,
        DescriptorRangeDesc,
        DescriptorPoolCreateFlags,
        DescriptorSetLayoutBinding
    },
    pool::{
        CommandPoolCreateFlags
    },
    buffer::{
        IndexBufferView,
        Usage
    },
    IndexType
};

use super::{
    adapter::AdapterState,
    image::ImageState,
    device::DeviceState,
    buffer::BufferState,
    model::{
        Uniform,
        Color,
        Vertex
    },
    desc::{
        DescSetLayout
    }
};

use std::{
    fs,
    rc::Rc,
    cell::RefCell,
    io::Cursor
};

const DEFAULT_TEXTURE: &[u8] = include_bytes!("../../data/error.png");

//TODO: Should separate to Geometry, Material, Texture
//TODO: Should create pipeline as well
pub struct RenderObject<B: Backend> {
    device: Rc<RefCell<DeviceState<B>>>,
    color_desc_pool: Option<B::DescriptorPool>,
    texture_desc_pool: Option<B::DescriptorPool>,
    //
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    color: ColorValue,
    //
    image: Option<ImageState<B>>,
    color_uniform: Uniform<B>,
    vertex_buffer: BufferState<B>,
    index_buffer: Option<BufferState<B>>,
}

impl<B: Backend> RenderObject<B> {
    pub fn new(
        device: Rc<RefCell<DeviceState<B>>>,
        adapter: &AdapterState<B>,
        image_path: &str,
        vertices: &[Vertex],
        indices: &[u16],
    ) -> Self {
        let texture_desc = DescSetLayout::new(
            Rc::clone(&device),
            vec![
                DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: DescriptorType::SampledImage,
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false
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

        let color_desc = DescSetLayout::new(
            Rc::clone(&device),
            vec![DescriptorSetLayoutBinding {
                binding: 0,
                ty: DescriptorType::UniformBuffer,
                count: 1,
                stage_flags: ShaderStageFlags::FRAGMENT,
                immutable_samplers: false
            }]
        );

        let mut texture_desc_pool = unsafe {
            device.borrow()
            .device.create_descriptor_pool(
                1, //Number of sets
                &[
                    DescriptorRangeDesc {
                        ty: DescriptorType::SampledImage,
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

        let mut color_desc_pool = unsafe {
            device.borrow()
                .device.create_descriptor_pool(
                    1,
                    &[DescriptorRangeDesc {
                        ty: DescriptorType::UniformBuffer,
                        count: 1
                    }],
                    DescriptorPoolCreateFlags::empty()
                )
            }.ok();
        
        let mut staging_pool = unsafe {
            device.borrow()
                .device.create_command_pool(
                    device.borrow().queues.family,
                    CommandPoolCreateFlags::empty()
                )
            }.expect("Can't create Command Pool");
        
        let texture_desc = texture_desc.create_desc_set(texture_desc_pool.as_mut().unwrap());
        let color_desc = color_desc.create_desc_set(color_desc_pool.as_mut().unwrap());

        let image_bytes = match fs::read(image_path) {
            Ok(img) => img,
            Err(err) => {
                error!("{:?}", err);
                DEFAULT_TEXTURE.to_vec()
            }
        };
        
        let img = image::load(Cursor::new(&image_bytes[..]), image::PNG)
            .unwrap().to_rgba();

        let image = ImageState::new(
            texture_desc,
            &img,
            &adapter,
            Usage::TRANSFER_SRC,
            &mut device.borrow_mut(),
            &mut staging_pool
        );

        let vertex_buffer = BufferState::new_vertex_buffer(
            Rc::clone(&device), 
            &vertices, 
            &adapter.memory_types, 
            &mut staging_pool
        );

        let index_buffer = if indices.len() > 0 {
            Some(BufferState::new_index_buffer(
                Rc::clone(&device), 
                &indices, 
                &adapter.memory_types, 
                &mut staging_pool
            ))
        } else {
            None
        };

        let color_uniform = Uniform::new(
            Rc::clone(&device),
            &adapter.memory_types,
            &[1.0_f32, 0.1_f32, 0.1_f32, 1.0_f32],
            color_desc,
            0
        );

        //TODO: could save time by making it async
        //Should save the cmd_pool and destroy once done.
        image.wait_for_transfer_completion();

        unsafe {
            device.borrow().device.destroy_command_pool(staging_pool);
        }

        RenderObject {
            device,
            color_desc_pool,
            texture_desc_pool,
            //
            vertices: vertices.to_vec(),
            indices: if indices.len() > 0 {
                indices.to_vec()
            } else {
                Vec::new()
            },
            color: [1.0, 0.1, 0.1, 1.0],
            //
            image: Some(image),
            color_uniform,
            vertex_buffer,
            index_buffer
        }
    }

    pub fn get_color(&self) -> ColorValue {
        self.color
    }

    pub fn update_color(&mut self, color: &Color, value: f32) {
        //Update color state
        match color {
            Color::Red => self.color[0] = value / 255.0,
            Color::Green => self.color[1] = value / 255.0,
            Color::Blue => self.color[2] = value / 255.0,
            Color::Alpha => self.color[3] = value / 255.0,
        }

        //Update Buffer
        self.color_uniform.buffer
            .as_mut().unwrap()
            .update_data(0, &self.color);
    }

    pub fn update_buffers(&mut self) {

    }

    pub fn get_desc_set(&self) -> Vec<&B::DescriptorSet> {
        vec![
            self.image.as_ref().unwrap().desc.set.as_ref().unwrap(),
            self.color_uniform.desc.as_ref().unwrap().set.as_ref().unwrap()
        ]
    }

    pub fn append_desc_set<'a>(&'a self, vec: &mut Vec<&'a B::DescriptorSet>) {
        vec.push(self.image.as_ref().unwrap().desc.set.as_ref().unwrap());
        vec.push(self.color_uniform.desc.as_ref().unwrap().set.as_ref().unwrap());
    }

    pub fn get_layout(&self) -> Vec<&B::DescriptorSetLayout> {
        vec![
            self.image.as_ref().unwrap().get_layout(),
            self.color_uniform.get_layout()
        ]
    }

    pub fn append_layout<'a>(&'a self, vec: &mut Vec<&'a B::DescriptorSetLayout>) {
        vec.push(self.image.as_ref().unwrap().get_layout());
        vec.push(self.color_uniform.get_layout());
    }

    pub unsafe fn bind_buffers(&self, cmd: &mut B::CommandBuffer, offset: u32) -> u32 {
        cmd.bind_vertex_buffers(offset, Some((self.vertex_buffer.get_buffer(), 0)));
        if let Some(index_buffer) = &self.index_buffer {
            cmd.bind_index_buffer(
                IndexBufferView {
                    buffer: index_buffer.get_buffer(),
                    offset: 0, //TODO: what's the point of this offset?
                    index_type: IndexType::U16
                }
            )
        }

        offset + 1
    }
}

impl<B: Backend> Drop for RenderObject<B> {
    fn drop(&mut self) {
        self.device.borrow().device.wait_idle().unwrap();
        unsafe {
            self.device.borrow()
                .device.destroy_descriptor_pool(self.texture_desc_pool.take().unwrap());
            self.device.borrow()
                .device.destroy_descriptor_pool(self.color_desc_pool.take().unwrap())
        }
    }
}