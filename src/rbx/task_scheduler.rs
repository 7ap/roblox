use std::ffi::*;
use std::mem;
use std::ptr::{self, NonNull};

use pelite::pattern;
use pelite::pe::{Pe, PeView};
use windows::core::*;
use windows::Win32::System::LibraryLoader::*;

use super::constants::task_scheduler;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TaskSchedulerJob {
    /// Virtual method table.
    functions: *const usize,
    /// Ignore.
    _pad0: [c_char; 0x0C],
    /// Name of this job.
    name: usize,
}

impl TaskSchedulerJob {
    pub unsafe fn get_name(&self) -> String {
        let name = ptr::addr_of!(self.name);

        if *(name.byte_offset(0x10) as *const usize) < 16 {
            let name = name as *const c_char;
            return CStr::from_ptr(name).to_string_lossy().to_string();
        }

        let name = *(name as *const *const c_char);
        CStr::from_ptr(name).to_string_lossy().to_string()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TaskScheduler; // TODO: Reconstruct struct

impl TaskScheduler {
    pub unsafe fn get() -> NonNull<Self> {
        let base = GetModuleHandleA(PCSTR(ptr::null())).unwrap().0;
        let view = PeView::module(base as _);

        let scanner = view.scanner();
        let pattern = pattern::parse(task_scheduler::GET_TASK_SCHEDULER).unwrap();

        let mut save = [0; 8];
        if !scanner.finds_code(&pattern, &mut save) {
            log::error!("Failed to get TaskScheduler!");
        }

        let address = base as usize + save[0] as usize;
        log::debug!("TaskScheduler::get @ {:#08X?}", address);

        let get_task_scheduler: extern "cdecl" fn() -> *const usize = mem::transmute(address);
        log::debug!("TaskScheduler @ {:#08X?}", get_task_scheduler() as usize);

        NonNull::<TaskScheduler>::new((get_task_scheduler()) as *mut _).unwrap()
    }

    pub unsafe fn get_jobs_info(&self) -> Vec<NonNull<TaskSchedulerJob>> {
        let mut jobs = Vec::new();

        let mut job =
            *(ptr::from_ref(self).byte_offset(task_scheduler::JOBS) as *const *const usize);
        let end =
            *(ptr::from_ref(self).byte_offset(task_scheduler::JOBS + 0x04) as *const *const usize);

        log::debug!("Job @ {:#08X?}", job.addr());
        log::debug!("End @ {:#08X?}", end.addr());

        while job != end {
            jobs.push(NonNull::<TaskSchedulerJob>::new(*job as *mut _).unwrap());
            job = job.byte_offset(0x08);
        }

        jobs
    }

    pub unsafe fn get_jobs_by_name(&self, job_name: &str) -> Option<NonNull<TaskSchedulerJob>> {
        let jobs = self.get_jobs_info();

        for job in jobs {
            let name = job.as_ref().get_name();

            if name == job_name {
                return Some(job);
            }
        }

        None
    }

    pub unsafe fn print_jobs(&self) {
        let jobs = self.get_jobs_info();
        let count = jobs.len();

        log::info!("Printing {} jobs...", count);

        for job in jobs {
            let name = job.as_ref().get_name();
            let address = job.addr();

            log::info!("{} @ {:#08X?}", name, address);
        }

        log::info!("Printed {} jobs.", count);
    }
}
