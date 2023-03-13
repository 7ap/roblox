use std::ffi::*;

#[repr(C)]
pub struct FunctionBuffer {
    _todo0: [c_char; 40],
}

#[repr(C)]
pub struct FunctionBase {
    pub vtable: *mut usize,
    pub functor: FunctionBuffer,
}

#[repr(C)]
pub struct SharedCount {
    _todo0: usize,
}

#[repr(C)]
pub struct SharedPtr<T> {
    pub px: *mut T,
    pub pn: *mut SharedCount,
}
