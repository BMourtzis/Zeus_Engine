use gfx_hal::{
    Backend,
    device::Device,
    pso::{
        self,
        ShaderStageFlags,
        EntryPoint,
        Specialization,
        GraphicsShaderSet,
        Primitive,
        Rasterizer,
        ColorMask,
        BlendState,
        GraphicsPipelineDesc,
        ColorBlendDesc,
        Face,
        FrontFace,
        PolygonMode
    },
    pass::{
        Subpass
    }
};

use std::{
    cell::RefCell,
    fs,
    rc::Rc,
    borrow
};

use super::{
    device::DeviceState,
    model::Vertex
};

const ENTRY_NAME: &str = "main";

pub struct PipelineState<B: Backend> {
    pub pipeline: Option<B::GraphicsPipeline>,
    pub pipeline_layout: Option<B::PipelineLayout>,
    device: Rc<RefCell<DeviceState<B>>>
}

impl<B: Backend> PipelineState<B> {
    pub fn new<IS>(
        desc_layouts: IS,
        render_pass: &B::RenderPass,
        device_ptr: Rc<RefCell<DeviceState<B>>>
    ) -> Self
    where
        IS: IntoIterator,
        IS::Item: borrow::Borrow<B::DescriptorSetLayout>
    {
        let mut pipeline = Self::empty(Rc::clone(&device_ptr));
        pipeline.new_pipeline(
            desc_layouts, 
            &render_pass
        );

        pipeline
    }

    pub fn empty(device_ptr: Rc<RefCell<DeviceState<B>>>) -> Self {
        PipelineState {
            pipeline: None,
            pipeline_layout: None,
            device: Rc::clone(&device_ptr)
        }
    }

    pub fn new_pipeline<IS>(
        &mut self,
        desc_layouts: IS,
        render_pass: &B::RenderPass
    )
    where
        IS: IntoIterator,
        IS::Item: borrow::Borrow<B::DescriptorSetLayout> 
    {
        let device = &self.device.borrow().device;
        let pipeline_layout = unsafe {
            device.create_pipeline_layout(desc_layouts, &[(ShaderStageFlags::VERTEX, 0 .. 8)])
        }.expect("Could not create pipeline layout");

        let pipeline = {
            let vs_module = create_shader_module::<B>(&device, "data/quad.vert", glsl_to_spirv::ShaderType::Vertex);
            let fs_module = create_shader_module::<B>(&device, "data/quad.frag", glsl_to_spirv::ShaderType::Fragment);

            let pipeline = {
                let (vs_entry, fs_entry) = (
                    EntryPoint::<B> {
                        entry: ENTRY_NAME,
                        module: &vs_module,
                        specialization: gfx_hal::spec_const_list![0.8f32]
                    },
                    EntryPoint::<B> {
                        entry: ENTRY_NAME,
                        module: &fs_module,
                        specialization: Specialization::default()
                    }
                );

                let shader_entries = GraphicsShaderSet {
                    vertex: vs_entry,
                    hull: None,
                    domain: None,
                    geometry: None,
                    fragment: Some(fs_entry)
                };

                let subpass = Subpass {
                    index: 0,
                    main_pass: render_pass
                };

                let rasterizer = Rasterizer {
                    polygon_mode: PolygonMode::Fill,
                    cull_face: Face::empty(),
                    front_face: FrontFace::Clockwise,
                    depth_clamping: false,
                    depth_bias: Option::None,
                    conservative: false
                };

                let mut pipeline_desc = GraphicsPipelineDesc::new(
                    shader_entries,
                    Primitive::TriangleList,
                    rasterizer,
                    &pipeline_layout,
                    subpass
                );

                pipeline_desc.blender.targets.push(ColorBlendDesc {
                    mask: ColorMask::ALL,
                    blend: Some(BlendState::ALPHA)
                });

                Vertex::inject_desc(&mut pipeline_desc);

                unsafe {
                    device.create_graphics_pipeline(&pipeline_desc, None)
                }.expect("Could not create graphics pipeline")
            };

            unsafe {
                device.destroy_shader_module(vs_module);
                device.destroy_shader_module(fs_module);
            }

            pipeline
        };

        self.pipeline = Some(pipeline);
        self.pipeline_layout = Some(pipeline_layout);
    }

    pub fn is_empty(&self) -> bool {
        if let Some(_) = &self.pipeline {
            false
        } else {
            true
        }
    }
}

impl<B: Backend> Drop for PipelineState<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        unsafe {
            device.destroy_graphics_pipeline(self.pipeline.take().unwrap());
            device.destroy_pipeline_layout(self.pipeline_layout.take().unwrap());
        }
    }
}


fn create_shader_module<B: Backend>(
    device: &B::Device,
    path: &str,
    shader_type: glsl_to_spirv::ShaderType
) -> B::ShaderModule {
    //Read a shader file and compile it to SPIR-V
    let glsl = fs::read_to_string(path).unwrap();
    let file = glsl_to_spirv::compile(&glsl, shader_type).unwrap();
    //Read SPIR-V and create shader module
    let spirv: Vec<u32> = pso::read_spirv(file).unwrap();
    
    unsafe {
        device.create_shader_module(&spirv)
    }.unwrap()
}