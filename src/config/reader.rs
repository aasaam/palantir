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
use std::fs::File;
use std::io::Read;
use toml;
// ------------------ bring internal functions/traits -------------------------
use super::config::*;
use crate::connection::appargs;

pub struct ConfigReader;

impl ConfigReader {
    pub fn make() -> Config {
        debug!("reading config file: {}", &appargs::APP_ARGS.config);

        let mut file = File::open(&appargs::APP_ARGS.config).expect(
            "cannot find config file"
            );
        let mut conf = String::new();

        file.read_to_string(&mut conf)
            .expect("cannot read config file");

        debug!("read config file: {}", &appargs::APP_ARGS.config);

        toml::de::from_str(&conf).expect("syntax error in config file")
    }
}