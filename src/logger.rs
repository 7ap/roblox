use anyhow::Result;
use log::{Level, LevelFilter, Log, Metadata, Record};

struct Logger;

static LOGGER: Logger = Logger;

pub fn init() -> Result<()> {
    log::set_logger(&LOGGER).unwrap();

    if cfg!(debug_assertions) {
        log::set_max_level(LevelFilter::Debug);
    } else {
        log::set_max_level(LevelFilter::Info);
    }

    Ok(())
}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = record.args();

            match record.level() {
                Level::Error => unsafe {
                    crate::overlay::TOASTS.error(message.to_string());

                    println!("[error] {}", message);
                },
                Level::Warn => unsafe {
                    crate::overlay::TOASTS.warning(message.to_string());

                    println!("[warn] {}", message);
                },
                Level::Info => unsafe {
                    crate::overlay::TOASTS.info(message.to_string());

                    println!("[info] {}", message);
                },
                Level::Debug => {
                    println!("[debug] {}", message);
                }
                Level::Trace => {
                    println!("[trace] {}", message);
                }
            }
        }
    }

    fn flush(&self) {}
}
