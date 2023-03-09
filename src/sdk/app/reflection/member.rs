use std::ffi::*;

#[repr(C)]
pub struct MemberDescriptorContainer {
    _pad0: [c_char; 0x060], // 0x000..0x060
}
