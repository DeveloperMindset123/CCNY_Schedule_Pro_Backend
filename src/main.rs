// TODO : implement Actix API endpoints to retrieve JSON data as input
// TODO : determine a function that can convert given JSOn data into appropriate data container to push into diesel table.
// #![deny(warnings)]
mod routes;
use actix_files::NamedFile;
use routes::{auth_routes, rmp_routes, course_routes};
use actix_web::{body, get, middleware, rt, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error};
use tokio::sync::broadcast;
use actix_web::middleware::Logger;
use env_logger::Env;
use awc::{Client, ws};
use futures_util::{SinkExt as _, StreamExt as _};
use log::{debug, error, info};

// #[tokio::main(flavor="current_thread")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Starting http server at http://localhost:5000");
    let (tx, _) = broadcast::channel::<web::Bytes>(128);

    /**
     * within closure, move means capture by value rather than reference
     */
    HttpServer::new(move || {
        App::new()
        // attach pre-defined routes using the .service()
        // attach manual routes using .route() method (meaning path for the route hanlder functions hasn't been defined)
        // websocket UI html file
            // .service(web::resource("/").to(index))

            // // websocket routes
            // // echo_heartbeat_ws handler is attached to http:/localhost:4000/ws
            // // this is where attaching routes manually is helpful
            // .service(web::resource("/ws").route(web::get().to(echo_heartbeat_ws)))
            // .service(web::resource("/ws-basic").route(web::get().to(echo_ws)))
            // .service(web::resource("/ws-broadcast").route(web::get().to(broadcast_ws)))
            // .service(web::resource("/send").route(web::post().to(send_to_broadcast_ws)))
            .app_data(web::JsonConfig::default().limit(4096))       // limit the size of payload via global configuration
            .wrap(Logger::default())
            .service(auth_routes::RootRoute)
            .service(auth_routes::echo)
            .service(auth_routes::TestGet)
            .service(auth_routes::index_manual)
            .service(auth_routes::signup_handler)
            .service(auth_routes::signin_handler)
            .service(rmp_routes::professor_summary_handler)
            .service(rmp_routes::professor_list_handler)
            .service(rmp_routes::professor_summary_handler)
            .service(rmp_routes::professor_comments_handler)
            .service(course_routes::retrieve_course_info)
            .service(course_routes::retrieve_department_list)
            .service(course_routes::retrieve_course_list)
            .route("/manualRoute", web::get().to(auth_routes::manual_hello))
            .route("/test_json", web::post().to(auth_routes::enter_username_info))
            // .service(web::resource("/json").route(web::post().to(auth_routes::enter_username_info)))
    })
    .bind(("127.0.0.1", 5000))?     // self-reference the current device itself
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
                .service(auth_routes::RootRoute)
                .service(auth_routes::TestGet)).await;

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

