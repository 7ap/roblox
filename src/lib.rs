#![feature(strict_provenance)]
#![feature(pointer_byte_offsets)]

mod hooks;
mod sdk;

use std::ffi::*;
use std::mem;
use std::ptr;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use env_logger::Env;
use windows::Win32::Foundation::*;
use windows::Win32::System::Console::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    use crate::hooks::Hooks;
    use crate::sdk::app::v8datamodel::*;
    use crate::sdk::base::*;

    env_logger::init_from_env(Env::default().default_filter_or("DEBUG"));
    unsafe { AllocConsole() };
    let hooks = Hooks::new();

    hooks.enable().expect("hooks should be enabled");

    while unsafe { !GetAsyncKeyState(VK_END.0 as i32) & 0x01 } == 0x01 {
        if unsafe { GetAsyncKeyState(VK_Z.0 as i32) & 0x01 } == 0x01 {
            let task_scheduler = unsafe { &mut *TaskScheduler::get()? };
            log::info!("TaskScheduler @ {:p}", task_scheduler);

            for job in task_scheduler.get_jobs_info() {
                let job = unsafe { &mut *job };

                // Atrocious hack. Find better solution at some point.
                let size: [c_char; 4] = job.name[16..20].try_into()?;
                let size: isize = unsafe { mem::transmute_copy(&size) };

                let name = if size >= 16 {
                    let string = job.name.as_ptr() as *const *const c_char;
                    unsafe { CStr::from_ptr(*string) }
                } else {
                    let string = job.name.as_ptr() as *const c_char;
                    unsafe { CStr::from_ptr(string) }
                };

                log::info!("TaskSchedulerJob<{}> @ {:p}", name.to_str()?, job);
            }
        }

        if unsafe { GetAsyncKeyState(VK_X.0 as i32) & 0x01 } == 0x01 {
            let data_model = unsafe { &mut *DataModel::get()? };
            log::info!("DataModel @ {:p}", data_model);

            let class_descriptor = unsafe { &mut *data_model.descriptor };
            log::info!("ClassDescriptor @ {:p}", class_descriptor);

            let property_descriptors = &mut class_descriptor.property_descriptors;
            log::info!("propertyDescriptors @ {:p}", property_descriptors);
        }

        thread::sleep(Duration::from_millis(50));
    }

    hooks.disable().expect("hooks should be disabled");
    unsafe { FreeConsole() };

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
