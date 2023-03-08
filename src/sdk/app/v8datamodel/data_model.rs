use std::ops::Deref;

use anyhow::Result;

use crate::sdk::app::v8tree::*;
use crate::sdk::base::*;

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
    pub fn get() -> Result<*mut Self> {
        let task_scheduler = unsafe { &mut *TaskScheduler::get()? };

        let data_model = unsafe {
            (&mut *(&mut *task_scheduler.get_jobs_by_name("Render")?).arbiter[0] as *mut usize)
                .byte_offset(0x04) as *mut Self
        };

        Ok(data_model)
    }
}
