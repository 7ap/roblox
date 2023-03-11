use std::ffi::*;

#[repr(C)]
pub struct Attributes {
    _pad0: [c_char; 0x008],
}

#[repr(C)]
pub struct Descriptor {
    pub vtable: *const usize,        // 0x000..0x004
    pub name: *const c_char,         // 0x004..0x008
    pub attributes: Attributes,      // 0x008..0x010
    _todo_counter_index: [usize; 2], // 0x010..0x018
}
