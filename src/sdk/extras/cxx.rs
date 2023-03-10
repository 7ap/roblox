pub struct Pair<T1, T2> {
    pub first: T1,
    pub second: T2,
}

#[repr(C)]
pub struct Vector<T> {
    pub begin: *mut T,
    pub end: *mut usize,
    pub end_cap: *mut usize,
}
