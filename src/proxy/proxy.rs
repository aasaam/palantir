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
extern crate actix_web;
extern crate futures;
// ------------------ bring external functions/traits -------------------------
use actix_web::{HttpRequest, HttpResponse, HttpMessage, client};
use futures::{Stream, Future};
use std::time::Duration;
use crate::config::defaults;
/// PalantirProxy struct
pub struct PalantirProxy<'a> {
    forward_url: &'a str,
    timeout: Duration,
}

impl<'a> PalantirProxy<'a> {
    /// Making a new PalantirProxy
    pub fn new(forward_url: &'a str) -> PalantirProxy<'a> {
        PalantirProxy{ 
            forward_url, 
            timeout: Duration::from_secs(defaults::upstream_timeout()) 
            }
    }
    /// Implementing timeout
    pub fn timeout(mut self, duration: Duration) -> PalantirProxy<'a> {
        self.timeout = duration;
        self
    }
    /// forwarding uri
    fn forward_uri(&self, req: &HttpRequest) -> String {
        let forward_url: &str = self.forward_url;

        let forward_uri = match req.uri().query() {
            Some(query) => format!(
                "{}{}?{}", forward_url, req.uri().path(), query
                ),
            None => format!(
                "{}{}", forward_url, req.uri().path()
                ),
        };

        forward_uri
    }
    /// Forwarding HTTP request
    pub fn forward(&self, req: HttpRequest) -> 
        impl Future<Item=actix_web::HttpResponse, Error=actix_web::Error>  {

        let mut forward_req = client::ClientRequest::build_from(&req);
        forward_req.uri(self.forward_uri(&req).as_str());

        let forward_body = req.payload().from_err();
        let forward_req = forward_req
                                    //.no_default_headers()
                                    //.set_header_if_none(
                                    //    actix_web::http::header::USER_AGENT, ""
                                    //    )
                                    .body(actix_web::Body::Streaming(
                                        Box::new(forward_body))
                                        )
                                    .expect("Valid forward request needed");

        forward_req.send()
                    .timeout(self.timeout)
                    .map_err(|error| {
                        error.into()
                    })
                    .map(|resp| {
                        let mut back_rsp = HttpResponse::build(resp.status());

                        let back_body = resp.payload().from_err();
                        let back_rsp = back_rsp
                            .body(actix_web::Body::Streaming(
                                Box::new(back_body))
                                );

                        back_rsp
                    })
    }
    
}