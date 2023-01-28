use std::ops::Deref;
use std::ptr::NonNull;

use super::constants::data_model;
use super::instance::Instance;
use super::task_scheduler::TaskScheduler;

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
    pub unsafe fn get() -> NonNull<Self> {
        let data_model = *(TaskScheduler::get()
            .as_ref()
            .get_jobs_by_name("Render")
            .unwrap()
            .as_ptr()
            .byte_offset(data_model::OFFSET) as *const *const usize);

        log::trace!("DataModel @ {:#08X?}", data_model.addr());

        NonNull::<DataModel>::new(data_model as *mut _).unwrap()
    }
}
