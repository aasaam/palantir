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
// ------------------ bring external functions/traits -------------------------
use std::time::Duration;
// ------------------ bring internal functions/traits -------------------------
use crate::proxy::proxy::PalantirProxy;
use crate::connect::appargs;

pub fn connect_upstream(req: actix_web::HttpRequest) -> 
    impl futures::Future<Item=actix_web::HttpResponse, Error=actix_web::Error> {

        // This function makes a new PalantirProxy struct based on the config
        PalantirProxy::new(&appargs::APP_CONF.upstream.inet.to_owned())
            .timeout(Duration::from_secs(appargs::APP_CONF.upstream.timeout))
            .forward(req)
    }