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

pub fn find_depth_format(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> vk::Format {
    find_supported_format(
        instance,
        physical_device,
        &[
            vk::Format::D32_SFLOAT,
            vk::Format::D32_SFLOAT_S8_UINT,
            vk::Format::D24_UNORM_S8_UINT,
        ],
        vk::ImageTiling::OPTIMAL,
        vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
    )

}

pub fn find_supported_format(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    candidate_formats: &[vk::Format],
    tiling: vk::ImageTiling,
    features: vk::FormatFeatureFlags,
) -> vk::Format {
    for &format in candidate_formats.iter() {
        let format_properties =
            unsafe { instance.get_physical_device_format_properties(physical_device, format) };
        if tiling == vk::ImageTiling::LINEAR
            && format_properties.linear_tiling_features.contains(features)
        {
            return format.clone();
        } else if tiling == vk::ImageTiling::OPTIMAL
            && format_properties.optimal_tiling_features.contains(features)
        {
            return format.clone();
        }
    }

    panic!("Failed to find supported format!")

}