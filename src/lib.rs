#![feature(strict_provenance)]
#![feature(ptr_from_ref)]
#![feature(pointer_byte_offsets)]

mod console;
mod rbx;

use std::thread;
use std::time::Duration;

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

use crate::rbx::instance::DataModel;
use crate::rbx::task_scheduler::TaskScheduler;

unsafe fn main() -> Result<()> {
    console::attach();

    log::info!("Hello, world!");

    let task_scheduler = TaskScheduler::get();
    let data_model = DataModel::get();

    task_scheduler.print_jobs();

    for child in data_model.get_children().iter() {
        log::info!("{} @ {:#08X?}", child.get_name(), child.this.addr());
    }

    while !GetAsyncKeyState(VK_END.0.into()) & 0x01 == 0x01 {
        thread::sleep(Duration::from_millis(50));
    }

    console::detach();

    Ok(())
}

#[no_mangle]
unsafe extern "system" fn DllMain(module: HINSTANCE, reason: u32, _: usize) -> isize {
    if reason == 1 {
        thread::spawn(move || unsafe {
            match main() {
                Ok(_) => FreeLibraryAndExitThread(module, 0),
                Err(_) => FreeLibraryAndExitThread(module, 1),
            }
        });

        return 1;
    };

    0
}
