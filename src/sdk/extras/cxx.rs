use std::ffi::*;
use std::mem;

use anyhow::Result;

#[repr(C)]
pub struct Pair<T1, T2> {
    pub first: T1,
    pub second: T2,
}

#[repr(C)]
pub struct String {
    _todo: [c_char; 24],
}

impl String {
    pub fn c_str(&self) -> Result<*const c_char> {
        // Atrocious hack. Find better solution at some point.
        let size: [c_char; 4] = self._todo[16..20].try_into()?;
        let size: isize = unsafe { mem::transmute_copy(&size) };

        let string = if size >= 16 {
            let string = self._todo.as_ptr() as *const *const c_char;
            unsafe { CStr::from_ptr(*string) }
        } else {
            let string = self._todo.as_ptr() as *const c_char;
            unsafe { CStr::from_ptr(string) }
        };

        Ok(string.as_ptr())
    }

    pub fn r_str(&self) -> Result<std::string::String> {
        let string = unsafe { CStr::from_ptr(self.c_str()?) };

        Ok(std::string::String::from_utf8_lossy(string.to_bytes()).to_string())
    }
}
#[repr(C)]
pub struct Vector<T> {
    pub begin: *mut T,
    pub end: *mut T,
    pub end_cap: *mut T,
}

impl<T> Vector<T> {
    pub fn convert(&self) -> Vec<*mut T> {
        let mut buffer = Vec::new();
        let mut begin = self.begin;

        while begin != self.end {
            unsafe {
                buffer.push(begin);
                begin = begin.add(1);
            }
        }

        buffer
    }
}
