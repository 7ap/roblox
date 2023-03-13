mod freeconsole;

use anyhow::Result;
use retour::StaticDetour;

pub struct Hooks {
    freeconsole: &'static StaticDetour<unsafe extern "system" fn() -> bool>,
}

impl Hooks {
    pub fn new() -> Self {
        Hooks {
            freeconsole: freeconsole::create().expect("hook `freeconsole` should be created"),
        }
    }

    pub fn enable(&self) -> Result<()> {
        unsafe {
            self.freeconsole
                .enable()
                .expect("hook `freeconsole` should be enabled");
        };

        Ok(())
    }

    pub fn disable(&self) -> Result<()> {
        unsafe {
            self.freeconsole
                .disable()
                .expect("hook `freeconsole` should be disabled");
        };

        Ok(())
    }
}
