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
        PipelineStage
    },
    pool::CommandPool,
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
    swapchain::SwapchainState,
    device::DeviceState,
    pass::RenderPassState,
    framebuffer::FramebufferState,
    backend::BackendState,
    model::{
        Color,
        Vertex
    },
    pipeline::PipelineState,
    camera::CameraState,
    obj::RenderObject
};

use crate::zeus_core::{
    time::Stopwatch,
    math::{
        vector::Vector2,
        vector::Vector3,
        matrix::Matrix4
    },
    input
};

use std::{
    rc::Rc,
    cell::RefCell,
    iter
};

use winit::event::VirtualKeyCode;

pub struct RendererState<B: Backend> {
    swapchain: Option<SwapchainState<B>>,
    object: Option<RenderObject<B>>,
    device: Rc<RefCell<DeviceState<B>>>,
    pub backend: BackendState<B>,
    render_pass: RenderPassState<B>,
    pipeline: PipelineState<B>,
    framebuffer: FramebufferState<B>,
    viewport: Viewport,
    timer: Stopwatch,
    camera: CameraState<B>,
    pub recreate_swapchain: bool,
    bg_color: ColorValue,
    cur_color: Color,
    cur_value: u32
}

impl<B:Backend> RendererState<B> {
    pub fn new (mut backend: BackendState<B>) -> Self {
        info!("New renderer state!");

        let device = Rc::new(RefCell::new(DeviceState::new(
            backend.adapter.adapter.take().unwrap(),
            &backend.surface
        )));

        let mut swapchain = Some(SwapchainState::new(&mut backend, Rc::clone(&device)));

        let mut camera = CameraState::new(
            swapchain.as_ref().unwrap().size as usize,
            Rc::clone(&device),
            &backend.adapter.memory_types
        );

        let viewport = RendererState::create_viewport(swapchain.as_ref().unwrap());

        camera.update_ubo(
            Matrix4::new().translate(0.0, 0.0, 0.0).rotate_y(180.0),
            Matrix4::new(),
            Matrix4::perspective(
                90.0_f32.to_radians(), 
                (viewport.rect.w/viewport.rect.h) as f32,
                0.1,
                1000.0
            )
        );

        camera.update_all_buffers();

        let render_pass = RenderPassState::new(swapchain.as_ref().unwrap(), Rc::clone(&device));

        let framebuffer = unsafe { FramebufferState::new(
            Rc::clone(&device),
            &render_pass,
            swapchain.as_mut().unwrap()
        )};

        let pipeline = PipelineState::empty(Rc::clone(&device));

        RendererState {
            backend,
            pipeline,
            device,
            object: None,
            render_pass,
            swapchain,
            framebuffer,
            viewport,
            timer: Stopwatch::new(),
            camera,
            recreate_swapchain: true,
            bg_color: [0.1, 0.1, 0.1, 1.0],
            cur_color: Color::Red,
            cur_value: 0
        }
    }

    pub fn load_level(&mut self) {
        info!("Load new level");

        let object = RenderObject::new(
            Rc::clone(&self.device),
            &self.backend.adapter,
            "./data/logo.png", 
            &VERTICES, 
            &INDICES
        );

        let mut layouts = Vec::new();
        self.camera.append_layout(&mut layouts);
        object.append_layout(&mut layouts);

        self.pipeline.new_pipeline(layouts, self.render_pass.render_pass.as_ref().unwrap());

        self.object = Some(object);
    }

    fn recreate_swapchain(&mut self) {
        debug!("Recreate Swapchain");
        self.device.borrow().device.wait_idle().unwrap();

        self.swapchain.take().unwrap();

        self.swapchain = Some(SwapchainState::new(&mut self.backend, Rc::clone(&self.device)));

        self.render_pass = RenderPassState::new(self.swapchain.as_ref().unwrap(), Rc::clone(&self.device));

        self.framebuffer = unsafe {
            FramebufferState::new(
                Rc::clone(&self.device), 
                &self.render_pass,
                self.swapchain.as_mut().unwrap()
            )
        };

        let mut layouts = Vec::new();
        self.camera.append_layout(&mut layouts);
        //NOTE: recreate swapchain is called only from draw after our check.
        self.object.as_ref().unwrap().append_layout(&mut layouts);

        self.pipeline.new_pipeline(layouts, self.render_pass.render_pass.as_ref().unwrap());

        self.viewport = RendererState::create_viewport(self.swapchain.as_ref().unwrap());
    }

    fn create_viewport(swapchain: &SwapchainState<B>) -> Viewport {
        Viewport {
            rect: Rect {
                x: 0,
                y: 0,
                w: swapchain.extent.width as i16,
                h: swapchain.extent.height as i16
            },
            depth: 0.0 .. 10.0
        }
    }

    pub fn draw(&mut self)
    {
        if self.pipeline.is_empty() {
            error!("No Pipeline!\nPlease load the level");
            return;
        }

        //NOTE: hard frame cap at 120 frames
        if self.timer.check_delta() < 8 {
            debug!("Skipped Draw Call");
            return;
        }

        debug!("Drawing Frame");
        debug!("Framerate: {}", self.timer.get_framerate());
        
        if self.recreate_swapchain {
            self.recreate_swapchain();
            self.recreate_swapchain = false;
        }

        //Get Delta
        self.timer.update_time();

        //Get Frame index
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

        //Updates
        self.call_updates(frame as usize);

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

            //Bind graphics and buffers
            cmd_buffer.bind_graphics_pipeline(self.pipeline.pipeline.as_ref().unwrap());

            self.object.as_ref().unwrap().bind_buffers(&mut cmd_buffer, 0);

            //TODO: Possible improvement, should save this item and update when needed.
            let mut desc_sets = Vec::new();
            self.camera.append_desc_set(&mut desc_sets);
            self.object.as_ref().unwrap().append_desc_set(&mut desc_sets);

            cmd_buffer.bind_graphics_descriptor_sets(
                self.pipeline.pipeline_layout.as_ref().unwrap(),
                0,
                desc_sets,
                &[]
            );

            cmd_buffer.draw_indexed(0..6, 0, 0..1);
            cmd_buffer.end_render_pass();
            cmd_buffer.finish();

            let submission = Submission {
                command_buffers: iter::once(&cmd_buffer),
                wait_semaphores: iter::once((&*image_acquired, PipelineStage::BOTTOM_OF_PIPE)),
                signal_semaphores: iter::once(&*image_present)
            };

            self.device.borrow_mut().queues.queues[0].submit(submission, Some(framebuffer_fence));
            command_buffers.push(cmd_buffer);

            if self.swapchain.as_ref().unwrap()
                .swapchain.as_ref().unwrap()
                .present(
                    &mut self.device.borrow_mut().queues.queues[0],
                    frame,
                    Some(&*image_present)
                ).is_err()
            {
                self.recreate_swapchain = true;
                return;
            }
        }
    }

    fn call_updates(&mut self, frame: usize) {
        self.update_camera(frame);
        self.update_colors();
    }

    fn update_camera(&mut self, frame: usize) {
        let step = 0.01_f32;

        let mut updated = false;

        if input::get_btn(VirtualKeyCode::W) {
            updated = true;
            self.camera.update_model(Matrix4::new().translate(
                0.0,
                0.0,
                step * self.timer.get_delta_f64(),
            ));
        }

        if input::get_btn(VirtualKeyCode::S) {
            updated = true;
            self.camera.update_model(Matrix4::new().translate(
                0.0,
                0.0,
                -step * self.timer.get_delta_f64(),
            ));
        }

        if input::get_btn(VirtualKeyCode::A) {
            updated = true;
            self.camera.update_model(Matrix4::new().translate(
                step * self.timer.get_delta_f64(),
                0.0,
                0.0
            ));
        }

        if input::get_btn(VirtualKeyCode::D) {
            updated = true;
            self.camera.update_model(Matrix4::new().translate(
                -step * self.timer.get_delta_f64(),
                0.0,
                0.0
            ));
        }

        if input::get_btn(VirtualKeyCode::Q) {
            updated = true;
            self.camera.update_model(Matrix4::new().rotate_y(
                0.1 * self.timer.get_delta_f64())
            );
        }

        if input::get_btn(VirtualKeyCode::E) {
            updated = true;
            self.camera.update_model(Matrix4::new().rotate_y(
                -0.1 * self.timer.get_delta_f64())
            );
        }

        if updated {
            self.camera.update_buffer(frame);
        }
    }

    fn update_colors(&mut self) {
        if input::get_btn(VirtualKeyCode::Key0) {
            self.cur_value *= 10
        }

        if input::get_btn(VirtualKeyCode::Key1) {
            self.cur_value = self.cur_value * 10 + 1
        }
        
        if input::get_btn(VirtualKeyCode::Key2) {
            self.cur_value = self.cur_value * 10 + 2
        }

        if input::get_btn(VirtualKeyCode::Key3) {
            self.cur_value = self.cur_value * 10 + 3
        }

        if input::get_btn(VirtualKeyCode::Key4) {
            self.cur_value = self.cur_value * 10 + 4
        }

        if input::get_btn(VirtualKeyCode::Key5) {
            self.cur_value = self.cur_value * 10 + 5
        }
        
        if input::get_btn(VirtualKeyCode::Key6) {
            self.cur_value = self.cur_value * 10 + 6
        }

        if input::get_btn(VirtualKeyCode::Key7) {
            self.cur_value = self.cur_value * 10 + 7
        }

        if input::get_btn(VirtualKeyCode::Key8) {
            self.cur_value = self.cur_value * 10 + 8
        }

        if input::get_btn(VirtualKeyCode::Key9) {
            self.cur_value = self.cur_value * 10 + 9
        }

        if input::get_btn(VirtualKeyCode::R) {
            self.cur_value = 0;
            self.cur_color = Color::Red;
        }

        if input::get_btn(VirtualKeyCode::G) {
            self.cur_value = 0;
            self.cur_color = Color::Green;
        }

        if input::get_btn(VirtualKeyCode::B) {
            self.cur_value = 0;
            self.cur_color = Color::Blue;
        }

        if input::get_btn(VirtualKeyCode::V) {
            self.cur_value = 0;
            self.cur_color = Color::Alpha;
        }

        if input::get_btn(VirtualKeyCode::Return) {
            self.update_uniform_buffer(self.cur_value as f32);

            self.cur_value = 0;
        }

        if input::get_btn(VirtualKeyCode::C) {
            self.update_bg(self.cur_value as f32);

            self.cur_value = 0;
        }

        if input::get_btn(VirtualKeyCode::Up) {
            if self.cur_value < 255 {
                self.cur_value += 1;
            }

            self.update_uniform_buffer(self.cur_value as f32);
        }

        if input::get_btn(VirtualKeyCode::Down) {
            if self.cur_value > 0 {
                self.cur_value -= 1;
            }

            self.update_uniform_buffer(self.cur_value as f32);
        }

        if input::get_btn(VirtualKeyCode::Right) {
            if self.cur_value < 255 {
                self.cur_value += 1;
            }

            self.update_bg(self.cur_value as f32);
        }

        if input::get_btn(VirtualKeyCode::Left) {
            if self.cur_value > 0 {
                self.cur_value -= 1;
            }
            
            self.update_bg(self.cur_value as f32);
        }
    }

    fn update_uniform_buffer(&mut self, value: f32) {
        self.object.as_mut().unwrap().update_color(&self.cur_color, value);
    }

    fn update_bg(&mut self, value: f32) {
        match self.cur_color {
            Color::Red => self.bg_color[0] = value / 255.0,
            Color::Green => self.bg_color[1] = value / 255.0,
            Color::Blue => self.bg_color[2] = value / 255.0,
            Color::Alpha => {
                info!("Alpha is not valid for the background!");
            }
        }
    }
}

impl<B: Backend> Drop for RendererState<B> {
    fn drop(&mut self) {
        self.device.borrow().device.wait_idle().unwrap();
        self.swapchain.take();
    }
}

const VERTICES: [Vertex; 4] = [
    Vertex { a_pos: Vector3 { x: 0.5, y: 0.33, z: 2.5 }, a_uv: Vector2 { x: 0.0, y: 1.0 }},
    Vertex { a_pos: Vector3 { x: -0.5, y: 0.33, z: 2.5 }, a_uv: Vector2 { x: 1.0, y: 1.0 }},
    Vertex { a_pos: Vector3 { x: -0.5, y: -0.33, z: 2.5 }, a_uv: Vector2 { x: 1.0, y: 0.0 }},
    Vertex { a_pos: Vector3 { x: 0.5, y: -0.33, z: 2.5 }, a_uv: Vector2 { x: 0.0, y: 0.0 }}
];

const INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];
