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
// ------------------ bring external libraries/crates -------------------------
//use actix_web::test::TestServer;
//use crate::connect_upstream;
//fn main() {
//    let mut srv = TestServer::new(|app| app.handler(connect_upstream));
//    let req = srv.get().finish().unwrap();
//    let response = srv.execute(req.send()).unwrap();
//    assert!(response.status().is_success());
//}
