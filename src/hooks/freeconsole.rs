use std::mem;

use anyhow::Result;
use retour::static_detour;
use retour::StaticDetour;

#[link(name = "kernel32")]
extern "system" {
    fn FreeConsole() -> bool;
}

pub type FnFreeConsole = unsafe extern "system" fn() -> bool;

static_detour! {
    static FreeConsoleHook: unsafe extern "system" fn() -> bool;
}

fn closure() -> bool {
    true
}

pub fn create() -> Result<&'static StaticDetour<FnFreeConsole>> {
    let target: FnFreeConsole = unsafe { mem::transmute(FreeConsole as FnFreeConsole) };
    let detour = unsafe { FreeConsoleHook.initialize(target, closure)? };

    Ok(detour)
}
