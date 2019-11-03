use super::tools::vk_to_string;
use super::debug::ValidationInfo;
use super::swapchain::query_swapchain_support;
use super::structures::{
    SurfaceStuff, 
    QueueFamilyIndices,
    SwapChainSupportDetails,
    DeviceExtension
};

use ash::vk;
use ash::version::InstanceV1_0;

use std::ptr;
use std::collections::HashSet;
use std::ffi::CString;
use std::os::raw::c_char;
use std::convert::TryInto;

pub fn pick_physical_device(
    instance: &ash::Instance,
    surface_stuff: &SurfaceStuff,
    required_device_extensions: &DeviceExtension
) -> vk::PhysicalDevice 
{
    let physical_devices = unsafe {
        instance.enumerate_physical_devices()
            .expect("Failed to enumerate Physical Devices!")
    };

    let result = physical_devices.iter()
        .find(|physical_device| {
            let swapchain_support = query_swapchain_support(**physical_device, surface_stuff);
            is_physical_device_suitable(
                instance,
                **physical_device,
                surface_stuff,
                &swapchain_support,
                &required_device_extensions
            )
        });

    match result {
        None => panic!("failed to Find a suitalbe GPU!"),
        Some(p_physical_device) => *p_physical_device
    }
}

fn is_physical_device_suitable(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    surface_stuff: &SurfaceStuff,
    swapchain_support: &SwapChainSupportDetails,
    required_device_extensions: &DeviceExtension
) -> bool 
{
    // let device_features = unsafe { 
    //     instance.get_physical_device_features(physical_device) 
    // };

    let indices = find_queue_family(instance, physical_device, surface_stuff);

    let is_queue_family_supported = indices.is_complete();
    let is_device_extension_supported = check_device_extension_support(instance, physical_device, required_device_extensions);
    let is_swapchain_supported = !swapchain_support.formats.is_empty() && !swapchain_support.present_modes.is_empty();

    is_queue_family_supported && is_device_extension_supported && is_swapchain_supported
}

fn check_device_extension_support(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    required_device_extensions: &DeviceExtension,
) -> bool 
{
    let available_extensions = unsafe {
        instance.enumerate_device_extension_properties(physical_device)
            .expect("Faile to get device extension properties")
    };

    let mut available_extensions_names = vec![];

    println!("\tAvailable Device Extensions: ");
    for extension in available_extensions.iter() {
        let extension_name = vk_to_string(&extension.extension_name);
        println!("\t\tName: {}, Version: {}", extension_name, extension.spec_version);

        available_extensions_names.push(extension_name);
    }

    let mut required_extensions = HashSet::new();
    for extension in required_device_extensions.names.iter() {
        required_extensions.insert(extension.to_string());
    }

    for extension_name in available_extensions_names.iter() {
        required_extensions.remove(extension_name);
    }

    required_extensions.is_empty()
}

fn find_queue_family(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    surface_stuff: &SurfaceStuff
) -> QueueFamilyIndices 
{
    let queue_families = unsafe 
    {
        instance.get_physical_device_queue_family_properties(physical_device) 
    };

    let mut queue_family_indices = QueueFamilyIndices::new();

    // let mut index = 0;
    for (index, queue_family) in queue_families.iter().enumerate() {
        if queue_family.queue_count > 0 
            && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
        {
            queue_family_indices.graphics_family = index.try_into().expect("Could not convert into i32");
        }

        let is_present_support = unsafe {
            surface_stuff.surface_loader
                .get_physical_device_surface_support(
                    physical_device,
                    index as u32,
                    surface_stuff.surface
                )
        };

        if queue_family.queue_count > 0 && is_present_support {
            queue_family_indices.present_family = index.try_into().expect("Could not convert into i32");
        }

        if queue_family_indices.is_complete() {
            break;
        }

        // index += 1;
    }

    queue_family_indices
}

pub fn create_logical_device(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice, 
    validation: &ValidationInfo,
    surface_stuff: &SurfaceStuff
    ) -> (ash::Device, QueueFamilyIndices) 
{
    let indices = find_queue_family(instance, physical_device, surface_stuff);

    let mut unique_queue_families = HashSet::new();
    unique_queue_families.insert(indices.graphics_family as u32);
    unique_queue_families.insert(indices.present_family as u32);

    let queue_priorities = [1.0_f32];
    let mut queue_create_infos = vec![];
    for &queue_family in  unique_queue_families.iter() {
        let queue_create_info = vk::DeviceQueueCreateInfo 
        {
            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DeviceQueueCreateFlags::empty(),
            queue_family_index: queue_family,
            p_queue_priorities: queue_priorities.as_ptr(),
            queue_count: queue_priorities.len() as u32
        };
        queue_create_infos.push(queue_create_info);
    }

    let physical_device_features = vk::PhysicalDeviceFeatures {
        ..Default::default()
    };

    let required_validation_layer_raw_names: Vec<CString> = validation
        .required_validation_layers
        .iter()
        .map(|layer_name| CString::new(*layer_name).unwrap())
        .collect();
    let enable_layer_names: Vec<*const c_char> = required_validation_layer_raw_names
        .iter()
        .map(|layer_name| layer_name.as_ptr())
        .collect();
    
    let enable_extension_names = [
        ash::extensions::khr::Swapchain::name().as_ptr()
    ];

    let device_create_info = vk::DeviceCreateInfo 
    {
        s_type: vk::StructureType::DEVICE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::DeviceCreateFlags::empty(),
        queue_create_info_count: queue_create_infos.len() as u32,
        p_queue_create_infos: queue_create_infos.as_ptr(),
        enabled_layer_count: if validation.is_enable
        {
            enable_layer_names.len()
        } 
        else {
            0
        } as u32,
        pp_enabled_layer_names: if validation.is_enable
        {
            enable_layer_names.as_ptr()
        }
        else {
            ptr::null()
        },
        enabled_extension_count: enable_extension_names.len() as u32,
        pp_enabled_extension_names: enable_extension_names.as_ptr(),
        p_enabled_features: &physical_device_features
    };

    let device: ash::Device = unsafe {
        instance
            .create_device(physical_device, &device_create_info, None)
            .expect("Failed to create logica Device!")
    };

    (device, indices)
}