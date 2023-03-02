mod hooks;

use std::thread;
use std::time::Duration;

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::System::Console::*;
use windows::Win32::System::LibraryLoader::*;

#[tokio::main]
async unsafe fn main() -> Result<()> {
    hooks::attach()?;

    AllocConsole();
    println!("Hello, world!");
    thread::sleep(Duration::from_secs(10));

    hooks::detach()?;
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
