use std::mem;

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

pub static mut OLD_WND_PROC: Option<WNDPROC> = None;

pub unsafe extern "stdcall" fn hk_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    crate::overlay::APP.wnd_proc(msg, wparam, lparam);

    CallWindowProcW(OLD_WND_PROC.unwrap(), hwnd, msg, wparam, lparam)
}

pub unsafe fn create() -> Result<()> {
    OLD_WND_PROC = Some(mem::transmute(SetWindowLongA(
        crate::hooks::present::OUTPUT_WINDOW.unwrap(),
        GWLP_WNDPROC,
        hk_wnd_proc as _,
    )));

    Ok(())
}

pub unsafe fn restore() -> Result<()> {
    SetWindowLongA(
        crate::hooks::present::OUTPUT_WINDOW.unwrap(),
        GWLP_WNDPROC,
        OLD_WND_PROC.unwrap().unwrap() as usize as _,
    );

    Ok(())
}
