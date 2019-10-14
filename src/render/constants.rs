use ash::vk_make_version;
use super::debug::ValidationInfo;
use super::structures::DeviceExtension;

//Engine Details
pub const WINDOW_TITLE:&str = "Vulkan setup";
pub const ENGINE_NAME:&str = "Zeus Engine";
pub const ENGINE_VERSION:u32 = vk_make_version!(1, 0, 0);

//Vulkan Version
pub const APPLICATION_VERSION:u32 = vk_make_version!(1, 0, 0);
pub const API_VERSION:u32 = vk_make_version!(1, 0, 92);

//Windows Size
pub const WINDOW_WIDTH:u32 = 1920;
pub const WINDOW_HEIGHT:u32 = 1080;

//Shader
pub const VERTEX_SHADER_LOCATION: &str = "src/render/shaders/spv/shader.vert.spv";
pub const FRAGMENT_SHADER_LOCATION: &str = "src/render/shaders/spv/shader.frag.spv";

pub const VALIDATION: ValidationInfo = ValidationInfo {
    is_enable: true,
    required_validation_layers: ["VK_LAYER_KHRONOS_validation"]
};

//SwapChain
pub const DEVICE_EXTENSIONS: DeviceExtension = DeviceExtension {
    names: ["VK_KHR_swapchain"],
};

pub const MAX_FRAMES_IN_FLIGHT: usize = 2;
