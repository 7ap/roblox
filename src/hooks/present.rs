use std::sync::Once;

use detour::static_detour;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Dxgi::*;

pub static mut OUTPUT_WINDOW: Option<HWND> = None;

static_detour! {
    pub static Present: unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32) -> HRESULT;
}

pub type Present = unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32) -> HRESULT;

pub fn hk_present(swap_chain: IDXGISwapChain, sync_interval: u32, flags: u32) -> HRESULT {
    unsafe {
        static SETUP: Once = Once::new();

        SETUP.call_once(|| {
            let desc = swap_chain.GetDesc().unwrap();

            if desc.OutputWindow.0 == -1 {
                panic!("Window handle is invalid.");
            }

            OUTPUT_WINDOW = Some(desc.OutputWindow);

            // This is really messy. I'd like this to be in hooks::create however it will cause a weird race condition.
            crate::hooks::window_proc::create().unwrap();
        });

        Present.call(swap_chain, sync_interval, flags)
    }
}
