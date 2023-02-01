#![feature(strict_provenance)]
#![feature(ptr_from_ref)]
#![feature(pointer_byte_offsets)]

mod console;
mod rbx;

use std::thread;
use std::usize;

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::*;

use crate::rbx::instance::{DataModel, Instance};
use crate::rbx::task_scheduler::TaskScheduler;

unsafe fn main() -> Result<()> {
    console::attach();

    loop {
        let input = console::input("> ");
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let command: Vec<&str> = input.split_whitespace().collect();

        // TODO: This is *really* messy. Consider moving this into a submodule of console?
        match command[0].to_lowercase().as_str() {
            "get_jobs" => {
                TaskScheduler::get().print_jobs();
            }
            "get_children" => {
                let instance = if command.len() > 1 {
                    let instance = command[1].trim_start_matches("0x");
                    let instance = usize::from_str_radix(instance, 16);
                    let instance = instance.unwrap() as *mut Instance;

                    &*instance
                } else {
                    DataModel::get()
                };

                for child in instance.get_children().unwrap().iter() {
                    log::info!("{} @ {:p}", child.get_name(), child.this);
                }
            }
            "get_descendants" => {
                let instance = if command.len() > 1 {
                    let instance = command[1].trim_start_matches("0x");
                    let instance = usize::from_str_radix(instance, 16);
                    let instance = instance.unwrap() as *mut Instance;

                    &*instance
                } else {
                    DataModel::get()
                };

                for descendant in instance.get_descendants().unwrap().iter() {
                    log::info!("{} @ {:p}", descendant.get_name(), descendant.this);
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
