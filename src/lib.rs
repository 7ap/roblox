use std::thread;

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::*;

#[tokio::main]
async unsafe fn main() -> Result<()> {
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
