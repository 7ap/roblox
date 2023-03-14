mod freeconsole;
mod present;
mod resizebuffers;
mod wndproc;

use anyhow::Result;
use retour::StaticDetour;

pub struct Hooks {
    freeconsole: &'static StaticDetour<freeconsole::FnFreeConsole>,
    present: &'static StaticDetour<present::FnPresent>,
    resizebuffers: &'static StaticDetour<resizebuffers::FnResizeBuffers>,
    wndproc: (),
}

impl Hooks {
    pub fn new() -> Self {
        Hooks {
            freeconsole: freeconsole::create().expect("hook `freeconsole` should be created"),
            present: present::create().expect("hook `present` should be created"),
            resizebuffers: resizebuffers::create().expect("hook `resizebuffers` should be created"),
            wndproc: (),
        }
    }

    pub fn enable(&self) -> Result<()> {
        unsafe {
            self.freeconsole
                .enable()
                .expect("hook `freeconsole` should be enabled");

            self.present
                .enable()
                .expect("hook `present` should be enabled`");

            self.resizebuffers
                .enable()
                .expect("hook `resizebuffers` should be enabled`");

            // wndproc::enable()?;
        };

        Ok(())
    }

    pub fn disable(&self) -> Result<()> {
        unsafe {
            self.freeconsole
                .disable()
                .expect("hook `freeconsole` should be disabled");

            self.present
                .disable()
                .expect("hook `present` should be disabled");

            self.resizebuffers
                .disable()
                .expect("hook `resizebuffers` should be disabled");

            wndproc::disable()?;
        };

        Ok(())
    }
}
