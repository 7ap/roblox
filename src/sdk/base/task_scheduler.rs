use std::mem;
use std::ptr;

use pelite::pattern;
use pelite::pe::{Pe, PeView};
use windows::core::*;
use windows::Win32::System::LibraryLoader::*;

use crate::sdk::base::*;

#[repr(C)]
pub struct TaskScheduler;

impl TaskScheduler {
    pub fn get() -> *mut Self {
        static SIGNATURE: &str = "55 8B EC 64 A1 00 00 00 00 6A FF 68 ? ? ? ? 50 64 89 25 00 00 00 00 83 EC 14 64 A1 2C 00 00 00 8B 08 A1 ? ? ? ? 3B 81 08 00 00 00 7F 29 A1 ? ? ? ? 8B 4D F4 64 89 0D 00 00 00 00 8B E5 5D C3 8D 4D E4 E8 ? ? ? ? 68 ? ? ? ? 8D 45 E4 50 E8 ? ? ? ? 68 ? ? ? ? E8 ? ? ? ? 83 C4 04 83 3D ? ? ? ? ? 75 C1 68";

        let base = unsafe { GetModuleHandleA(PCSTR(ptr::null())).unwrap().0 as usize };
        let view = unsafe { PeView::module(base as _) };

        let scanner = view.scanner();
        let pattern = pattern::parse(SIGNATURE).unwrap();

        let mut save = [0];
        if !scanner.finds_code(&pattern, &mut save) {
            panic!("could not find task scheduler");
        }

        unsafe {
            mem::transmute::<usize, unsafe extern "cdecl" fn() -> *mut TaskScheduler>(
                base + save[0] as usize,
            )()
        }
    }

    pub fn get_jobs_info(&self) -> Vec<*mut TaskSchedulerJob> {
        todo!()
    }

    pub fn get_jobs_by_name(&self, name: &str) -> Option<*mut TaskSchedulerJob> {
        todo!()
    }

    pub fn print_jobs(&self) {
        unimplemented!()
    }
}
