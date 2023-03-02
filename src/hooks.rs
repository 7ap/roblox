mod freeconsole;

use anyhow::Result;

pub unsafe fn attach() -> Result<()> {
    freeconsole::create()?;

    Ok(())
}

pub unsafe fn detach() -> Result<()> {
    freeconsole::destroy()?;

    Ok(())
}
