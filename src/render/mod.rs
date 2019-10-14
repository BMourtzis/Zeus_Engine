//Private
mod constants;
mod debug;
mod device;
mod drawing;
mod pipeline;
mod platforms;
mod structures;
mod swapchain;
mod tools;
mod window;

use self::structures::{
    QueueFamilyIndices,
    SyncObjects,
    SurfaceStuff
};

//deps
use ash::version::DeviceV1_0;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;

use std::ptr;
use std::ffi::CString;
use std::os::raw::c_void;

use winit::{ControlFlow, Event, EventsLoop, VirtualKeyCode, WindowEvent};

pub struct ZeusEngine {
    window: winit::Window,

    _entry: ash::Entry,
    instance: ash::Instance,
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,

    physical_device: vk::PhysicalDevice,
    device: ash::Device,

    queue_family: QueueFamilyIndices,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,

    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<vk::Image>,
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    swapchain_imageviews: Vec<vk::ImageView>, 
    swapchain_framebuffers: Vec<vk::Framebuffer>,

    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    graphics_pipeline: vk::Pipeline,

    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,

    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    current_frame: usize,

    is_framebuffer_resized: bool
}

impl ZeusEngine {
    pub fn new(events_loop: &winit::EventsLoop) -> ZeusEngine {
        let window = window::init_window(
            &events_loop,
            constants::WINDOW_TITLE,
            constants::WINDOW_WIDTH,
            constants::WINDOW_HEIGHT
        );

        let entry = ash::Entry::new()
            .expect("Failed to start Entry!");
        let instance = ZeusEngine::create_instance(
            &entry, 
            constants::VALIDATION.is_enable, 
            &constants::VALIDATION.required_validation_layers.to_vec()
        );

        let surface_stuff = window::create_surface(
            &entry,
            &instance,
            &window,
            constants::WINDOW_WIDTH,
            constants::WINDOW_HEIGHT
        );
        let (debug_utils_loader, debug_messenger) = 
            debug::setup_debug_utils(constants::VALIDATION.is_enable, &entry, &instance);

        let physical_device = 
            device::pick_physical_device(&instance, &surface_stuff, &constants::DEVICE_EXTENSIONS);
        let (device, queue_family) = device::create_logical_device(
            &instance,
            physical_device,
            &constants::VALIDATION,
            &surface_stuff
        );

        let graphics_queue = unsafe {
            device.get_device_queue(queue_family.graphics_family as u32, 0)
        };
        let present_queue = unsafe {
            device.get_device_queue(queue_family.present_family as u32, 0)
        };
        
        let swapchain_stuff = swapchain::create_swapchain(
            &instance,
            &device,
            physical_device,
            &window,
            &surface_stuff,
            &queue_family,
        );
        let swapchain_imageviews = swapchain::create_image_views(
            &device,
            swapchain_stuff.swapchain_format,
            &swapchain_stuff.swapchain_images
        );

        // Here we set the layouts that are causing us issues
        let render_pass = pipeline::create_render_pass(&device, swapchain_stuff.swapchain_format);
        let (graphics_pipeline, pipeline_layout) = pipeline::create_graphics_pipeline(
            &device,
            render_pass,
            swapchain_stuff.swapchain_extent
        );

        let swapchain_framebuffers = drawing::create_framebuffers(
            &device, 
            render_pass, 
            &swapchain_imageviews, 
            swapchain_stuff.swapchain_extent
        );

        let command_pool = drawing::create_command_pool(
            &device, 
            &queue_family
        );
        let command_buffers = drawing::create_command_buffers(
            &device,
            command_pool,
            graphics_pipeline,
            &swapchain_framebuffers,
            render_pass,
            swapchain_stuff.swapchain_extent
        );
        let sync_objects = ZeusEngine::create_sync_objects(&device);
        
        ZeusEngine {
            window,

            _entry: entry,
            instance,
            surface: surface_stuff.surface,
            surface_loader: surface_stuff.surface_loader,
            debug_utils_loader,
            debug_messenger,

            physical_device,
            device,

            queue_family,
            graphics_queue,
            present_queue,

            swapchain_loader: swapchain_stuff.swapchain_loader,
            swapchain: swapchain_stuff.swapchain,
            swapchain_images: swapchain_stuff.swapchain_images,
            swapchain_format: swapchain_stuff.swapchain_format,
            swapchain_extent: swapchain_stuff.swapchain_extent,
            swapchain_imageviews,
            swapchain_framebuffers,

            render_pass,
            pipeline_layout,
            graphics_pipeline,

            command_pool,
            command_buffers,

            image_available_semaphores: sync_objects.image_available_semaphores,
            render_finished_semaphores: sync_objects.render_finished_semaphores,
            in_flight_fences: sync_objects.inflight_fences,
            current_frame: 0,

            is_framebuffer_resized: false
        }
    }

    fn create_instance(
        entry: &ash::Entry,
        is_enable_debug: bool,
        required_validation_layers: &[&str]
        ) -> ash::Instance {
        if is_enable_debug
            && !debug::check_validation_layer_support(entry, &required_validation_layers)
        {
            panic!("Validation layers requested, but not available");
        }

        let app_name = CString::new(constants::WINDOW_TITLE)
            .expect("Could not read C String");
        let engine_name = CString::new(constants::ENGINE_NAME)
            .expect("Could not read C String");
        let app_info = vk::ApplicationInfo {
            p_application_name: app_name.as_ptr(),
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            application_version: constants::APPLICATION_VERSION,
            p_engine_name: engine_name.as_ptr(),
            engine_version: constants::ENGINE_VERSION,
            api_version: constants::API_VERSION
        };

        let debug_utils_create_info = debug::populate_debug_messenger_create_info();

        let extension_names = platforms::required_extension_names();

        let required_validation_layer_raw_names: Vec<CString> = 
            constants::VALIDATION.required_validation_layers
            .iter()
            .map(|layer_name| CString::new(*layer_name)
                .expect("Could not read Layer Name")
            ).collect();
        let enable_label_names: Vec<* const i8> = 
            required_validation_layer_raw_names
                .iter()
                .map(|layer_name| layer_name.as_ptr())
                .collect();

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: if constants::VALIDATION.is_enable {
                &debug_utils_create_info as *const vk::DebugUtilsMessengerCreateInfoEXT as *const c_void
            }
            else {
                ptr::null()
            },
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names: if constants::VALIDATION.is_enable {
                enable_label_names.as_ptr()
            }
            else {
                ptr::null()
            },
            enabled_layer_count: if constants::VALIDATION.is_enable {
                enable_label_names.len()
            }
            else {
                0
            } as u32,
            pp_enabled_extension_names: extension_names.as_ptr(),
            enabled_extension_count: extension_names.len() as u32
        };

        let instance: ash::Instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to craete Instance!")
        };

        instance
    }

    fn create_sync_objects(device: &ash::Device) -> SyncObjects {
        let mut sync_objects = SyncObjects {
            image_available_semaphores: vec![],
            render_finished_semaphores: vec![],
            inflight_fences: vec![]
        };

        let semaphore_create_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::SemaphoreCreateFlags::empty()
        };

        let fences_create_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::FenceCreateFlags::SIGNALED
        };

        for _ in 0..constants::MAX_FRAMES_IN_FLIGHT {
            unsafe {
                let image_available_semaphores = device
                    .create_semaphore(&semaphore_create_info, None)
                    .expect("Failed to crate Semaphore Object");

                let render_finished_semaphores = device
                    .create_semaphore(&semaphore_create_info, None)
                    .expect("Failed to create Semaphore Object");

                let inflight_fences = device
                    .create_fence(&fences_create_info, None)
                    .expect("Failed to create Fence Object");

                sync_objects.image_available_semaphores
                    .push(image_available_semaphores);
                sync_objects.render_finished_semaphores
                    .push(render_finished_semaphores);
                sync_objects.inflight_fences.push(inflight_fences);
            }
        }

        sync_objects
    }

    //The error message occurs when the image_index changes from 0 to 1
    fn draw_frame(&mut self) {
        let wait_fences = [self.in_flight_fences[self.current_frame]];

        unsafe {
            self.device.wait_for_fences(&wait_fences, true, std::u64::MAX)
                .expect("Failed to wait for Fence!")
        }

        let (image_index, _is_sub_optimal) = unsafe {
            let result = self.swapchain_loader.acquire_next_image(
                self.swapchain, 
                std::u64::MAX, 
                self.image_available_semaphores[self.current_frame],
                vk::Fence::null()
            );

            match result {
                Ok(image_index) => image_index,
                Err(vk_result) => match vk_result {
                    vk::Result::ERROR_OUT_OF_DATE_KHR => {
                        self.recreate_swapchain();
                        return;
                    },
                    _ => panic!("Failed to acquire Swap Chain Image!")
                }
            }
        };

        let wait_semaphores = [self.image_available_semaphores[self.current_frame]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [self.render_finished_semaphores[self.current_frame]];

        let submit_infos = [vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &self.command_buffers[image_index as usize],
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr()
        }];

        unsafe {
            self.device.reset_fences(&wait_fences)
                .expect("Failed to reset Fences");

            self.device
                .queue_submit(
                    self.graphics_queue, 
                    &submit_infos, 
                    self.in_flight_fences[self.current_frame]
                )
                .expect("Failed to execute queue submit");
        }

        let swapchains = [self.swapchain];

        let present_info = vk::PresentInfoKHR {
            s_type: vk::StructureType::PRESENT_INFO_KHR,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: signal_semaphores.as_ptr(),
            swapchain_count: 1,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: &image_index,
            p_results: ptr::null_mut()
        };

        let result = unsafe {
            self.swapchain_loader
                .queue_present(self.present_queue, &present_info)
        };

        let is_resized = match result {
            Ok(_) => self.is_framebuffer_resized,
            Err(vk_result) => match vk_result {
                vk::Result::ERROR_OUT_OF_DATE_KHR | vk::Result::SUBOPTIMAL_KHR => true,
                _ => panic!("Failed to execute queue present.")
            }
        };

        if is_resized {
            self.is_framebuffer_resized = false;
            self.recreate_swapchain();
        }

        self.current_frame = (self.current_frame + 1) % constants::MAX_FRAMES_IN_FLIGHT;
    }

    fn recreate_swapchain(&mut self) {
        let surface_stuff = SurfaceStuff {
            surface_loader: self.surface_loader.clone(),
            surface: self.surface,
            screen_width: constants::WINDOW_WIDTH,
            screen_height: constants::WINDOW_HEIGHT
        };

        unsafe {
            self.device.device_wait_idle()
                .expect("Failed to wait device idle")
        }

        self.cleanup_swapchain();

        let swapchain_stuff = swapchain::create_swapchain(
            &self.instance,
            &self.device,
            self.physical_device,
            &self.window,
            &surface_stuff,
            &self.queue_family
        );
        self.swapchain_loader = swapchain_stuff.swapchain_loader;
        self.swapchain = swapchain_stuff.swapchain;
        self.swapchain_images = swapchain_stuff.swapchain_images;
        self.swapchain_format = swapchain_stuff.swapchain_format;
        self.swapchain_extent = swapchain_stuff.swapchain_extent;

        self.swapchain_imageviews = swapchain::create_image_views(
            &self.device,
            self.swapchain_format,
            &self.swapchain_images
        );

        self.render_pass = pipeline::create_render_pass(&self.device, self.swapchain_format);
        let (graphics_pipeline, pipeline_layout) = pipeline::create_graphics_pipeline(
            &self.device,
            self.render_pass,
            swapchain_stuff.swapchain_extent
        );
        self.graphics_pipeline = graphics_pipeline;
        self.pipeline_layout = pipeline_layout;

        self.swapchain_framebuffers = drawing::create_framebuffers(
            &self.device,
            self.render_pass,
            &self.swapchain_imageviews,
            self.swapchain_extent
        );
        self.command_buffers = drawing::create_command_buffers(
            &self.device,
            self.command_pool,
            self.graphics_pipeline,
            &self.swapchain_framebuffers,
            self.render_pass,
            self.swapchain_extent
        );
    }

    fn cleanup_swapchain(&self) {
        unsafe {
            self.device.free_command_buffers(self.command_pool, &self.command_buffers);

            for &framebuffer in self.swapchain_framebuffers.iter() {
                self.device.destroy_framebuffer(framebuffer, None);
            }

            self.device.destroy_pipeline(self.graphics_pipeline, None);
            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_render_pass(self.render_pass, None);

            for &image_view in self.swapchain_imageviews.iter() {
                self.device.destroy_image_view(image_view, None);
            }
            
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
        }
    }
}


impl Drop for ZeusEngine {
    fn drop(&mut self) {
        unsafe {
            for i in 0..constants::MAX_FRAMES_IN_FLIGHT {
                self.device
                    .destroy_semaphore(self.image_available_semaphores[i], None);
                self.device
                    .destroy_semaphore(self.render_finished_semaphores[i], None);
                self.device.destroy_fence(self.in_flight_fences[i], None);
            }

            self.cleanup_swapchain();

            self.device.destroy_command_pool(self.command_pool, None);

            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);

            if constants::VALIDATION.is_enable {
                self.debug_utils_loader.destroy_debug_utils_messenger(self.debug_messenger, None);
            }
            
            self.instance.destroy_instance(None);
        }
    }
}

pub struct ProgramProc {
    pub events_loop: EventsLoop
}

impl ProgramProc {
    pub fn new() -> ProgramProc {
        let events_loop = EventsLoop::new();
        ProgramProc {
            events_loop
        }
    }

    pub fn main_loop(&mut self, renderer: &mut ZeusEngine) {
        self.events_loop.run_forever(|event| {
            if let Event::WindowEvent{ event, ..} = event { 
                match event {
                    WindowEvent::KeyboardInput {input, ..} => {
                        if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                            return ControlFlow::Break;
                        }
                    },
                    WindowEvent::CloseRequested => return ControlFlow::Break,
                    _ => ()
                } 
            }

            renderer.draw_frame();
            ControlFlow::Continue
        });

        unsafe {
            renderer.device.device_wait_idle()
                .expect("Failed to wait device idle!")
        };
    }
}

impl Default for ProgramProc {
    fn default() -> Self {
        ProgramProc::new()
    }
}