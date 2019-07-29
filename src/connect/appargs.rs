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
/// This function connects to uppstream server and forwards url
// ----------------------------- bring Modules --------------------------------
//mod crate::proxy;
// mod pool;
//mod config;
//mod connect;
// mod health;
// mod cache;
// mod header;
// mod metrics;
// ------------------ bring external functions/traits -------------------------
use clap::{Arg, App};
// ------------------ bring internal functions/traits -------------------------
use crate::config::config::Config;
use crate::config::reader::ConfigReader;

/// Application Arguments
pub struct AppArgs {
    pub config: String,
}

lazy_static! {
    /// Declaring lazily evaluated statics for app arguments
    pub static ref APP_ARGS: AppArgs = make_app_args();
    /// Declaring lazily evaluated statics for app configurations
    pub static ref APP_CONF: Config = ConfigReader::make();
}

pub fn make_app_args() -> AppArgs {
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