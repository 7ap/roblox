use std::ffi::*;

use crate::sdk::app::reflection::*;

#[repr(C)]
pub struct MemberDescriptorContainer {
    pub descriptors: [*mut usize; 3], // 0x000..0x00C
    pub descriptor_lookup: ([*mut usize; 3], isize, *const c_char, usize), // 0x00C..0x024
    pub derived_containers: [*mut usize; 3], // 0x024..0x030
    pub base: *const ClassDescriptor, // 0x030..0x034
    pub todo_descriptor_added_callback: usize, // 0x034..0x038
    _pad0: [c_char; 0x028],           // 0x038..0x05C
}
