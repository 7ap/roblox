use std::ffi::*;
use std::mem;
use std::ptr;

use anyhow::{anyhow, Result};
use pelite::pattern;
use pelite::pe::{Pe, PeView};
use windows::core::*;
use windows::Win32::System::LibraryLoader::*;

use crate::sdk::base::*;

#[repr(C)]
pub struct TaskScheduler {
    _pad0: [c_char; 0x134],                      // 0x000..0x134
    pub all_jobs: [*mut usize; 2],               // 0x134..0x13C
    _pad1: [c_char; 0x008],                      // 0x13C..0x144
    pub currently_running_jobs: [*mut usize; 2], // 0x144..0x14C
}

impl TaskScheduler {
    pub fn get() -> Result<*mut Self> {
        static SIGNATURE: &str = "55 8B EC 64 A1 00 00 00 00 6A FF 68 ? ? ? ? 50 64 89 25 00 00 00 00 83 EC 14 64 A1 2C 00 00 00 8B 08 A1 ? ? ? ? 3B 81 08 00 00 00 7F 29 A1 ? ? ? ? 8B 4D F4 64 89 0D 00 00 00 00 8B E5 5D C3 8D 4D E4 E8 ? ? ? ? 68 ? ? ? ? 8D 45 E4 50 E8 ? ? ? ? 68 ? ? ? ? E8 ? ? ? ? 83 C4 04 83 3D ? ? ? ? ? 75 C1 68";

        let base = unsafe { GetModuleHandleA(PCSTR(ptr::null())).unwrap().0 as usize };
        let view = unsafe { PeView::module(base as _) };

        let scanner = view.scanner();
        let pattern = pattern::parse(SIGNATURE).unwrap();

        let mut save = [0];
        if !scanner.finds_code(&pattern, &mut save) {
            return Err(anyhow!("task scheduler could not be found"));
        }

        Ok(unsafe {
            mem::transmute::<usize, unsafe extern "cdecl" fn() -> *mut TaskScheduler>(
                base + save[0] as usize,
            )()
        })
    }

    pub fn get_jobs_info(&self) -> Vec<*mut TaskSchedulerJob> {
        let mut begin = unsafe { &mut *self.all_jobs[0] as *mut usize };
        let end = unsafe { &mut *self.all_jobs[1] as *mut usize };

        let mut jobs = Vec::new();

        while begin != end {
            jobs.push(unsafe { mem::transmute::<usize, *mut TaskSchedulerJob>(*begin) });

            unsafe { begin = begin.byte_offset(0x08) }
        }

        jobs
    }

    pub fn get_jobs_by_name(&self, name: &str) -> Result<*mut TaskSchedulerJob> {
        let mut begin = unsafe { &mut *self.all_jobs[0] as *mut usize };
        let end = unsafe { &mut *self.all_jobs[1] as *mut usize };

        while begin != end {
            let job = unsafe { &mut *mem::transmute::<usize, *mut TaskSchedulerJob>(*begin) };

            // Atrocious hack. Find better solution at some point.
            let size: [c_char; 4] = job.name[16..20].try_into()?;
            let size: isize = unsafe { mem::transmute_copy(&size) };

            let job_name = if size >= 16 {
                let string = job.name.as_ptr() as *const *const c_char;
                unsafe { CStr::from_ptr(*string) }
            } else {
                let string = job.name.as_ptr() as *const c_char;
                unsafe { CStr::from_ptr(string) }
            };

            if job_name.to_str()? == name {
                return Ok(unsafe { mem::transmute::<usize, *mut TaskSchedulerJob>(*begin) });
            }

            unsafe { begin = begin.byte_offset(0x08) }
        }

        Err(anyhow!("no job found with the name \"{}\"", name))
    }

    pub fn print_jobs(&self) -> Result<()> {
        let mut begin = unsafe { &mut *self.currently_running_jobs[0] as *mut usize };
        let end = unsafe { &mut *self.currently_running_jobs[1] as *mut usize };

        while begin != end {
            let job = unsafe { &mut *mem::transmute::<usize, *mut TaskSchedulerJob>(*begin) };

            // Atrocious hack. Find better solution at some point.
            let size: [c_char; 4] = job.name[16..20].try_into()?;
            let size: isize = unsafe { mem::transmute_copy(&size) };

            let job_name = if size >= 16 {
                let string = job.name.as_ptr() as *const *const c_char;
                unsafe { CStr::from_ptr(*string) }
            } else {
                let string = job.name.as_ptr() as *const c_char;
                unsafe { CStr::from_ptr(string) }
            };

            log::info!(
                "TaskScheduler::Job::{}, state: {}, seconds spend in job: {}",
                job_name.to_str()?,
                "TODO",
                "TODO"
            );

            unsafe { begin = begin.byte_offset(0x08) }
        }

        Ok(())
    }
}
