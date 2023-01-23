mod free_console;

use anyhow::{Context, Result};

pub unsafe fn create() -> Result<()> {
    free_console::create().context("Failed to create FreeConsole hook!")?;

    Ok(())
}

pub unsafe fn restore() -> Result<()> {
    free_console::restore().context("Failed to restore FreeConsole hook!")?;

    Ok(())
}
