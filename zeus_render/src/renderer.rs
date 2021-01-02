extern crate gfx_backend_vulkan as back;

use gfx_hal::{
    command::{
        ClearColor, ClearValue, CommandBuffer, CommandBufferFlags, ClearDepthStencil, Level, SubpassContents
    },
    device::Device,
    pool::CommandPool,
    pso::{
        ColorValue, Rect, Viewport
    },
    queue::{
        CommandQueue, Submission
    },
    window::{
        Extent2D, PresentationSurface
    },
    Backend,
};

use super::{
    backend::BackendState,
    buffer::DepthBuffer,
    camera::CameraState,
    constants::{
        DEPTH_IMAGE_FORMAT, DIMS
    },
    device::DeviceState,
    error::NoLevelLoadedError,
    framebuffer::FramebufferState,
    model::{
        Color, Dimensions
    },
    obj::RenderObject,
    pass::RenderPassState,
    pipeline::PipelineState,
    swapchain::SwapchainState,
};

use crate::zeus_core::{
    input,
    math::Matrix4,
    time::Stopwatch,
};

use std::{
    cell::RefCell,
    iter,
    rc::Rc
};

use winit::event::VirtualKeyCode;

pub struct RendererState<B: Backend> {
    swapchain: SwapchainState<B>,
    object: Option<RenderObject<B>>,
    device: Rc<RefCell<DeviceState<B>>>,
    pub backend: BackendState<B>,
    render_pass: RenderPassState<B>,
    pipeline: PipelineState<B>,
    framebuffer: FramebufferState<B>,
    viewport: Viewport,
    timer: Stopwatch,
    camera: CameraState<B>,
    window_dimensions: Extent2D,
    pub recreate_swapchain: bool,
    bg_color: ColorValue,
    cur_color: Color,
    cur_value: u32,
    depth_buffer: Option<DepthBuffer<B>>
}

impl<B: Backend> RendererState<B> {
    pub fn new(mut backend: BackendState<B>) -> Self {
        info!("New renderer state!");

        let device = Rc::new(RefCell::new(DeviceState::new(
            backend.adapter.adapter.take().expect("Backend Adapter is empty!"),
            &backend.surface,
        )));

        let window_dimensions = Extent2D {
            width: DIMS.width,
            height: DIMS.height
        };

        let swapchain = SwapchainState::new(
            &mut backend,
            Rc::clone(&device),
            window_dimensions
        );

        let mut camera = CameraState::new(
            swapchain.size as usize,
            Rc::clone(&device),
            &backend.adapter.memory_types,
        );

        let viewport = RendererState::create_viewport(&swapchain);

        let mut camera_model = Matrix4::new_rotation_y(90.0);
        camera_model.translate(0.0, -0.5, -1.0);

        camera.update_ubo(
            camera_model,
            Matrix4::new(),
            Matrix4::perspective(
                90.0_f32.to_radians(),
                (viewport.rect.w / viewport.rect.h) as f32,
                0.1,
                1000.0,
            ),
        );

        camera.update_all_buffers();

        let render_pass = RenderPassState::new(&swapchain, Rc::clone(&device));

        let depth_buffer = if DepthBuffer::stencil_support(Rc::clone(&device), DEPTH_IMAGE_FORMAT) {
            Some(DepthBuffer::new(
                Rc::clone(&device),
                &backend.adapter,
                Dimensions {
                    width: window_dimensions.width,
                    height: window_dimensions.height
                }
            ))
        } else {
            None
        };

        let framebuffer = unsafe {
            FramebufferState::new(
                Rc::clone(&device),
                swapchain.size,
            )
        };

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
            window_dimensions,
            recreate_swapchain: true,
            bg_color: [0.0, 0.0, 0.0, 1.0],
            cur_color: Color::Red,
            cur_value: 0,
            depth_buffer
        }
    }

    pub fn load_level(&mut self) {
        info!("Load new level");

        // let object = RenderObject::new_from_vertices(
        //     Rc::clone(&self.device),
        //     &self.backend.adapter,
        //     "./data/textures/viking_room.png",
        //     &VERTICES,
        //     &INDICES,
        // );

        let object = RenderObject::new_from_model(
            Rc::clone(&self.device),
            &self.backend.adapter,
            "./data/models/viking_room.obj",
            "./data/textures/viking_room.png"
        );

        let mut layouts = Vec::new();
        self.camera.append_layout(&mut layouts);
        object.append_layout(&mut layouts);

        self.pipeline.new_pipeline(
            layouts,
            self.render_pass.render_pass.as_ref().unwrap()
        );

        self.object = Some(object);

        self.recreate_swapchain();
    }

    fn recreate_swapchain(&mut self) {
        debug!("Recreate Swapchain");
        self.device.borrow().device.wait_idle()
            .expect("Device is empty!");

        self.swapchain = SwapchainState::new(
            &mut self.backend,
            Rc::clone(&self.device),
            self.window_dimensions
        );

        self.render_pass = RenderPassState::new(
            &self.swapchain,
            Rc::clone(&self.device)
        );

        self.depth_buffer = if DepthBuffer::stencil_support(
            Rc::clone(&self.device),
            DEPTH_IMAGE_FORMAT
        ) {
            Some(DepthBuffer::new(
                Rc::clone(&self.device),
                &self.backend.adapter,
                Dimensions {
                    width: self.window_dimensions.width,
                    height: self.window_dimensions.height
                }
            ))
        } else {
            None
        };

        self.framebuffer = unsafe {
            FramebufferState::new(
                Rc::clone(&self.device),
                self.swapchain.size
            )
        };

        let mut layouts = Vec::new();
        self.camera.append_layout(&mut layouts);
        //NOTE: recreate swapchain is called only from draw after our check.
        self.object.as_ref()
            .expect("Render Object is empty!")
            .append_layout(&mut layouts);

        self.pipeline.new_pipeline(
            layouts,
            self.render_pass.render_pass.as_ref()
                .expect("Render Pass is empty!")
        );

        self.viewport = RendererState::create_viewport(
            &self.swapchain
        );
    }

    fn create_viewport(swapchain: &SwapchainState<B>) -> Viewport {
        Viewport {
            rect: Rect {
                x: 0,
                y: 0,
                w: swapchain.extent.width as i16,
                h: swapchain.extent.height as i16,
            },
            depth: -1.0 .. 1.0,
        }
    }

    pub fn draw(&mut self) -> Result<(), NoLevelLoadedError> {
        if self.pipeline.is_empty() {
            return Err(NoLevelLoadedError{
                message: "No level is loaded!!/nPlease a level before you try to draw the scene".to_string()
            })
        }

        //NOTE: hard frame cap at 1000 frames
        if self.timer.check_delta() < 1 {
            debug!("Skipped Draw Call");
            return Ok(());
        }

        debug!("Drawing Frame");
        debug!("Framerate: {}", self.timer.get_framerate());

        if self.recreate_swapchain {
            self.recreate_swapchain();
            self.recreate_swapchain = false;
        }

        //Get Delta
        self.timer.update_time();

        let surface_image = unsafe {
            match self.backend.surface.acquire_image(!0) {
                Ok((i, _)) => i,
                Err(_) => {
                    self.recreate_swapchain = true;
                    return Ok(())
                }
            }
        };
        
        let attachments = vec![
            std::borrow::Borrow::borrow(&surface_image),
            self.depth_buffer.as_ref().unwrap()
                .depth_buffer.get_image_view().as_ref().unwrap()
        ];

        let framebuffer = unsafe {
            self.device.borrow()
                .device.create_framebuffer(
                    self.render_pass.render_pass.as_ref().unwrap(),
                    attachments,
                    self.swapchain.extent
                ).expect("Could not create framebuffer")
        };

        let frame_idx = (self.swapchain.frame_index % self.swapchain.size) as usize;
        self.swapchain.frame_index += 1;

        //Updates
        self.update_camera();
        self.update_colors();
        self.camera.update_buffer(frame_idx);

        let framedata = self.framebuffer.get_frame_data(frame_idx);

        unsafe {
            framedata.cmd_pool.reset(false);

            let mut cmd_buffer = match framedata.cmd_buffers.pop() {
                Some(cmd_buffer) => cmd_buffer,
                None => framedata.cmd_pool.allocate_one(Level::Primary),
            };

            cmd_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);
            if cfg!(debug_assertions) {
                cmd_buffer.begin_debug_marker("setup", 0);
            }
            cmd_buffer.set_viewports(0, &[self.viewport.clone()]);
            cmd_buffer.set_scissors(0, &[self.viewport.rect]);
            cmd_buffer.bind_graphics_pipeline(
                self.pipeline.pipeline.as_ref()
                    .expect("Pipeline is empty!")
            );
            if cfg!(debug_assertions) {
                cmd_buffer.end_debug_marker();
            }
            

            cmd_buffer.begin_render_pass(
                self.render_pass.render_pass.as_ref()
                    .expect("Render Pass is empty!"),
                &framebuffer,
                self.viewport.rect,
                &[ClearValue {
                    color: ClearColor {
                        float32: self.bg_color,
                    },
                }, ClearValue {
                    depth_stencil: ClearDepthStencil {
                        depth: 1.0_f32,
                        stencil: 0_u32
                    }
                }],
                SubpassContents::Inline,
            );

            self.object.as_ref().unwrap()
                .bind_buffers(&mut cmd_buffer, 0);

            //TODO: Possible improvement, should save this item and update when needed.
            let mut desc_sets = Vec::new();
            self.camera.append_desc_set(&mut desc_sets);
            self.object.as_ref().unwrap()
                .append_desc_set(&mut desc_sets);

            cmd_buffer.bind_graphics_descriptor_sets(
                self.pipeline.pipeline_layout.as_ref().expect("Pipeline Layout is empty!"),
                0,
                desc_sets,
                &[],
            );

            cmd_buffer.draw_indexed(0..self.object.as_ref().unwrap().indices.len() as u32, 0, 0..1);
            cmd_buffer.end_render_pass();
            if cfg!(debug_assertions) {
                cmd_buffer.insert_debug_marker("done", 0);
            }
            cmd_buffer.finish();

            let submission = Submission {
                command_buffers: iter::once(&cmd_buffer),
                wait_semaphores: None,
                signal_semaphores: iter::once(&*framedata.present_sem),
            };

            self.device.borrow_mut().queues.queues[0].submit(submission, None);
            framedata.cmd_buffers.push(cmd_buffer);

            //present frame
            if let Err(_) = self.device.borrow_mut().queues.queues[0].present(
                &mut *self.backend.surface,
                surface_image,
                Some(framedata.present_sem)
            ) {
                self.recreate_swapchain = true;
            }

            self.device.borrow().device.destroy_framebuffer(framebuffer);
        }

        Ok(())
    }

    pub fn update_window_dimensions(&mut self, width: u32, height: u32) {
        self.window_dimensions = Extent2D {
            width,
            height
        };

        self.recreate_swapchain = true;
    }

    pub fn update_camera_rotation(&mut self, x: f64, _y: f64) {
        let step = -0.5_f32;

        self.camera
            .update_model(Matrix4::new_rotation_y(x as f32 * step * self.timer.get_delta_f32()));
    }

    //TODO: Need to update the 
    // pub fn update_fov(&mut self, delta: f32) {
    //     let step = 0.05_f32;

    //     // self.camera.update_proj(Matrix4::perspective(
    //     //     i * step * self.timer.get_delta_f64(),
    //     //     (self.viewport.rect.w / self.viewport.rect.h) as f32,
    //     //     0.1,
    //     //     1000.0,
    //     // ));
    // }

    fn update_camera(&mut self) {
        let step = 0.005_f32;

        if input::is_btn_down(VirtualKeyCode::W) {
            self.camera.update_model(Matrix4::new_traslation(
                0.0,
                0.0,
                step * self.timer.get_delta_f32(),
            ));
        }

        if input::is_btn_down(VirtualKeyCode::S) {
            self.camera.update_model(Matrix4::new_traslation(
                0.0,
                0.0,
                -step * self.timer.get_delta_f32(),
            ));
        }

        if input::is_btn_down(VirtualKeyCode::A) {
            self.camera.update_model(Matrix4::new_traslation(
                step * self.timer.get_delta_f32(),
                0.0,
                0.0,
            ));
        }

        if input::is_btn_down(VirtualKeyCode::D) {
            self.camera.update_model(Matrix4::new_traslation(
                -step * self.timer.get_delta_f32(),
                0.0,
                0.0,
            ));
        }

        if input::is_btn_down(VirtualKeyCode::R) {
            self.camera.update_model(Matrix4::new_traslation(
                0.0,
                -step * self.timer.get_delta_f32(),
                0.0,
            ));
        }

        if input::is_btn_down(VirtualKeyCode::F) {
            self.camera.update_model(Matrix4::new_traslation(
                0.0,
                step * self.timer.get_delta_f32(),
                0.0,
            ));
        }

        if input::is_btn_down(VirtualKeyCode::Q) {
            self.camera
                .update_model(Matrix4::new_rotation_y(0.1 * self.timer.get_delta_f32()));
        }

        if input::is_btn_down(VirtualKeyCode::E) {
            self.camera
                .update_model(Matrix4::new_rotation_y(-0.1 * self.timer.get_delta_f32()));
        }

        if input::is_btn_down(VirtualKeyCode::J) {
            self.camera.set_proj(Matrix4::perspective(
                90.0_f32.to_radians(),
                (self.viewport.rect.w / self.viewport.rect.h) as f32,
                0.1,
                1000.0,
            ));
        }

        if input::is_btn_down(VirtualKeyCode::K) {
            self.camera.set_proj(Matrix4::perspective(
                120.0_f32.to_radians(),
                (self.viewport.rect.w / self.viewport.rect.h) as f32,
                0.1,
                1000.0,
            ));
        }
    }

    fn update_colors(&mut self) {
        if input::is_btn_down(VirtualKeyCode::Key0) {
            self.cur_value *= 10
        }

        if input::is_btn_down(VirtualKeyCode::Key1) {
            self.cur_value = self.cur_value * 10 + 1
        }

        if input::is_btn_down(VirtualKeyCode::Key2) {
            self.cur_value = self.cur_value * 10 + 2
        }

        if input::is_btn_down(VirtualKeyCode::Key3) {
            self.cur_value = self.cur_value * 10 + 3
        }

        if input::is_btn_down(VirtualKeyCode::Key4) {
            self.cur_value = self.cur_value * 10 + 4
        }

        if input::is_btn_down(VirtualKeyCode::Key5) {
            self.cur_value = self.cur_value * 10 + 5
        }

        if input::is_btn_down(VirtualKeyCode::Key6) {
            self.cur_value = self.cur_value * 10 + 6
        }

        if input::is_btn_down(VirtualKeyCode::Key7) {
            self.cur_value = self.cur_value * 10 + 7
        }

        if input::is_btn_down(VirtualKeyCode::Key8) {
            self.cur_value = self.cur_value * 10 + 8
        }

        if input::is_btn_down(VirtualKeyCode::Key9) {
            self.cur_value = self.cur_value * 10 + 9
        }

        if input::is_btn_down(VirtualKeyCode::R) {
            self.cur_value = 0;
            self.cur_color = Color::Red;
        }

        if input::is_btn_down(VirtualKeyCode::G) {
            self.cur_value = 0;
            self.cur_color = Color::Green;
        }

        if input::is_btn_down(VirtualKeyCode::B) {
            self.cur_value = 0;
            self.cur_color = Color::Blue;
        }

        if input::is_btn_down(VirtualKeyCode::V) {
            self.cur_value = 0;
            self.cur_color = Color::Alpha;
        }

        if input::is_btn_down(VirtualKeyCode::Return) {
            self.update_uniform_buffer(self.cur_value as f32);

            self.cur_value = 0;
        }

        if input::is_btn_down(VirtualKeyCode::C) {
            self.update_bg(self.cur_value as f32);

            self.cur_value = 0;
        }

        if input::is_btn_down(VirtualKeyCode::Up) {
            if self.cur_value < 255 {
                self.cur_value += 1;
            }

            self.update_uniform_buffer(self.cur_value as f32);
        }

        if input::is_btn_down(VirtualKeyCode::Down) {
            if self.cur_value > 0 {
                self.cur_value -= 1;
            }

            self.update_uniform_buffer(self.cur_value as f32);
        }

        if input::is_btn_down(VirtualKeyCode::Right) {
            if self.cur_value < 255 {
                self.cur_value += 1;
            }

            self.update_bg(self.cur_value as f32);
        }

        if input::is_btn_down(VirtualKeyCode::Left) {
            if self.cur_value > 0 {
                self.cur_value -= 1;
            }

            self.update_bg(self.cur_value as f32);
        }
    }

    fn update_uniform_buffer(&mut self, value: f32) {
        self.object.as_mut().unwrap()
            .update_color(&self.cur_color, value);
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
        self.device.borrow()
            .device.wait_idle().unwrap();
    }
}
