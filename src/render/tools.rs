use std::ffi::CStr;
use std::os::raw::c_char;

pub fn vk_to_string(rw_string_array: &[c_char]) -> String {
    let raw_string = unsafe {
        let pointer = rw_string_array.as_ptr();
        CStr::from_ptr(pointer)
    };

    raw_string.to_str()
        .expect("Failed to convert vulkan raw string")
        .to_owned()
}