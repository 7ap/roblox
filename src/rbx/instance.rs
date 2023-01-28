use std::ffi::*;

#[repr(C)]
pub struct Instance {
    pub vtable: *const usize,     // 0x0000
    pub this: *const Self,        // 0x0004
    _pad0: [c_char; 0x08],        // 0x0008..0x000C
    pub descriptor: *const usize, // 0x0010
    _pad1: [c_char; 0x14],        // 0x0014..0x0020
    pub name: *const usize,       // 0x0024
    pub children: *const usize,   // 0x0028
    _pad2: [c_char; 0x04],        // 0x002C
    pub parent: *const Self,      // 0x0030
}
