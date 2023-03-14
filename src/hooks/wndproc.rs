use std::mem;

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

pub static mut WND_PROC: Option<WNDPROC> = None;

fn closure(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        CallWindowProcA(
            WND_PROC.expect("`WND_PROC` is null"),
            hwnd,
            msg,
            wparam,
            lparam,
        )
    }
}

pub fn enable() -> Result<()> {
    unsafe {
        WND_PROC = Some(mem::transmute(SetWindowLongA(
            super::present::OUTPUT_WINDOW.expect("`OUTPUT_WINDOW` should be `HWND`"),
            GWLP_WNDPROC,
            closure as _,
        )));
    };

    Ok(())
}

pub fn disable() -> Result<()> {
    unsafe {
        SetWindowLongA(
            crate::hooks::present::OUTPUT_WINDOW.unwrap(),
            GWLP_WNDPROC,
            WND_PROC
                .expect("`WND_PROC` is null")
                .expect("dawg this is Serious.") as _,
        );
    };

    Ok(())
}
