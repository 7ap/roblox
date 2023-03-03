mod task_scheduler;

pub use task_scheduler::TaskScheduler;

pub unsafe fn read_string(string: *const usize) -> String {
    use std::ffi::*;

    if *(string.byte_offset(0x10) as *const usize) < 16 {
        return CStr::from_ptr(string as *const c_char)
            .to_string_lossy()
            .to_string();
    }

    CStr::from_ptr(*(string as *const *const c_char))
        .to_string_lossy()
        .to_string()
}
