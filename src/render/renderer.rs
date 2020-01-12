extern crate gfx_backend_vulkan as back;

use gfx_hal::{
    device::Device,
    Backend,
    command::{
        Level,
        CommandBufferFlags,
        ClearValue,
        ClearColor,
        SubpassContents,
        CommandBuffer
    },
    pso::{
        Viewport,
        ColorValue,
        Rect,
        DescriptorSetLayoutBinding,
        DescriptorType,
        ShaderStageFlags,
        DescriptorRangeDesc,
        DescriptorPoolCreateFlags,
        PipelineStage
    },
    pool::{
        CommandPoolCreateFlags,
        CommandPool
    },
    buffer::Usage,
    window::{
        SwapImageIndex,
        Swapchain
    },
    queue::{
        Submission,
        CommandQueue
    }
};

use super::{
    image::ImageState,
    swapchain::SwapchainState,
    device::DeviceState,
    pass::RenderPassState,
    buffer::{
        FramebufferState,
        BufferState
    },
    backend::BackendState,
    utils::{
        Uniform,
        Color,
        Vertex
    },
    pipeline::PipelineState,
    desc::{
        DescSetLayout
    }
};

use std::{
    rc::Rc,
    cell::RefCell,
    io::Cursor,
    iter
};

use winit::event;

pub trait SurfaceTrait {
    #[cfg(feature = "gl")]
    fn get_contect_t(&self) -> &back::glutin::RawContext<back::glutin::PossiblyCurrent>;
}

impl SurfaceTrait for <back::Backend as Backend>::Surface {
    #[cfg(feature = "gl")]
    fn get_contect_t(self) -> &back::glutin::RawContext<back::glutin::PossiblyCurrent> {
        self.context()
    }
}

pub struct RendererState<B: Backend> {
    uniform_desc_pool: Option<B::DescriptorPool>,
    img_desc_pool: Option<B::DescriptorPool>,
    swapchain: Option<SwapchainState<B>>,
    device: Rc<RefCell<DeviceState<B>>>,
    pub backend: BackendState<B>,
    vertex_buffer: BufferState<B>,
    render_pass: RenderPassState<B>,
    uniform: Uniform<B>,
    pipeline: PipelineState<B>,
    framebuffer: FramebufferState<B>,
    viewport: Viewport,
    image: ImageState<B>,
    pub recreate_swapchain: bool,
    color: ColorValue,
    bg_color: ColorValue,
    cur_color: Color,
    cur_value: u32
}

impl<B:Backend> RendererState<B> {
    pub unsafe fn new (mut backend: BackendState<B>) -> Self {
        let device = Rc::new(RefCell::new(DeviceState::new(
            backend.adapter.adapter.take().unwrap(),
            &backend.surface
        )));

        let image_desc = DescSetLayout::new(
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

        let uniform_desc = DescSetLayout::new(
            Rc::clone(&device),
            vec![DescriptorSetLayoutBinding {
                binding: 0,
                ty: DescriptorType::UniformBuffer,
                count: 1,
                stage_flags: ShaderStageFlags::FRAGMENT,
                immutable_samplers: false
            }]
        );

        let mut img_desc_pool = device.borrow()
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
            ).ok();
        
        let mut uniform_desc_pool = device.borrow()
            .device.create_descriptor_pool(
                1,
                &[DescriptorRangeDesc {
                    ty: DescriptorType::UniformBuffer,
                    count: 1
                }],
                DescriptorPoolCreateFlags::empty()
            ).ok();
        
        let image_desc = image_desc.create_desc_set(img_desc_pool.as_mut().unwrap());
        let uniform_desc = uniform_desc.create_desc_set(uniform_desc_pool.as_mut().unwrap());

        // println!("Memory Types: {:?}", backend.adapter.memory_types);

        const IMAGE_LOGO: &[u8] = include_bytes!("../../data/logo.png");
        let img = image::load(Cursor::new(&IMAGE_LOGO[..]), image::PNG)
            .unwrap().to_rgba();

        let mut staging_pool = device.borrow()
            .device.create_command_pool(
                device.borrow().queues.family,
                CommandPoolCreateFlags::empty()
            ).expect("Can't create Command Pool");
        
        let image = ImageState::new(
            image_desc,
            &img,
            &backend.adapter,
            Usage::TRANSFER_SRC,
            &mut device.borrow_mut(),
            &mut staging_pool
        );

        let vertex_buffer = BufferState::new::<Vertex>(
            Rc::clone(&device),
            &QUAD,
            Usage::VERTEX,
            &backend.adapter.memory_types
        );

        let uniform = Uniform::new(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            &[0.1, 0.1, 0.1, 1.0],
            uniform_desc,
            0
        );

        image.wait_for_transfer_completion();

        device.borrow().device.destroy_command_pool(staging_pool);

        let mut swapchain = Some(SwapchainState::new(&mut backend, Rc::clone(&device)));

        let render_pass = RenderPassState::new(swapchain.as_ref().unwrap(), Rc::clone(&device));

        let framebuffer = FramebufferState::new(
            Rc::clone(&device),
            &render_pass,
            swapchain.as_mut().unwrap()
        );

        let pipeline = PipelineState::new(
            vec![image.get_layout(), uniform.get_layout()],
            render_pass.render_pass.as_ref().unwrap(),
            Rc::clone(&device)
        );

        let viewport = RendererState::create_viewport(swapchain.as_ref().unwrap());

        RendererState {
            backend,
            device,
            image,
            img_desc_pool,
            uniform_desc_pool,
            vertex_buffer,
            uniform,
            render_pass,
            pipeline,
            swapchain,
            framebuffer,
            viewport,
            recreate_swapchain: true,
            color: [0.1, 0.1, 0.1, 1.0],
            bg_color: [0.1, 0.1, 0.1, 1.0],
            cur_color: Color::Red,
            cur_value: 0
        }
    }

    pub fn recreate_swapchain(&mut self) {
        self.device.borrow().device.wait_idle().unwrap();

        self.swapchain.take().unwrap();

        self.swapchain = Some(unsafe {
            SwapchainState::new(&mut self.backend, Rc::clone(&self.device))
        });
        
        self.render_pass = unsafe {
            RenderPassState::new(self.swapchain.as_ref().unwrap(), Rc::clone(&self.device))
        };

        self.framebuffer = unsafe {
            FramebufferState::new(
                Rc::clone(&self.device), 
                &self.render_pass,
                self.swapchain.as_mut().unwrap()
            )
        };

        self.pipeline = unsafe {
            PipelineState::new(
                vec![self.image.get_layout(), self.uniform.get_layout()],
                self.render_pass.render_pass.as_ref().unwrap(),
                Rc::clone(&self.device)
            )
        };

        self.viewport = RendererState::create_viewport(self.swapchain.as_ref().unwrap());
        
    }

    //NOTE: the camera basically
    fn create_viewport(swapchain: &SwapchainState<B>) -> Viewport {
        Viewport {
            rect: Rect {
                x: 0,
                y: 0,
                w: swapchain.extent.width as i16,
                h: swapchain.extent.height as i16
            },
            depth: 0.0 .. 1.0
        }
    }

    pub fn draw(&mut self)
    where B::Surface: SurfaceTrait
    {
        if self.recreate_swapchain {
            self.recreate_swapchain();
            self.recreate_swapchain = true;
        }

        let sem_index = self.framebuffer.next_acq_pre_pair_index();

        let frame: SwapImageIndex = unsafe {
            let (acquire_semaphore, _) = self.framebuffer
                .get_frame_data(None, Some(sem_index)).1.unwrap();
            
            match self.swapchain.as_mut().unwrap()
                .swapchain.as_mut().unwrap()
                .acquire_image(!0, Some(acquire_semaphore), None)
            {
                Ok((i, _)) => i,
                Err(_) => {
                    self.recreate_swapchain = true;
                    return;
                }
            }
        };

        let (fid, sid) = self.framebuffer.get_frame_data(Some(frame as usize), Some(sem_index));

        let (framebuffer_fence, framebuffer, command_pool, command_buffers) = fid.unwrap();
        let (image_acquired, image_present) = sid.unwrap();

        unsafe {
            self.device.borrow()
                .device.wait_for_fence(framebuffer_fence, !0).unwrap();
            self.device.borrow()
                .device.reset_fence(framebuffer_fence).unwrap();
            command_pool.reset(false);

            let mut cmd_buffer = match command_buffers.pop() {
                Some(cmd_buffer) => cmd_buffer,
                None => command_pool.allocate_one(Level::Primary)
            };
            cmd_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);
            cmd_buffer.begin_render_pass(
                self.render_pass.render_pass.as_ref().unwrap(),
                framebuffer,
                self.viewport.rect,
                &[ClearValue {
                    color: ClearColor {
                        float32: self.bg_color,
                    },
                }],
                SubpassContents::Inline
            );

            cmd_buffer.set_viewports(0, &[self.viewport.clone()]);
            cmd_buffer.set_scissors(0, &[self.viewport.rect]);
            
            cmd_buffer.bind_graphics_pipeline(self.pipeline.pipeline.as_ref().unwrap());
            cmd_buffer.bind_vertex_buffers(0, Some((self.vertex_buffer.get_buffer(), 0)));
            cmd_buffer.bind_graphics_descriptor_sets(
                self.pipeline.pipeline_layout.as_ref().unwrap(),
                0,
                vec![
                    self.image.desc.set.as_ref().unwrap(),
                    self.uniform.desc.as_ref().unwrap().set.as_ref().unwrap()
                ],
                &[]
            );

            cmd_buffer.draw(0..6, 0..1);
            cmd_buffer.end_render_pass();
            cmd_buffer.finish();

            let submission = Submission {
                command_buffers: iter::once(&cmd_buffer),
                wait_semaphores: iter::once((&*image_acquired, PipelineStage::BOTTOM_OF_PIPE)),
                signal_semaphores: iter::once(&*image_present)
            };

            self.device.borrow_mut().queues.queues[0].submit(submission, Some(framebuffer_fence));
            command_buffers.push(cmd_buffer);

            if let Err(_) = self.swapchain.as_ref().unwrap()
                .swapchain.as_ref().unwrap()
                .present(
                    &mut self.device.borrow_mut().queues.queues[0],
                    frame,
                    Some(&*image_present)
                )
            {
                self.recreate_swapchain = true;
                return;
            }
        }
    }

    //TODO: inject this from the outside somehow
        //Try to move the viewport wit the input
        // self.viewport.rect.x += 1;
        // self.viewport.rect.y += 1;
    pub fn input(&mut self, kc: event::VirtualKeyCode) {
        match kc {
            event::VirtualKeyCode::Key0 => self.cur_value *= 10,
            event::VirtualKeyCode::Key1 => self.cur_value = self.cur_value * 10 + 1,
            event::VirtualKeyCode::Key2 => self.cur_value = self.cur_value * 10 + 2,
            event::VirtualKeyCode::Key3 => self.cur_value = self.cur_value * 10 + 3,
            event::VirtualKeyCode::Key4 => self.cur_value = self.cur_value * 10 + 4,
            event::VirtualKeyCode::Key5 => self.cur_value = self.cur_value * 10 + 5,
            event::VirtualKeyCode::Key6 => self.cur_value = self.cur_value * 10 + 6,
            event::VirtualKeyCode::Key7 => self.cur_value = self.cur_value * 10 + 7,
            event::VirtualKeyCode::Key8 => self.cur_value = self.cur_value * 10 + 8,
            event::VirtualKeyCode::Key9 => self.cur_value = self.cur_value * 10 + 9,
            event::VirtualKeyCode::R => {
                self.cur_value = 0;
                self.cur_color = Color::Red;
            },
            event::VirtualKeyCode::G => {
                self.cur_value = 0;
                self.cur_color = Color::Green;
            },
            event::VirtualKeyCode::B => {
                self.cur_value = 0;
                self.cur_color = Color::Blue;
            },
            event::VirtualKeyCode::A => {
                self.cur_value = 0;
                self.cur_color = Color::Alpha;
            },
            event::VirtualKeyCode::Return => {
                self.update_uniform_buffer(self.cur_value as f32);
                
                self.cur_value = 0;
            },
            event::VirtualKeyCode::C => {
                self.update_bg(self.cur_value as f32);

                self.cur_value = 0;
            },
            event::VirtualKeyCode::Up => {
                if self.cur_value < 255 {
                    self.cur_value += 1;
                }

                self.update_uniform_buffer(self.cur_value as f32);
            }
            event::VirtualKeyCode::Down => {
                if self.cur_value > 0 {
                    self.cur_value -= 1;
                }

                self.update_uniform_buffer(self.cur_value as f32);
            },
            event::VirtualKeyCode::Right => {
                if self.cur_value < 255 {
                    self.cur_value += 1;
                }

                self.update_bg(self.cur_value as f32);
            },
            event::VirtualKeyCode::Left => {
                if self.cur_value > 0 {
                    self.cur_value -= 1;
                }
                
                self.update_bg(self.cur_value as f32);
            }
            _ => return,
        }
        println!("Set {:?} color to: {} (Press enter/C to confirm)", self.cur_color, self.cur_value);
    }

    fn update_uniform_buffer(&mut self, value: f32) {
        match self.cur_color {
            Color::Red => self.color[0] = value / 255.0,
            Color::Green => self.color[1] = value / 255.0,
            Color::Blue => self.color[2] = value / 255.0,
            Color::Alpha => self.color[3] = value / 255.0,
        }

        self.uniform.buffer
            .as_mut().unwrap()
            .update_data(0, &self.color);
    }

    fn update_bg(&mut self, value: f32) {
        match self.cur_color {
            Color::Red => self.bg_color[0] = value / 255.0,
            Color::Green => self.bg_color[1] = value / 255.0,
            Color::Blue => self.bg_color[2] = value / 255.0,
            Color::Alpha => {
                println!("Alpha is not valid for the background!");
            }
        }
    }
}

impl<B: Backend> Drop for RendererState<B> {
    fn drop(&mut self) {
        self.device.borrow().device.wait_idle().unwrap();
        unsafe {
            self.device.borrow()
                .device.destroy_descriptor_pool(self.img_desc_pool.take().unwrap());
            self.device.borrow()
                .device.destroy_descriptor_pool(self.uniform_desc_pool.take().unwrap());
            self.swapchain.take();
        }
    }
}

const QUAD: [Vertex; 6] = [
    Vertex { a_pos: [ -1.0, 0.33], a_uv: [0.0, 1.0 ] },
    Vertex { a_pos: [ 0.0, 0.33], a_uv: [1.0, 1.0 ] },
    Vertex { a_pos: [ 0.0,-0.33], a_uv: [1.0, 0.0 ] },

    Vertex { a_pos: [-1.0, 0.33], a_uv: [0.0, 1.0 ] },
    Vertex { a_pos: [0.0, -0.33], a_uv: [1.0, 0.0 ] },
    Vertex { a_pos: [-1.0, -0.33], a_uv: [0.0, 0.0 ] },
];