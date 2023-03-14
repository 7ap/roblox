use std::mem;

use anyhow::Result;
use retour::static_detour;
use retour::StaticDetour;
use shroud::directx::directx11;
use windows::core::*;
use windows::Win32::Graphics::Dxgi::*;

pub type FnResizeBuffers =
    unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32, u32, u32, u32) -> HRESULT;

static_detour! {
    static ResizeBuffersHook: unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32, u32, u32, u32) -> HRESULT;
}

fn closure(
    swap_chain: IDXGISwapChain,
    buffer_count: u32,
    width: u32,
    height: u32,
    new_format: u32,
    swap_chain_flags: u32,
) -> HRESULT {
    unsafe {
        ResizeBuffersHook.call(
            swap_chain,
            buffer_count,
            width,
            height,
            new_format,
            swap_chain_flags,
        )
    }
}

pub fn create() -> Result<&'static StaticDetour<FnResizeBuffers>> {
    let target: FnResizeBuffers =
        unsafe { mem::transmute(directx11::methods()?.swapchain_vmt()[13]) };
    let detour = unsafe { ResizeBuffersHook.initialize(target, closure)? };

    Ok(detour)
}
