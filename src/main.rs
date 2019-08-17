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
// ----------------------------- bring Modules --------------------------------
mod proxy;
// mod pool;
mod config;
mod connection;
// mod health;
// mod cache;
// mod header;
// mod metrics;
// ------------------ bring external libraries/crates -------------------------
extern crate actix_web;
extern crate futures;
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate toml;
// ------------------ bring external functions/traits -------------------------
use std::ops::Deref;
use std::str::FromStr;
use log::LevelFilter;
// ------------------ bring internal functions/traits -------------------------
use config::logger::ConfigLogger;
use connection::connection::connect_upstream;
use connection::appargs;
//use pool::pool::ThreadPool;
// ---------------------- main functions of palantir --------------------------
/// This function ensures all statics are valid (a `deref` is enough to lazily 
/// initialize them)
fn ensure_states() {
    let (_, _) = (appargs::APP_ARGS.deref(), appargs::APP_CONF.deref());
}

/// The main function running reverse proxy
fn main() {
    let _logger = ConfigLogger::init(
        LevelFilter::from_str(
            &appargs::APP_CONF.palantir.log_level).expect("invalid log level"
            ),
        );
    // Ensure all states are bound
    ensure_states();
        actix_web::server::new(
            || actix_web::App::new()
                .resource("/{tail:.*}", |r| r.with_async(connect_upstream))
            )
            .bind(&appargs::APP_CONF.palantir.inet)
            .unwrap()
            .workers(appargs::APP_CONF.palantir.workers)
            .run(); 
    
}