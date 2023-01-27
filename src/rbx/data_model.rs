use std::ptr::{self, NonNull};

use super::constants::data_model;
use super::task_scheduler::TaskScheduler;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DataModel;

impl DataModel {
    pub unsafe fn get() -> NonNull<Self> {
        let data_model = *(TaskScheduler::get()
            .as_ref()
            .get_jobs_by_name("Render")
            .unwrap()
            .as_ptr()
            .byte_offset(data_model::OFFSET) as *const *const usize);

        log::debug!("DataModel @ {:#08X?}", data_model.addr());

        NonNull::<DataModel>::new(data_model as *mut _).unwrap()
    }
}
