use std::ops::Deref;

use crate::sdk::app::v8tree::Instance;

#[repr(C)]
pub struct DataModel {
    _super: Instance,
}

impl Deref for DataModel {
    type Target = Instance;

    fn deref(&self) -> &Self::Target {
        &self._super
    }
}

impl DataModel {
    pub unsafe fn get() -> *mut Self {
        todo!()
    }
}
