use std::ffi::*;

#[repr(C)]
pub struct Descriptor {
    _pad0: [c_char; 0x018], // 0x000..0x018
}
