use std::ffi::*;

// TODO: Make this a macro.
pub unsafe fn read_string(string: *const usize) -> String {
    if *(string.byte_offset(0x10) as *const usize) < 16 {
        return CStr::from_ptr(string as *const c_char)
            .to_string_lossy()
            .to_string();
    }

    CStr::from_ptr(*(string as *const *const c_char))
        .to_string_lossy()
        .to_string()
}
