use super::structures:: {
    SurfaceStuff,
    QueueFamilyIndices,
    SwapChainSupportDetails,
    SwapChainStuff
};

use ash::vk;
use ash::version::DeviceV1_0;

use std::ptr;

pub fn create_swapchain(
    instance: &ash::Instance,
    device: &ash::Device,
    physical_device: vk::PhysicalDevice,
    window: &winit::Window,
    surface_stuff: &SurfaceStuff,
    queue_family: &QueueFamilyIndices
) -> SwapChainStuff {
    let swapchain_support = query_swapchain_support(physical_device, surface_stuff);

    let surface_format = choose_swapchain_format(&swapchain_support.formats);
    let present_mode = choose_swapchain_present_mode(&swapchain_support.present_modes);
    let extent = choose_swapchain_extent(&swapchain_support.capabilities, window);

    // use std::cmp::min;

    // let image_count = min(
    //     swapchain_support.capabilities.min_image_count + 1,
    //     swapchain_support.capabilities.max_image_count
    // );

    let image_count = 2;

    let (image_sharing_mode, queue_family_index_count, queue_family_indices) =
        if queue_family.graphics_family != queue_family.present_family 
        {
            (
                vk::SharingMode::CONCURRENT,
                2,
                vec![
                    queue_family.graphics_family as u32,
                    queue_family.present_family as u32
                ]
            ) 
        }
        else {
            (vk::SharingMode::EXCLUSIVE, 0, vec![])
        };

        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface: surface_stuff.surface,
            min_image_count: image_count,
            image_color_space: surface_format.color_space,
            image_format: surface_format.format,
            image_extent: extent,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode,
            p_queue_family_indices: queue_family_indices.as_ptr(),
            queue_family_index_count,
            pre_transform: swapchain_support.capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode,
            clipped: vk::TRUE,
            old_swapchain: vk::SwapchainKHR::null(),
            image_array_layers: 1
        };

        let swapchain_loader = ash::extensions::khr::Swapchain::new(instance, device);
        let swapchain = unsafe {
            swapchain_loader.create_swapchain(&swapchain_create_info, None)
                .expect("Failed to create Swapchain")
        };

        let swapchain_images = unsafe {
            swapchain_loader.get_swapchain_images(swapchain)
                .expect("Faile to create Swapchain Images")
        };

        SwapChainStuff {
            swapchain_loader,
            swapchain,
            swapchain_format: surface_format.format,
            swapchain_extent: extent,
            swapchain_images
        }
}

pub fn query_swapchain_support(
    physical_device: vk::PhysicalDevice,
    surface_stuff: &SurfaceStuff
) -> SwapChainSupportDetails 
{
    unsafe {
        let capabilities = surface_stuff.surface_loader
            .get_physical_device_surface_capabilities(physical_device, surface_stuff.surface)
            .expect("Failed to query for surface capabilities.");
        let formats = surface_stuff.surface_loader
            .get_physical_device_surface_formats(physical_device, surface_stuff.surface)
            .expect("Failed to query for surface formats.");
        let present_modes = surface_stuff.surface_loader
            .get_physical_device_surface_present_modes(physical_device, surface_stuff.surface)
            .expect("Failed to query for surface present mode.");

        SwapChainSupportDetails {
            capabilities,
            formats,
            present_modes
        }
    }
}

fn choose_swapchain_format(available_formats: &[vk::SurfaceFormatKHR]) 
-> vk::SurfaceFormatKHR 
{
    for available_format in available_formats {
        if available_format.format == vk::Format::B8G8R8A8_UNORM
            && available_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        {
            return *available_format;
        }
    }

    *available_formats.first().unwrap()
}

fn choose_swapchain_present_mode (available_present_modes: &[vk::PresentModeKHR])
    -> vk::PresentModeKHR 
{
    let mut best_mode = vk::PresentModeKHR::FIFO;

    for &available_present_mode in available_present_modes.iter() {
        if available_present_mode == vk::PresentModeKHR::MAILBOX {
            return available_present_mode;
        }
        else if available_present_mode == vk::PresentModeKHR::IMMEDIATE {
            best_mode = available_present_mode
        }
    }

    best_mode
}

fn choose_swapchain_extent(
    capabilities: &vk::SurfaceCapabilitiesKHR,
    window: &winit::Window
) -> vk::Extent2D 
{
    if capabilities.current_extent.width != u32::max_value() 
    {
        capabilities.current_extent
    }
    else {
        use num::clamp;

        let window_size = window.get_inner_size()
            .expect("Failed to get the size of the Window");
        println!("\t\tInner Window Size: ({},{})", window_size.width, window_size.height);

        vk::Extent2D {
            width: clamp(
                window_size.width as u32,
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width
            ),
            height: clamp(
                window_size.height as u32,
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height
            )
        }
    }
}

pub fn create_image_views(
    device: &ash::Device,
    surface_format: vk::Format,
    images: &[vk::Image]
    ) -> Vec<vk::ImageView> 
{
    let swapchain_imageviews: Vec<vk::ImageView> = images
        .iter()
        .map(|&image| {
            create_image_view(
                device,
                image,
                surface_format,
                vk::ImageAspectFlags::COLOR, 
                1
            )
        }).collect();
    
    swapchain_imageviews
}

fn create_image_view(
    device: &ash::Device,
    image: vk::Image,
    format: vk::Format,
    aspect_flags: vk::ImageAspectFlags,
    mip_levels: u32
) -> vk::ImageView
{
    let imageview_create_info = vk::ImageViewCreateInfo {
        s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::ImageViewCreateFlags::empty(),
        view_type: vk::ImageViewType::TYPE_2D,
        format,
        components: vk::ComponentMapping {
            r: vk::ComponentSwizzle::IDENTITY,
            g: vk::ComponentSwizzle::IDENTITY,
            b: vk::ComponentSwizzle::IDENTITY,
            a: vk::ComponentSwizzle::IDENTITY
        },
        subresource_range: vk::ImageSubresourceRange {
            aspect_mask: aspect_flags,
            base_mip_level: 0,
            level_count: mip_levels,
            base_array_layer: 0,
            layer_count: 1,
        },
        image
    };

    unsafe {
        device.create_image_view(&imageview_create_info, None)
            .expect("Failed to create Image View!")
    }
}