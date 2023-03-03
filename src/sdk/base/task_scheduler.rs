use std::ffi::*;
use std::mem;
use std::ptr::{self, NonNull};

use pelite::pattern;
use pelite::pe::{Pe, PeView};
use windows::core::*;
use windows::Win32::System::LibraryLoader::*;

use crate::sdk::base::TaskSchedulerJob;
use crate::utilities;

#[repr(C)]
pub struct TaskScheduler {
    _pad0: [c_char; 0x134],                 // 0x000..0x134
    pub all_jobs: *mut usize,               // 0x134..0x138
    pub currently_running_jobs: *mut usize, // 0x138..0x13C
}

impl TaskScheduler {
    pub unsafe fn get() -> NonNull<TaskScheduler> {
        // TODO: Shorten ASAP, this is garbage.
        static SIGNATURE: &str = "55 8B EC 64 A1 00 00 00 00 6A FF 68 ? ? ? ? 50 64 89 25 00 00 00 00 83 EC 14 64 A1 2C 00 00 00 8B 08 A1 ? ? ? ? 3B 81 08 00 00 00 7F 29 A1 ? ? ? ? 8B 4D F4 64 89 0D 00 00 00 00 8B E5 5D C3 8D 4D E4 E8 ? ? ? ? 68 ? ? ? ? 8D 45 E4 50 E8 ? ? ? ? 68 ? ? ? ? E8 ? ? ? ? 83 C4 04 83 3D ? ? ? ? ? 75 C1 68";

        let base = GetModuleHandleA(PCSTR(ptr::null())).unwrap().0 as usize;
        let view = PeView::module(base as _);

        let scanner = view.scanner();
        let pattern = pattern::parse(SIGNATURE).unwrap();

        let mut save = [0; 8];
        if !scanner.finds_code(&pattern, &mut save) {
            panic!("could not find task scheduler");
        }

        let address = base + save[0] as usize;
        let scheduler: extern "cdecl" fn() -> NonNull<TaskScheduler> = mem::transmute(address);

        scheduler()
    }

    pub unsafe fn get_jobs_info(&self) -> Vec<NonNull<TaskSchedulerJob>> {
        let mut jobs = Vec::new();

        let mut begin = *&self.all_jobs as *mut usize;
        let end = *&self.currently_running_jobs as *mut usize;

        while begin != end {
            jobs.push(NonNull::new(*begin as *mut _).expect("job is null"));

            begin = begin.byte_offset(0x08);
        }

        jobs
    }

    pub unsafe fn get_jobs_by_name(&self, name: &str) -> Option<NonNull<TaskSchedulerJob>> {
        let jobs = self.get_jobs_info();

        for job in jobs {
            if utilities::read_string(ptr::addr_of!(job.as_ref().name)) == name {
                return Some(job);
            }
        }

        None
    }

    pub unsafe fn print_jobs(&self) {
        let jobs = self.get_jobs_info();

        for job in jobs {
            println!(
                "TaskScheduler::Job::{}, state: {}, seconds spend in job: {}", // the typo is for realism™
                utilities::read_string(ptr::addr_of!(job.as_ref().name)), // TODO: Fix `sdk::read_string` to accept `job.as_ref().name`. I don't like `ptr::addr_of!`.
                "TODO", // TODO: Find `state`, shouldn't take very long but it's 00:17 and I want to commit something.
                job.as_ref().step_start_time, // TODO: Figure out if this offset is correct (looks right?). See above comment.
            );
        }
    }
}
