use std::ffi::*;
use std::mem;
use std::ptr::{self, NonNull};

use pelite::pattern;
use pelite::pe::{Pe, PeView};
use windows::core::*;
use windows::Win32::System::LibraryLoader::*;

use super::constants::task_scheduler;
use crate::utilities;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TaskSchedulerJob {
    vtable: *const usize,  // 0x0000
    this: *const Self,     // 0x0004
    _pad0: [c_char; 0x08], // 0x0008..0x000C
    name: usize,           // 0x0010
}

impl TaskSchedulerJob {
    // TODO: Remove this. This is ugly.
    pub unsafe fn get_name(&self) -> String {
        utilities::read_string(ptr::addr_of!(self.name))
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TaskScheduler; // TODO: Reconstruct

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
        log::trace!("TaskScheduler::get @ {:#08X?}", address);

        let get_task_scheduler: extern "cdecl" fn() -> *const usize = mem::transmute(address);
        log::trace!("TaskScheduler @ {:#08X?}", get_task_scheduler() as usize);

        NonNull::<TaskScheduler>::new((get_task_scheduler()) as *mut _).unwrap()
    }

    pub unsafe fn get_jobs_info(&self) -> Vec<NonNull<TaskSchedulerJob>> {
        let mut jobs = Vec::new();

        let mut job =
            *(ptr::from_ref(self).byte_offset(task_scheduler::JOBS) as *const *const usize);
        let end =
            *(ptr::from_ref(self).byte_offset(task_scheduler::JOBS + 0x04) as *const *const usize);

        log::trace!("Job @ {:#08X?}", job.addr());
        log::trace!("End @ {:#08X?}", end.addr());

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
