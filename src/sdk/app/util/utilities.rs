use crate::sdk::extras::*;

#[repr(C)]
pub struct CopyOnWritePtr<T> {
    pub object: boost::SharedPtr<T>,
}
