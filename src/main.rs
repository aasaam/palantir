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
use std::time::Duration;
use clap::{Arg, App};
use std::ops::Deref;
use std::str::FromStr;
use log::LevelFilter;
// ------------------ bring internal functions/traits -------------------------
use proxy::proxy::PalantirProxy;
use config::config::Config;
use config::logger::ConfigLogger;
use config::reader::ConfigReader;
//use pool::pool::ThreadPool;
// ---------------------- main functions of palantir --------------------------
/// Application Arguments
struct AppArgs {
    config: String,
}

lazy_static! {
    /// Declaring lazily evaluated statics for app arguments
    static ref APP_ARGS: AppArgs = make_app_args();
    /// Declaring lazily evaluated statics for app configurations
    static ref APP_CONF: Config = ConfigReader::make();
}

fn make_app_args() -> AppArgs {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Path to configuration file")
                .default_value("./config.toml")
                .takes_value(true),
        )
        .get_matches();

    // Generate owned app arguments
    AppArgs {
        config: String::from(
            matches.value_of("config").expect("invalid config value")
            ),
    }
}

/// This function ensures all statics are valid (a `deref` is enough to lazily 
/// initialize them)
fn ensure_states() {
    let (_, _) = (APP_ARGS.deref(), APP_CONF.deref());
}
/// This function connects to uppstream server and forwards url
fn connect_upstream(req: actix_web::HttpRequest) -> 
    impl futures::Future<Item=actix_web::HttpResponse, Error=actix_web::Error> {

        // This function makes a new PalantirProxy struct based on the config
PalantirProxy::new(&APP_CONF.upstream.inet.to_owned())
            .timeout(Duration::from_secs(APP_CONF.upstream.timeout))
            .forward(req)
    }

/// The main function running reverse proxy
fn main() {
    let _logger = ConfigLogger::init(
        LevelFilter::from_str(
            &APP_CONF.palantir.log_level).expect("invalid log level"
            ),
        );
    // Ensure all states are bound
    ensure_states();
        actix_web::server::new(
            || actix_web::App::new()
                .resource("/{tail:.*}", |r| r.with_async(connect_upstream))
            )
            .bind(&APP_CONF.palantir.inet)
            .unwrap()
            .run(); 
}