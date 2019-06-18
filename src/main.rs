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
mod pool;
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
#[macro_use]
extern crate serde_derive;
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
// use pool::pool::ThreadPool;
// ---------------------- main functions of palantir --------------------------

struct AppArgs {
    config: String,
}

pub static LINE_FEED: &'static str = "\r\n";

lazy_static! {
    static ref APP_ARGS: AppArgs = make_app_args();
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
                .default_value("./config.cfg")
                .takes_value(true),
        )
        .get_matches();

    // Generate owned app arguments
    AppArgs {
        config: String::from(matches.value_of("config").expect("invalid config value")),
    }
}


fn ensure_states() {
    // Ensure all statics are valid (a `deref` is enough to lazily initialize them)
    let (_, _) = (APP_ARGS.deref(), APP_CONF.deref());
}

fn palantir_one(req: actix_web::HttpRequest) -> 
    impl futures::Future<Item=actix_web::HttpResponse, Error=actix_web::Error> {
        let proxaddr: String = APP_CONF.upstream.inet.to_owned();
        let proxaddr_slice: &str = &proxaddr[..];
        PalantirProxy::new(proxaddr_slice)
            .timeout(Duration::from_secs(1))
            .forward(req)
    }

fn main() {
    let _logger = ConfigLogger::init(
        LevelFilter::from_str(&APP_CONF.palantir.log_level).expect("invalid log level"),
        );
    // Ensure all states are bound
    ensure_states();
    actix_web::server::new(|| actix_web::App::new()
        .resource("/", |r| r.with_async(palantir_one))
        )
        .bind(&APP_CONF.palantir.inet)
        .unwrap()
        .run();
    
}