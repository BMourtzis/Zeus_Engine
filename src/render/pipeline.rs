use gfx_hal::{
    Backend,
    device::Device,
    format::Format,
    pso::{
        self,
        ShaderStageFlags,
        EntryPoint,
        Specialization,
        GraphicsShaderSet,
        Primitive,
        Rasterizer,
        ColorMask,
        VertexInputRate,
        BlendState,
        Element,
        AttributeDesc,
        GraphicsPipelineDesc,
        ColorBlendDesc,
        VertexBufferDesc
    },
    pass::{
        Subpass
    }
};

use std::{
    cell::RefCell,
    fs,
    rc::Rc,
    mem::size_of,
    borrow
};

use super::{
    device::DeviceState,
    utils::Vertex
};

const ENTRY_NAME: &str = "main";

pub struct PipelineState<B: Backend> {
    pub pipeline: Option<B::GraphicsPipeline>,
    pub pipeline_layout: Option<B::PipelineLayout>,
    device: Rc<RefCell<DeviceState<B>>>
}

impl<B: Backend> PipelineState<B> {
    pub unsafe fn new<IS>(
        desc_layouts: IS,
        render_pass: &B::RenderPass,
        device_ptr: Rc<RefCell<DeviceState<B>>>
    ) -> Self
    where
        IS: IntoIterator,
        IS::Item: borrow::Borrow<B::DescriptorSetLayout>
    {
        let device = &device_ptr.borrow().device;
        let pipeline_layout = device.create_pipeline_layout(desc_layouts, &[(ShaderStageFlags::VERTEX, 0 .. 8)])
            .expect("Could not create pipeline layout");

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

                // let rasterizer = Rasterizer {
                //     polygon_mode: PolygonMode::Line(State::Static(2.0)),
                //     cull_face: Face::empty(),
                //     front_face: FrontFace::CounterClockwise,
                //     depth_clamping: false,
                //     depth_bias: Option::None,
                //     conservative: false
                // };

                let mut pipeline_desc = GraphicsPipelineDesc::new(
                    shader_entries,
                    Primitive::TriangleList,
                    Rasterizer::FILL,
                    &pipeline_layout,
                    subpass
                );

                pipeline_desc.blender.targets.push(ColorBlendDesc {
                    mask: ColorMask::ALL,
                    blend: Some(BlendState::ALPHA)
                });

                Vertex::inject_desc(&mut pipeline_desc);

                device.create_graphics_pipeline(&pipeline_desc, None)
            };

            device.destroy_shader_module(vs_module);
            device.destroy_shader_module(fs_module);

            pipeline.unwrap()
        };

        PipelineState {
            pipeline: Some(pipeline),
            pipeline_layout: Some(pipeline_layout),
            device: Rc::clone(&device_ptr)
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


unsafe fn create_shader_module<B: Backend>(
    device: &B::Device,
    path: &str,
    shader_type: glsl_to_spirv::ShaderType
) -> B::ShaderModule {
    //Read a shader file and compile it to SPIR-V
    let glsl = fs::read_to_string(path).unwrap();
    let file = glsl_to_spirv::compile(&glsl, shader_type).unwrap();
    //Read SPIR-V and create shader module
    let spirv: Vec<u32> = pso::read_spirv(file).unwrap();
    device.create_shader_module(&spirv).unwrap()
}