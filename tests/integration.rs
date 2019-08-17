// ------------------ bring external libraries/crates -------------------------
// ----------------------------- bring Modules --------------------------------
//fn connection_server_proxy(req: actix_web::HttpRequest) -> 
//    impl futures::Future<Item=actix_web::HttpResponse, Error=actix_web::Error> {
//        PalantirProxy::new(&String::from(
//            "https://127.0.0.1:9062"))
//            .timeout(std::time::Duration::from_secs(100))
//            .forward(req)
//    }
//#[test]

//fn make_palantir() {
//        use actix_web::test::TestServer;
//        let mut srv = TestServer::new(|_r|
//            actix_web::server::new(
//                || actix_web::App::new()
//                    .resource("/{tail:.*}", |r| r.with_async(
//                        connection_server_proxy
//                        ))
//                    )
//                    .bind(&String::from("0.0.0.0:8080"))
//                    .unwrap()
//                    .run()
//        );
//        let request = srv.get().finish().unwrap();
//        let response = srv.execute(request.send()).unwrap();
//        assert!(response.status().is_success()s);
//}
        