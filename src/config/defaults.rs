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
// -------------------- Default configuration functions -----------------------
pub fn palantir_log_level() -> String {
    "error".to_string()
}

pub fn palantir_inet() -> String {
    "0.0.0.0:8080".to_string()
}

pub fn upstream_inet() -> String {
    "http://127.0.0.1:9002".to_string()
}

pub fn upstream_timeout() -> u64 {
    5
}