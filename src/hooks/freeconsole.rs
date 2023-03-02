use std::mem;

use anyhow::Result;
use retour::static_detour;

#[link(name = "kernel32")]
extern "system" {
    fn FreeConsole() -> bool;
}

static_detour! {
    static FreeConsoleHook: unsafe extern "system" fn() -> bool;
}

type FnFreeConsole = unsafe extern "system" fn() -> bool;

fn closure() -> bool {
    true
}

pub unsafe fn create() -> Result<()> {
    let target: FnFreeConsole = mem::transmute(FreeConsole as *const extern "system" fn());
    FreeConsoleHook.initialize(target, closure)?.enable()?;

    Ok(())
}

pub unsafe fn destroy() -> Result<()> {
    FreeConsoleHook.disable()?;

    Ok(())
}
