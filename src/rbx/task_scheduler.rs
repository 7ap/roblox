use std::ffi::*;
use std::mem;

use anyhow::Result;
use pelite::pattern;
use pelite::pe::{Pe, PeView};
use windows::core::*;
use windows::Win32::System::LibraryLoader::*;

use super::constants::task_scheduler as constants;

pub struct TaskScheduler {
    task_scheduler: *const usize,
}

impl TaskScheduler {
    pub unsafe fn get() -> Result<Self> {
        let base = GetModuleHandleA(PCSTR(std::ptr::null())).unwrap().0;
        let view = PeView::module(base as _);

        let scanner = view.scanner();
        let pattern = pattern::parse(constants::GET_TASK_SCHEDULER).unwrap();

        let mut save = [0; 8];
        if !scanner.finds_code(&pattern, &mut save) {
            log::error!("Failed to get TaskScheduler!");
        }

        let get_task_scheduler = (base as usize + save[0] as usize) as *const usize;
        let get_task_scheduler: constants::GetTaskScheduler = mem::transmute(get_task_scheduler);
        let task_scheduler = get_task_scheduler();

        log::info!("TaskScheduler @ {:#08X?}", task_scheduler.addr());

        Ok(Self { task_scheduler })
    }

    // TODO: This works for what I need it to do, however a few job names are still weird. This should be fixed if it becomes an issue.
    unsafe fn get_job_name(job: *const usize) -> String {
        let name = job.byte_offset(constants::NAME) as *const c_char;
        let name = CStr::from_ptr(name).to_str();

        if name.is_err() {
            let name = *(job.byte_offset(constants::NAME) as *const *const c_char);
            return String::from_utf8_lossy(CStr::from_ptr(name).to_bytes()).to_string();
        }

        name.unwrap().to_string()
    }

    pub unsafe fn get_jobs_info(&self) -> Vec<*const usize> {
        let mut jobs = Vec::new();

        let mut job = *(self.task_scheduler as *const *const usize).byte_offset(constants::JOB);
        let end_job = *(self.task_scheduler as *const *const usize).byte_offset(constants::END);

        while job != end_job {
            jobs.push(*job as *const usize);
            job = job.byte_offset(0x08);
        }

        jobs
    }

    pub unsafe fn get_jobs_by_name(&self, job_name: &str) -> Option<*const usize> {
        for &job in self.get_jobs_info().iter() {
            if Self::get_job_name(job) == job_name {
                return Some(job);
            }
        }

        None
    }

    pub unsafe fn print_jobs(&self) {
        log::info!("Printing jobs...");

        let jobs = self.get_jobs_info();

        for &job in jobs.iter() {
            let job_name = Self::get_job_name(job);
            log::info!("{} @ {:#08X?}", job_name, job.addr());
        }

        log::info!("Printed {} jobs.", jobs.len());
    }
}
