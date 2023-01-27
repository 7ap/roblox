#![feature(strict_provenance)]
#![feature(ptr_from_ref)]
#![feature(pointer_byte_offsets)]

mod hooks;
mod logger;
mod overlay;
mod rbx;

use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

use crate::rbx::data_model::DataModel;
use crate::rbx::script_context::ScriptContext;
use crate::rbx::task_scheduler::TaskScheduler;

unsafe fn main() -> Result<()> {
    let start_time = Instant::now();

    logger::init()?;
    hooks::create()?;

    log::info!("Initialized in {}ms.", start_time.elapsed().as_millis());

    let task_scheduler = TaskScheduler::get().as_ref();
    let script_context = ScriptContext::get().as_ref();
    let data_model = DataModel::get().as_ref();

    task_scheduler.print_jobs();

    while !GetAsyncKeyState(VK_END.0.into()) & 0x01 == 0x01 {
        thread::sleep(Duration::from_millis(50));
    }

    hooks::restore()?;

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
