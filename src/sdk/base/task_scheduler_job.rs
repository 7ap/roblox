use std::ffi::*;

#[repr(C)]
pub struct TaskSchedulerJob {
    _pad0: [c_char; 0x010], // 0x000..0x010
    pub name: [c_char; 24], // 0x010..0x028
}
