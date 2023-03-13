use std::mem;

use anyhow::Result;
use retour::static_detour;
use retour::StaticDetour;

#[link(name = "kernel32")]
extern "system" {
    fn FreeConsole() -> bool;
}

static_detour! {
    static FreeConsoleHook: unsafe extern "system" fn() -> bool;
}

fn closure() -> bool {
    true
}

pub fn create() -> Result<&'static StaticDetour<unsafe extern "system" fn() -> bool>> {
    let target = unsafe { mem::transmute(FreeConsole as *const extern "system" fn()) };
    let detour = unsafe { FreeConsoleHook.initialize(target, closure)? };

    Ok(detour)
}
