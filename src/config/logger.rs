// palantir
// HTTP REST API reverse proxy
// Copyright: 2018, Maani Beigy <manibeygi@gmail.com>, 
//                  AASAAM Software Group <info@aasaam.com>
// License: MIT/Apache-2.0
//! # palantir
//!
//! `palantir` is a HTTP REST API reverse proxy. It performs load balance,
//! caching, and health check; and also prevents DDOS and reports metrics 
//! concerning health status of backend servers.
//!
// ------------------ bring external functions/traits -------------------------
extern crate log;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
// -------------------- Configuration log structs/traits ----------------------
/// ConfigLogger struct
pub struct ConfigLogger;

impl log::Log for ConfigLogger {
    /// Enabling Debug log level
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }
    /// Recording logs and printing
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("({}) - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
/// Implementing ConfigLogger and setting it up 
impl ConfigLogger {
    pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
        log::set_max_level(level);
        log::set_boxed_logger(Box::new(ConfigLogger))
    }
}