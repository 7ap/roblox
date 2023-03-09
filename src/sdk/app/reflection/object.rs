use std::ffi::*;

#[repr(C)]
pub struct ClassDescriptor;

#[repr(C)]
pub struct DescribedBase {
    _super0: [c_char; 0x004],         // 0x000..0x004
    _super1: [c_char; 0x008],         // 0x004..0x00C
    descriptor: *mut ClassDescriptor, // 0x00C..0x010
    xml_id: [usize; 2],               // 0x010..0x018
}
