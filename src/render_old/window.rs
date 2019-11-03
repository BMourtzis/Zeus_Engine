use super::structures::SurfaceStuff;
use super::platforms;

use winit::EventsLoop;

pub fn init_window(
    events_loop: &EventsLoop,
    title: &str,
    width: u32,
    height: u32
    ) -> winit::Window 
    {
        winit::WindowBuilder::new()
            .with_title(title)
            .with_dimensions((width, height).into())
            .build(events_loop)
            .expect("Failed to create window.")
}

pub fn create_surface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    window: &winit::Window,
    screen_width: u32,
    screen_height: u32
) ->  SurfaceStuff
{
    let surface = unsafe {
        platforms::create_surface(entry, instance, window)
            .expect("Failed to create surface.")
    };
    let surface_loader = ash::extensions::khr::Surface::new(entry, instance);

    SurfaceStuff{
        surface_loader,
        surface,
        screen_width,
        screen_height
    }
}