use gfx_hal::{
    buffer::{
        IndexBufferView, Usage, SubRange
    },
    command::CommandBuffer,
    device::Device,
    pool::CommandPoolCreateFlags,
    pso::{
        ColorValue, DescriptorPoolCreateFlags, DescriptorRangeDesc, DescriptorSetLayoutBinding,DescriptorType, ShaderStageFlags, ImageDescriptorType, BufferDescriptorType, BufferDescriptorFormat
    },
    Backend, IndexType,
};

use zeus_core::{
    math::{
        Vector2,
        Vector3,
        Vector4
    }, 
    time::Stopwatch
};

use super::{
    adapter::AdapterState,
    buffer::BufferState,
    desc::DescSetLayout,
    device::DeviceState,
    image::ImageState,
    model::{
        Color,
        Uniform,
        Vertex
    },
};

use tobj;

use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::Rc
};

//TODO: Should separate to Geometry, Material, Texture
//TODO: Should create pipeline as well
pub struct RenderObject<B: Backend> {
    device: Rc<RefCell<DeviceState<B>>>,
    color_desc_pool: Option<B::DescriptorPool>,
    texture_desc_pool: Option<B::DescriptorPool>,
    //
    
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    color: ColorValue,
    //
    image: Option<ImageState<B>>,
    color_uniform: Uniform<B>,
    vertex_buffer: BufferState<B>,
    index_buffer: Option<BufferState<B>>,
}

//TODO: add Some(Texture)
//Texture = image path and type
impl<B: Backend> RenderObject<B> {
    pub fn new_from_vertices(
        device: Rc<RefCell<DeviceState<B>>>,
        adapter: &AdapterState<B>,
        texture_path: &str,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> Self {
        let texture_desc = DescSetLayout::new(
            Rc::clone(&device),
            vec![
                DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: DescriptorType::Image {
                        ty: ImageDescriptorType::Sampled {
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
                    immutable_samplers: false,
                },
            ],
        );

        let color_desc = DescSetLayout::new(
            Rc::clone(&device),
            vec![DescriptorSetLayoutBinding {
                binding: 0,
                ty: DescriptorType::Buffer {
                    ty: BufferDescriptorType::Uniform,
                    format: BufferDescriptorFormat::Structured {
                        dynamic_offset: false
                    }
                },
                count: 1,
                stage_flags: ShaderStageFlags::FRAGMENT,
                immutable_samplers: false,
            }],
        );

        let mut texture_desc_pool = unsafe {
            device.borrow().device.create_descriptor_pool(
                1, //Number of sets
                &[
                    DescriptorRangeDesc {
                        ty: DescriptorType::Image {
                            ty: ImageDescriptorType::Sampled {
                                with_sampler: false
                            }
                        },
                        count: 1,
                    },
                    DescriptorRangeDesc {
                        ty: DescriptorType::Sampler,
                        count: 1,
                    },
                ],
                DescriptorPoolCreateFlags::empty(),
            )
        }.ok();

        let mut color_desc_pool = unsafe {
            device.borrow().device.create_descriptor_pool(
                1,
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

        let mut staging_pool = unsafe {
            device.borrow().device.create_command_pool(
                device.borrow().queues.family,
                CommandPoolCreateFlags::empty(),
            )
        }.expect("Can't create Command Pool");

        let texture_desc = texture_desc.create_desc_set(texture_desc_pool.as_mut().unwrap());
        let color_desc = color_desc.create_desc_set(color_desc_pool.as_mut().unwrap());

        let image = ImageState::new_texture(
            texture_desc,
            texture_path,
            &adapter,
            Usage::TRANSFER_SRC,
            &mut device.borrow_mut(),
            &mut staging_pool,
        );

        let vertex_buffer = BufferState::new_vertex_buffer(
            Rc::clone(&device),
            &vertices,
            &adapter.memory_types,
            &mut staging_pool,
        );

        let index_buffer = if !indices.is_empty() {
            Some(BufferState::new_index_buffer(
                Rc::clone(&device),
                &indices,
                &adapter.memory_types,
                &mut staging_pool,
            ))
        } else {
            None
        };

        let color_uniform = Uniform::new(
            Rc::clone(&device),
            &adapter.memory_types,
            &[1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32],
            color_desc,
            0,
        );

        //TODO: could save time by making it async
        //Should save the cmd_pool and destroy once done.
        image.wait_for_transfer_completion();

        unsafe {
            device.borrow().device
                .destroy_command_pool(staging_pool);
        }

        RenderObject {
            device,
            color_desc_pool,
            texture_desc_pool,
            //
            vertices: vertices.to_vec(),
            indices: if !indices.is_empty() {
                indices.to_vec()
            } else {
                Vec::new()
            },
            color: [1.0, 0.1, 0.1, 1.0],
            //
            image: Some(image),
            color_uniform,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn new_from_model(
        device: Rc<RefCell<DeviceState<B>>>,
        adapter: &AdapterState<B>,
        model_path: &str,
        texture_path: &str
    ) -> Self {
        let mut timer = Stopwatch::new();

        let obj = tobj::load_obj(model_path, false);

        debug!("Loaded file in {} ms", timer.get_current_delta());

        //TODO: return an error
        if let Err(err) = obj {
            info!("{:?}", err);
        }

        let (models, _materials) = obj.unwrap();

        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut unique_vertex_map: BTreeMap<Vertex, u32> = BTreeMap::new();

        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            
            for (_j, idx) in mesh.indices.iter().enumerate() {
                let index = *idx as usize;

                let vertex = Vertex {
                    a_pos: Vector3 {
                        x: mesh.positions[index * 3],
                        y: mesh.positions[index * 3 + 1],
                        z: mesh.positions[index * 3 + 2]
                    },
                    a_color: Vector4 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                        w: 1.0
                    },
                    a_uv: Vector2 {
                        x: mesh.texcoords[index * 2],
                        y: 1.0 - mesh.texcoords[index * 2 + 1]
                    }
                };

                if unique_vertex_map.get(&vertex).is_none() {
                    unique_vertex_map.insert(vertex, vertices.len() as u32);
                    vertices.push(vertex);
                }

                indices.push(*unique_vertex_map.get(&vertex).unwrap());
            }
        }
        
        timer.update_time();

        info!("Loaded Model with {} vertices and {} indices in {} ms", vertices.len(), indices.len(), timer.get_delta());

        Self::new_from_vertices(
            device,
            &adapter,
            texture_path,
            &vertices,
            &indices
        )
    }

    #[allow(dead_code)]
    pub fn get_color(&self) -> ColorValue {
        self.color
    }

    pub fn update_color(
        &mut self,
        color: &Color,
        value: f32,
    ) {
        //Update color state
        match color {
            Color::Red => self.color[0] = value / 255.0,
            Color::Green => self.color[1] = value / 255.0,
            Color::Blue => self.color[2] = value / 255.0,
            Color::Alpha => self.color[3] = value / 255.0,
        }

        //Update Buffer
        self.color_uniform
            .buffer.as_mut().unwrap()
            .update_data(0, &self.color);
    }

    #[allow(dead_code)]
    pub fn update_buffers(&mut self) {}

    #[allow(dead_code)]
    pub fn get_desc_set(&self) -> Vec<&B::DescriptorSet> {
        vec![
            self.image.as_ref().unwrap()
                .desc.set.as_ref().unwrap(),
            self.color_uniform
                .desc.as_ref().unwrap()
                .set.as_ref().unwrap(),
        ]
    }

    pub fn append_desc_set<'a>(
        &'a self,
        vec: &mut Vec<&'a B::DescriptorSet>,
    ) {
        vec.push(self.image.as_ref().unwrap().desc.set.as_ref().unwrap());
        vec.push(
            self.color_uniform
                .desc.as_ref().unwrap()
                .set.as_ref().unwrap(),
        );
    }

    #[allow(dead_code)]
    pub fn get_layout(&self) -> Vec<&B::DescriptorSetLayout> {
        vec![
            self.image.as_ref().unwrap()
                .get_layout(),
            self.color_uniform.get_layout(),
        ]
    }

    pub fn append_layout<'a>(
        &'a self,
        vec: &mut Vec<&'a B::DescriptorSetLayout>,
    ) {
        vec.push(self.image.as_ref().unwrap().get_layout());
        vec.push(self.color_uniform.get_layout());
    }

    pub unsafe fn bind_buffers(
        &self,
        cmd: &mut B::CommandBuffer,
        offset: u32,
    ) -> u32 {
        cmd.bind_vertex_buffers(
            offset, 
            Some((self.vertex_buffer.get_buffer(), SubRange::WHOLE))
        );
        
        if let Some(index_buffer) = &self.index_buffer {
            cmd.bind_index_buffer(IndexBufferView {
                buffer: index_buffer.get_buffer(),
                range: SubRange {
                    offset: 0,
                    size: None
                },
                index_type: IndexType::U32,
            })
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
                .device.destroy_descriptor_pool(self.color_desc_pool.take().unwrap());
        }
    }
}
