#![feature(strict_provenance)]
#![feature(pointer_byte_offsets)]
#![feature(ptr_from_ref)]

mod hooks;
mod sdk;

use std::thread;
use std::time::Duration;

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::System::Console::*;
use windows::Win32::System::LibraryLoader::*;

use sdk::TaskScheduler;

#[tokio::main]
async unsafe fn main() -> Result<()> {
    hooks::attach()?;
    AllocConsole();

    println!("TaskScheduler @ {:p}", TaskScheduler::get());
    TaskScheduler::get().as_ref().print_jobs();
    thread::sleep(Duration::from_secs(15)); // TODO: Hold thread until `END` is pressed.

    hooks::detach()?;
    FreeConsole();

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
