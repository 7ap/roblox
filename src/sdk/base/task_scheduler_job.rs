use std::ffi::*;

use crate::sdk::extras::*;

#[repr(C)]
pub struct TaskSchedulerJob {
    _pad0: [c_char; 0x010],               // 0x000..0x010
    pub name: cxx::String,                // 0x010..0x028
    pub arbiter: boost::SharedPtr<usize>, // 0x028..0x030
}
