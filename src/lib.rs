#![feature(strict_provenance)]
#![feature(ptr_from_ref)]
#![feature(pointer_byte_offsets)]

mod console;
mod rbx;

use std::thread;

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::*;

use crate::rbx::instance::DataModel;
use crate::rbx::task_scheduler::TaskScheduler;

unsafe fn main() -> Result<()> {
    console::attach();

    log::info!("Hello, world!");

    loop {
        let input = console::input("> ");
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let command: Vec<&str> = input.split_whitespace().collect();
        match command[0].to_lowercase().as_str() {
            "print_jobs" => {
                TaskScheduler::get().print_jobs();
            }
            "print_datamodel" => {
                for child in DataModel::get().get_children().iter() {
                    log::info!("{} @ {:#08X?}", child.get_name(), child.this.addr());
                }
            }
            "exit" => {
                break;
            }
            _ => {
                log::error!("\"{}\" is an invalid command.", command[0]);
            }
        }
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
