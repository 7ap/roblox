use std::ffi::*;
use std::ptr;

#[repr(C)]
pub struct TaskSchedulerJob {
    vtable: *const usize,  // 0x0000
    this: *const Self,     // 0x0004
    _pad0: [c_char; 0x08], // 0x0008..0x000C
    name: usize,           // 0x0010
}

impl TaskSchedulerJob {
    pub unsafe fn get_name(&self) -> String {
        let name = ptr::addr_of!(self.name);

        if *(name.byte_offset(0x10) as *const usize) < 16 {
            return CStr::from_ptr(name as *const c_char)
                .to_string_lossy()
                .to_string();
        }

        CStr::from_ptr(*(name as *const *const c_char))
            .to_string_lossy()
            .to_string()
    }
}
