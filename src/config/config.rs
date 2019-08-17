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
// use std::net::SocketAddr;
use serde::Deserialize;
// ------------------ bring internal functions/traits -------------------------
use super::defaults;
// -------------------------- Configuration structs ---------------------------
/// Config Struct
#[derive(Deserialize)]
pub struct Config {
    pub upstream: ConfigUpstream,
    pub palantir: ConfigPalantir,
}
/// Upstream Config Struct
#[derive(Deserialize)]
pub struct ConfigUpstream {
    #[serde(default = "defaults::upstream_inet")]
    pub inet: String,
     #[serde(default = "defaults::upstream_timeout")]
    pub timeout: u64,
}
/// Reverse Proxy Config Struct
#[derive(Deserialize)]
pub struct ConfigPalantir {
    #[serde(default = "defaults::palantir_log_level")]
    pub log_level: String,

    #[serde(default = "defaults::palantir_inet")]
    pub inet: String,

    #[serde(default = "defaults::palantir_workers")]
    pub workers: usize,
}
