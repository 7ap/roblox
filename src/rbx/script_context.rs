use std::ptr::NonNull;

use super::constants::script_context;
use super::instance::Instance;
use super::task_scheduler::TaskScheduler;

#[repr(C)]
pub struct ScriptContext(Instance);

impl ScriptContext {
    pub unsafe fn get() -> NonNull<Self> {
        let script_context = *(TaskScheduler::get()
            .as_ref()
            .get_jobs_by_name("WaitingHybridScriptsJob")
            .unwrap()
            .as_ptr()
            .byte_offset(script_context::OFFSET)
            as *const *const usize);

        log::trace!("ScriptContext @ {:#08X?}", script_context.addr());

        NonNull::<ScriptContext>::new(script_context as *mut _).unwrap()
    }
}
