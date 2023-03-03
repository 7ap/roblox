use std::ffi::*;

#[repr(C)]
pub struct TaskSchedulerJob {
    pub vtable: *const usize, // 0x000..0x004
    pub this: *const Self,    // 0x004..0x008
    _pad0: [c_char; 0x008],   // 0x008..0x010
    pub name: usize,          // 0x010..0x014
    _pad1: [c_char; 0x024],   // 0x014..0x038
    pub step_start_time: f64, // 0x038..0x040
}
