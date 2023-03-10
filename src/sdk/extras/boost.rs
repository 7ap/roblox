#[repr(C)]
pub struct SharedCount {
    _todo0: usize,
}

#[repr(C)]
pub struct SharedPtr<T> {
    pub px: *mut T,
    pub pn: *mut SharedCount,
}
