use std::mem;
use std::sync::Once;

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
    static HOOKED: Once = Once::new();

    HOOKED.call_once(|| {
        log::debug!("`freeconsole` hooked!");
    });

    true
}

pub fn create() -> Result<&'static StaticDetour<FnFreeConsole>> {
    let target: FnFreeConsole = unsafe { mem::transmute(FreeConsole as FnFreeConsole) };
    let detour = unsafe { FreeConsoleHook.initialize(target, closure)? };

    Ok(detour)
}
