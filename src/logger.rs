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
                Level::Error => {
                    println!("[error] {}", message);
                }
                Level::Warn => {
                    println!("[warn] {}", message);
                }
                Level::Info => {
                    println!("[info] {}", message);
                }
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
