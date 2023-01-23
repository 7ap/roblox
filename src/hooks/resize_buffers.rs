use detour::static_detour;
use windows::core::*;
use windows::Win32::Graphics::Dxgi::*;

static_detour! {
    pub static ResizeBuffers: unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32, u32, u32, u32) -> HRESULT;
}

pub type ResizeBuffers =
    unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32, u32, u32, u32) -> HRESULT;

pub fn hk_resize_buffers(
    swap_chain: IDXGISwapChain,
    buffer_count: u32,
    width: u32,
    height: u32,
    new_format: u32,
    swap_chain_flags: u32,
) -> HRESULT {
    unsafe {
        ResizeBuffers.call(
            swap_chain,
            buffer_count,
            width,
            height,
            new_format,
            swap_chain_flags,
        )
    }
}
