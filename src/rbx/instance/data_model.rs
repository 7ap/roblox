use std::ops::Deref;
use std::ptr;

use crate::rbx::TaskScheduler;

use super::Instance;

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
    pub unsafe fn get() -> &'static Self {
        // TODO: Simplify (if possible) later.
        let data_model = (*(ptr::from_ref(TaskScheduler::get().get_jobs_by_name("Render").unwrap())
            .byte_offset(0x28) as *const *const usize) as *const usize)
            .byte_offset(0x04);

        log::trace!("DataModel @ {:#08X?}", data_model.addr());

        &*(data_model as *mut Self)
    }
}
