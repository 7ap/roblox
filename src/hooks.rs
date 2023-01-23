mod free_console;
mod present;
mod resize_buffers;
mod window_proc;

use std::mem;

use anyhow::{Context, Result};
use shroud::directx::directx11;

pub unsafe fn create() -> Result<()> {
    let directx11_methods = directx11::methods()?;

    let present_address = directx11_methods.swapchain_vmt()[8];
    let resize_buffers_address = directx11_methods.swapchain_vmt()[13];

    let present: present::Present = mem::transmute(present_address);
    let resize_buffers: resize_buffers::ResizeBuffers = mem::transmute(resize_buffers_address);

    free_console::create().context("Failed to create FreeConsole hook!")?;

    present::Present
        .initialize(present, present::hk_present)
        .context("Failed to initialize Present hook!")?
        .enable()
        .context("Failed to enable Present hook!")?;

    resize_buffers::ResizeBuffers
        .initialize(resize_buffers, resize_buffers::hk_resize_buffers)
        .context("Failed to initialize ResizeBuffers hook!")?
        .enable()
        .context("Failed to enable ResizeBuffers hook!")?;

    Ok(())
}

pub unsafe fn restore() -> Result<()> {
    resize_buffers::ResizeBuffers
        .disable()
        .context("Failed to disable ResizeBuffers hook!")?;

    window_proc::restore().context("Failed to restore WindowProc hook!")?;

    present::Present
        .disable()
        .context("Failed to disable Present hook!")?;

    free_console::restore().context("Failed to restore FreeConsole hook!")?;

    Ok(())
}
