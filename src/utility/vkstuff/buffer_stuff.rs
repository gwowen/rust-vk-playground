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

pub fn create_buffer(
    device: &ash::Device,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    required_memory_properties: vk::MemoryPropertyFlags,
    device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
) -> (vk::Buffer, vk::DeviceMemory) {


    let buffer_create_info = vk::BufferCreateInfo {
        s_type: vk::StructureType::BUFFER_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::BufferCreateFlags::empty(),
        size,
        usage,
        sharing_mode: vk::SharingMode::EXCLUSIVE,
        queue_family_index_count: 0,
        p_queue_family_indices: ptr::null(),
    };

    let buffer = unsafe {
        device
            .create_buffer(&buffer_create_info, None)
            .expect("Failed to create Vertex Buffer")
    };

    let mem_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
    let memory_type = find_memory_type(
        mem_requirements.memory_type_bits,
        required_memory_properties,
        device_memory_properties,
    );

    let allocate_info = vk::MemoryAllocateInfo {
        s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
        p_next: ptr::null(),
        allocation_size: mem_requirements.size,
        memory_type_index: memory_type,
    };

    let buffer_memory = unsafe {
        device
            .allocate_memory(&allocate_info, None)
            .expect("Failed to allocate vertex buffer memory!")
    };

    unsafe {
        device
            .bind_buffer_memory(buffer, buffer_memory, 0)
            .expect("Failed to bind Buffer");
    }

    (buffer, buffer_memory)
}

pub fn copy_buffer(
    device: &ash::Device,
    submit_queue: vk::Queue,
    command_pool: vk::CommandPool,
    src_buffer: vk::Buffer,
    dst_buffer: vk::Buffer,
    size: vk::DeviceSize,
) {
    let command_buffer = begin_single_time_command(device, command_pool);

    let copy_regions = [vk::BufferCopy {
        src_offset: 0,
        dst_offset: 0,
        size,
    }];

    unsafe {
        device.cmd_copy_buffer(command_buffer, src_buffer, dst_buffer, &copy_regions);
    }

    end_single_time_command(device, command_pool, submit_queue, command_buffer);

}
pub fn begin_single_time_command(
    device: &ash::Device,
    command_pool: vk::CommandPool,
) -> vk::CommandBuffer {
    let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
        s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        p_next: ptr::null(),
        command_buffer_count: 1,
        command_pool,
        level: vk::CommandBufferLevel::PRIMARY,
    };

    let command_buffer = unsafe {
        device
            .allocate_command_buffers(&command_buffer_allocate_info)
            .expect("Failed to allocate Command Buffers!")
    }[0];

    let command_buffer_begin_info = vk::CommandBufferBeginInfo {
        s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
        p_next: ptr::null(),
        p_inheritance_info: ptr::null(),
        flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
    };

    unsafe {
        device
            .begin_command_buffer(command_buffer, &command_buffer_begin_info)
            .expect("Failed to begin recording Command Buffer at beginning!");
    }

    command_buffer
}

pub fn end_single_time_command(
    device: &ash::Device,
    command_pool: vk::CommandPool,
    submit_queue: vk::Queue,
    command_buffer: vk::CommandBuffer,
) {
    unsafe {
        device
            .end_command_buffer(command_buffer)
            .expect("Failed to record Command Buffer at Ending!");
    }

    let buffers_to_submit = [command_buffer];

    let sumbit_infos = [vk::SubmitInfo {
        s_type: vk::StructureType::SUBMIT_INFO,
        p_next: ptr::null(),
        wait_semaphore_count: 0,
        p_wait_semaphores: ptr::null(),
        p_wait_dst_stage_mask: ptr::null(),
        command_buffer_count: 1,
        p_command_buffers: buffers_to_submit.as_ptr(),
        signal_semaphore_count: 0,
        p_signal_semaphores: ptr::null(),
    }];

    unsafe {
        device
            .queue_submit(submit_queue, &sumbit_infos, vk::Fence::null())
            .expect("Failed to Queue Submit!");
        device
            .queue_wait_idle(submit_queue)
            .expect("Failed to wait Queue idle!");
        device.free_command_buffers(command_pool, &buffers_to_submit);
    }

}

pub fn find_memory_type(
    type_filter: u32,
    required_properties: vk::MemoryPropertyFlags,
    mem_properties: &vk::PhysicalDeviceMemoryProperties
) -> u32 {
    for (i, memory_type) in mem_properties.memory_types.iter().enumerate() {
        if (type_filter & (1 << i)) > 0 && memory_type.property_flags.contains(required_properties)
        {
            return i as u32;
        }
    }

    panic!("Failed to find suitable memory type!")

}

pub fn has_stencil_component(format: vk::Format) -> bool {
    format == vk::Format::D32_SFLOAT_S8_UINT || format == vk::Format::D24_UNORM_S8_UINT
}

pub fn copy_buffer_to_image(
    device: &ash::Device,
    command_pool: vk::CommandPool,
    submit_queue: vk::Queue,
    buffer: vk::Buffer,
    image: vk::Image,
    width: u32,
    height: u32,
) {
    let command_buffer = begin_single_time_command(device, command_pool);

    let buffer_image_regions = [vk::BufferImageCopy {
        image_subresource: vk::ImageSubresourceLayers {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            mip_level: 0,
            base_array_layer: 0,
            layer_count: 1,
        },
        image_extent: vk::Extent3D {
            width,
            height,
            depth: 1,
        },
        buffer_offset: 0,
        buffer_image_height: 0,
        buffer_row_length: 0,
        image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
    }];

    unsafe {
        device.cmd_copy_buffer_to_image(
            command_buffer,
            buffer,
            image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &buffer_image_regions,
        );
    }

    end_single_time_command(device, command_pool, submit_queue, command_buffer);
}

pub fn create_vertex_buffer<T>(
    device: &ash::Device,
    device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
    command_pool: vk::CommandPool,
    submit_queue: vk::Queue,
    data: &[T],
) -> (vk::Buffer, vk::DeviceMemory) {
    let buffer_size = ::std::mem::size_of_val(data) as vk::DeviceSize;

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        device,
        buffer_size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        &device_memory_properties,
    );

    unsafe {
        let data_ptr = device
            .map_memory(
                staging_buffer_memory,
                0,
                buffer_size,
                vk::MemoryMapFlags::empty(),
            )
            .expect("Failed to Map Memory") as *mut T;

        data_ptr.copy_from_nonoverlapping(data.as_ptr(), data.len());

        device.unmap_memory(staging_buffer_memory);
    }

    let (vertex_buffer, vertex_buffer_memory) = create_buffer(
        device,
        buffer_size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
        &device_memory_properties,
    );

    copy_buffer(
        device,
        submit_queue,
        command_pool,
        staging_buffer,
        vertex_buffer,
        buffer_size,
    );

    unsafe {
        device.destroy_buffer(staging_buffer, None);
        device.free_memory(staging_buffer_memory, None);
    }

    (vertex_buffer, vertex_buffer_memory)
}

pub fn create_index_buffer(
    device: &ash::Device,
    device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
    command_pool: vk::CommandPool,
    submit_queue: vk::Queue,
    data: &[u32],
) -> (vk::Buffer, vk::DeviceMemory) {
    let buffer_size = ::std::mem::size_of_val(data) as vk::DeviceSize;

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        device,
        buffer_size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        &device_memory_properties,
    );

    unsafe {
        let data_ptr = device
            .map_memory(
                staging_buffer_memory,
                0,
                buffer_size,
                vk::MemoryMapFlags::empty(),
            )
            .expect("Failed to Map Memory") as *mut u32;

        data_ptr.copy_from_nonoverlapping(data.as_ptr(), data.len());

        device.unmap_memory(staging_buffer_memory);
    }

    let (index_buffer, index_buffer_memory) = create_buffer(
        device,
        buffer_size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
        &device_memory_properties,
    );

    copy_buffer(
        device,
        submit_queue,
        command_pool,
        staging_buffer,
        index_buffer,
        buffer_size,
    );

    unsafe {
        device.destroy_buffer(staging_buffer, None);
        device.free_memory(staging_buffer_memory, None);
    }

    (index_buffer, index_buffer_memory)
}

