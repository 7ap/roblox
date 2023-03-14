use std::mem;
use std::sync::Once;

use anyhow::Result;
use retour::static_detour;
use retour::StaticDetour;
use shroud::directx::directx11;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Dxgi::*;

pub static mut OUTPUT_WINDOW: Option<HWND> = None;

pub type FnPresent = unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32) -> HRESULT;

static_detour! {
    static PresentHook: unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32) -> HRESULT;
}

fn closure(swap_chain: IDXGISwapChain, sync_interval: u32, flags: u32) -> HRESULT {
    static HOOKED: Once = Once::new();

    HOOKED.call_once(|| unsafe {
        let desc = swap_chain.GetDesc().expect("could not get swap chain desc");

        if desc.OutputWindow.0 == -1 {
            panic!("window handle is invalid");
        }

        OUTPUT_WINDOW = Some(desc.OutputWindow);

        // TODO: Move this back into `hooks.rs`
        super::wndproc::enable().expect("hook `wndproc` should be enabled");
    });

    unsafe { PresentHook.call(swap_chain, sync_interval, flags) }
}

pub fn create() -> Result<&'static StaticDetour<FnPresent>> {
    let target: FnPresent = unsafe { mem::transmute(directx11::methods()?.swapchain_vmt()[8]) };
    let detour = unsafe { PresentHook.initialize(target, closure)? };

    Ok(detour)
}
