use std::mem;
use std::ptr::{self, NonNull};

use pelite::pattern;
use pelite::pe::{Pe, PeView};
use windows::core::*;
use windows::Win32::System::LibraryLoader::*;

#[repr(C)]
pub struct TaskScheduler;

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
            panic!("Failed to get TaskScheduler!");
        }

        let address = base + save[0] as usize;
        let scheduler: extern "cdecl" fn() -> NonNull<TaskScheduler> = mem::transmute(address);

        scheduler()
    }
}
