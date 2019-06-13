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
// mod config;
// mod health;
// mod cache;
// mod header;
// mod metrics;
// ------------------ bring external libraries/crates -------------------------
extern crate actix_web;
extern crate futures;
// ------------------ bring external functions/traits -------------------------
use std::time::Duration;
// ------------------ bring internal functions/traits -------------------------
use proxy::proxy::PalantirProxy;
use pool::pool::ThreadPool;
// ---------------------- main functions of palantir --------------------------
fn palantir_one(req: actix_web::HttpRequest) -> 
    impl futures::Future<Item=actix_web::HttpResponse, Error=actix_web::Error> {
        PalantirProxy::new("http://127.0.0.1:9002")
        .timeout(Duration::from_secs(1))
        .forward(req)
}

fn main() {
    let pool = ThreadPool::new(7000);
    pool.execute(|| {
        actix_web::server::new(|| actix_web::App::new()
        .resource("/", |r| r.with_async(palantir_one))
        )
        .bind("0.0.0.0:8080")
        .unwrap()
        .run();
        });
    
}