use std::ffi::*;
use std::io::prelude::*;

use log::{Level, LevelFilter, Log, Metadata, Record};

// TODO: Use the windows crate instead of this.
#[link(name = "kernel32")]
extern "system" {
    fn AllocConsole() -> bool;
    fn FreeConsole() -> bool;
    fn VirtualProtect(
        lpAddress: *mut c_void,
        dwSize: usize,
        flNewProtect: u32,
        lpflOldProtect: *mut u32,
    ) -> bool;
}

struct Logger;

static LOGGER: Logger = Logger;

pub unsafe fn attach() {
    let free_console: *const extern "system" fn() -> bool = FreeConsole as _;

    let mut old: u32 = 0;

    VirtualProtect(free_console as *const _ as *mut c_void, 1, 0x40, &mut old);
    *(free_console as *const _ as *mut u8) = 0xC3; // Replace the first byte of `FreeConsole` with a `RET` instruction.
    VirtualProtect(free_console as *const _ as *mut c_void, 1, old, &mut old);

    AllocConsole();
    init_logger();
}

pub unsafe fn detach() {
    let free_console: *const extern "system" fn() -> bool = FreeConsole as _;

    let mut old: u32 = 0;

    VirtualProtect(free_console as *const _ as *mut c_void, 1, 0x40, &mut old);
    *(free_console as *const _ as *mut u8) = 0xFF; // Restore the first byte of `FreeConsole` with a `CALL` instruction.
    VirtualProtect(free_console as *const _ as *mut c_void, 1, old, &mut old);

    FreeConsole();
}

pub fn input(prompt: &str) -> String {
    let mut input = String::new();

    write!(std::io::stdout(), "{}", prompt)
        .map_err(|e| e.to_string())
        .unwrap();

    std::io::stdout()
        .flush()
        .map_err(|e| e.to_string())
        .unwrap();

    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())
        .unwrap();

    input
}

fn init_logger() {
    log::set_logger(&LOGGER).unwrap();

    if cfg!(debug_assertions) {
        log::set_max_level(LevelFilter::Debug);
    } else {
        log::set_max_level(LevelFilter::Info);
    }
}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = record.args();

            match record.level() {
                Level::Error => println!("[error] {}", message),
                Level::Warn => println!("[warn] {}", message),
                Level::Info => println!("[info] {}", message),
                Level::Debug => println!("[debug] {}", message),
                Level::Trace => println!("[trace] {}", message),
            }
        }
    }

    fn flush(&self) {}
}
