use std::ffi::*;

use crate::sdk::extras::*;

#[repr(C)]
pub struct Attributes {
    _todo: [c_char; 8],
}

#[repr(C)]
pub struct Descriptor {
    pub vtable: *const usize,     // 0x000..0x004
    pub name: *const cxx::String, // 0x004..0x008
    pub attributes: Attributes,   // 0x008..0x010
    pub counter_index: isize,     // 0x010..0x014
}
