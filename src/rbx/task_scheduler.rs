use std::mem;
use std::ptr::{self, NonNull};

use pelite::pattern;
use pelite::pe::{Pe, PeView};
use windows::core::*;
use windows::Win32::System::LibraryLoader::*;

use crate::rbx::TaskSchedulerJob;

static GET_TASK_SCHEDULER: &str = "55 8B EC 64 A1 00 00 00 00 6A FF 68 ? ? ? ? 50 64 89 25 00 00 00 00 83 EC 14 64 A1 2C 00 00 00 8B 08 A1 ? ? ? ? 3B 81 08 00 00 00 7F 29 A1 ? ? ? ? 8B 4D F4 64 89 0D 00 00 00 00 8B E5 5D C3 8D 4D E4 E8 ? ? ? ? 68 ? ? ? ? 8D 45 E4 50 E8 ? ? ? ? 68 ? ? ? ? E8 ? ? ? ? 83 C4 04 83 3D ? ? ? ? ? 75 C1 68";

#[repr(C)]
pub struct TaskScheduler; // TODO: Reconstruct

impl TaskScheduler {
    pub unsafe fn get() -> &'static TaskScheduler {
        let base = GetModuleHandleA(PCSTR(ptr::null())).unwrap().0;
        let view = PeView::module(base as _);

        let scanner = view.scanner();
        let pattern = pattern::parse(GET_TASK_SCHEDULER).unwrap();

        let mut save = [0; 8];
        if !scanner.finds_code(&pattern, &mut save) {
            log::error!("Failed to get TaskScheduler!");
        }

        let address = base as usize + save[0] as usize;
        let get_task_scheduler: extern "cdecl" fn() -> *const usize = mem::transmute(address);

        NonNull::<TaskScheduler>::new(get_task_scheduler() as *mut _)
            .expect("`TaskScheduler` is a null pointer")
            .as_mut()
    }

    pub unsafe fn get_jobs_info(&self) -> Vec<&'static mut TaskSchedulerJob> {
        let mut jobs = Vec::new();

        let mut job = *(ptr::from_ref(self).byte_offset(0x134) as *const *const usize);
        let end_job = *(ptr::from_ref(self).byte_offset(0x134 + 0x04) as *const *const usize);

        while job != end_job {
            let current_job = NonNull::<TaskSchedulerJob>::new(*job as *mut _)
                .expect("`TaskSchedulerJob` is a null pointer")
                .as_mut();

            jobs.push(current_job);
            job = job.byte_offset(0x08);
        }

        jobs
    }

    pub unsafe fn get_jobs_by_name(&self, job_name: &str) -> Option<&'static TaskSchedulerJob> {
        let jobs = self.get_jobs_info();

        for job in jobs {
            let name = job.get_name();

            if name == job_name {
                return Some(job);
            }
        }

        None
    }

    pub unsafe fn print_jobs(&self) {
        for job in self.get_jobs_info().iter() {
            log::info!("{} @ {:p}", job.get_name(), ptr::from_ref(job));
        }
    }
}
