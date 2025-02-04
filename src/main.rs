// #![deny(warnings)]
mod routes;
use actix_files::NamedFile;
use routes::actix_routes as custom_routes;
use actix_web::{body, get, middleware, rt, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error};
use tokio::sync::broadcast;
use actix_web::middleware::Logger;
use env_logger::Env;
use awc::{Client, ws};
use futures_util::{SinkExt as _, StreamExt as _};
use log::{debug, error, info};

#[tokio::main(flavor="current_thread")]     // configures the tokio runtime to use the single-threaded current-thread runtime
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Starting http server at http://localhost:5000");
    let (tx, _) = broadcast::channel::<web::Bytes>(128);

    /**
     * within closure, move means capture by value rather than reference
     */
    HttpServer::new(move || {
        App::new()
        // websocket UI html file
            // .service(web::resource("/").to(index))

            // // websocket routes
            // // echo_heartbeat_ws handler is attached to http:/localhost:4000/ws
            // // this is where attaching routes manually is helpful
            // .service(web::resource("/ws").route(web::get().to(echo_heartbeat_ws)))
            // .service(web::resource("/ws-basic").route(web::get().to(echo_ws)))
            // .service(web::resource("/ws-broadcast").route(web::get().to(broadcast_ws)))
            // .service(web::resource("/send").route(web::post().to(send_to_broadcast_ws)))
            
            .service(custom_routes::RootRoute)
            .service(custom_routes::echo)
            .service(custom_routes::TestGet)
            .wrap(Logger::default())
            .route("/manualRoute", web::get().to(custom_routes::manual_hello))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

use reqwest;

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App, http::StatusCode};
    use super::*;       // uses immediate parent libraries

    // // to print out the body's content
    // trait BodyTest {
    //     fn as_str(&self) -> &str;
    // }

    // impl BodyTest for Bytes

    #[actix_web::test]      // test 1 (tests all get routes here)
    async fn test_index_get() {
        let app = test::init_service(
            App::new()
                .service(custom_routes::RootRoute)
                .service(custom_routes::TestGet)).await;

        // default() is get
        // let req = test::TestRequest::default().insert_header(ContentType::plaintext()).to_request();
        let request1 = test::TestRequest::get().uri("/").to_request();
        let request2 = test::TestRequest::get().uri("/test_get").to_request();

        let response1 = test::call_service(&app, request1).await;
        let response2 = test::call_service(&app, request2).await;

        // let response1_body = response1.take_body();
        // let response1_body = response1_body.as_ref().unwrap();
        // // let res1 = test::call_service(&app, req).await;
        // println!("{}",format!("response 2 : {:?}", response2.body()));

        assert_eq!(response1.status(), StatusCode::OK);
        assert_eq!(response2.status(), StatusCode::OK);

        let response1_body = test::read_body(response1).await;
        println!("{}",format!("response 1 : {:?}", response1_body));

        let response2_body = test::read_body(response2).await;
        println!("{}",format!("response 2 : {:?}", response2_body));
    }

    // #[actix_web::test] 
    // async fn test_index_post() {
    //     let 

    // }
}

