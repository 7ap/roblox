#![feature(strict_provenance)]
#![feature(pointer_byte_offsets)]

mod hooks;
mod logger;
mod rbx;

use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

use rbx::task_scheduler::TaskScheduler;

unsafe fn main() -> Result<()> {
    let start_time = Instant::now();

    logger::init()?;
    hooks::create()?;

    log::info!("Initialized in {}ms.", start_time.elapsed().as_millis());

    let task_scheduler = TaskScheduler::get()?;
    task_scheduler.print_jobs();

    // *(Render + 0x28) = DataModel
    let data_model = *task_scheduler
        .get_jobs_by_name("Render")
        .unwrap()
        .byte_offset(0x28) as *const usize;

    log::info!("DataModel @ {:#08X?}", data_model.addr());

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
