use std::ffi::*;

use anyhow::Result;

// TODO: Use the windows crate instead of this.
#[link(name = "kernel32")]
extern "system" {
    fn AllocConsole() -> bool;
    fn FreeConsole() -> bool;
    fn VirtualProtect(
        lpAddress: *mut c_void,
        dwSize: usize,
        flNewProtect: u32,
        lpflOldProtect: *mut u32,
    ) -> bool;
    fn SetConsoleTitleA(lpConsoleTitle: *const u8) -> bool;
}

pub unsafe fn create() -> Result<()> {
    let free_console: *const extern "system" fn() -> bool = FreeConsole as _;

    let mut old: u32 = 0;

    VirtualProtect(free_console as *const _ as *mut c_void, 1, 0x40, &mut old);
    *(free_console as *const _ as *mut u8) = 0xC3; // Replace the first byte of `FreeConsole` with a `RET` instruction.
    VirtualProtect(free_console as *const _ as *mut c_void, 1, old, &mut old);

    AllocConsole();
    SetConsoleTitleA(b"Console\0".as_ptr());

    Ok(())
}

pub unsafe fn restore() -> Result<()> {
    let free_console: *const extern "system" fn() -> bool = FreeConsole as _;

    let mut old: u32 = 0;

    VirtualProtect(free_console as *const _ as *mut c_void, 1, 0x40, &mut old);
    *(free_console as *const _ as *mut u8) = 0xFF; // Restore the first byte of `FreeConsole` with a `CALL` instruction.
    VirtualProtect(free_console as *const _ as *mut c_void, 1, old, &mut old);

    // We're back to normal!
    FreeConsole();

    Ok(())
}
