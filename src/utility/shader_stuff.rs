use ash::version::DeviceV1_0;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;

use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

use crate::utility::constants::*;
use crate::utility::debug;
use crate::utility::platforms;
use crate::utility::structures::*;

pub fn create_shader_module(
    device: &ash::Device,
    spirv_path: &str) -> vk::ShaderModule {
    // read the spirv file
    let spirv_u8 = std::fs::read(spirv_path)
        .unwrap_or_else(|_| panic!("Failed to read spirv file at {}!"));
    
    // create shader module
    let spirv_u32 = {
        /* This is needed because std::fs::read returns a Vec<u8>, but Vulkan
        wants a &[u32] slice.

        We break the slice into a prefix, middle and suffix, and make sure that
        the prefix and suffix are empty. This ensures that we don't miss
        alignment and get invalid data. */
        let(prefix_u8, middle_u32, suffix_u8) = unsafe { spirv_u8.align_to::<u32>() };
        assert_eq!(prefix_u8.len(), 0);
        assert_eq!(suffix_u8.len(), 0);
        middle_u32
    };

    let create_info = vk::ShaderModuleCreateInfo::builder().code(spirv_u32);

    let vk_shader_module = unsafe {
        device
            .create_shader_module(&create_info, None)
            .expect("Failed to create shader module!")
    };

    vk_shader_module
}