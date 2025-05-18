mod routes;
use actix_files::NamedFile;
use routes::{auth_routes, rmp_routes, course_routes};
use actix_web::{body, get, middleware, rt, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error};
use tokio::sync::broadcast;
use actix_web::middleware::Logger;
use env_logger::Env;
use log::{ info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Starting http server at http://localhost:5000");
    let (tx, _) = broadcast::channel::<web::Bytes>(128);
    HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().limit(4096))       
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
            .service(course_routes::get_department)
            .service(course_routes::retrieve_course_list_based_on_department)
            .route("/manualRoute", web::get().to(auth_routes::manual_hello))
            .route("/test_json", web::post().to(auth_routes::enter_username_info))
    })
    .bind(("127.0.0.1", 5000))?     // self-reference the current device itself
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App, http::StatusCode};
    use super::*;       

    #[actix_web::test]      
    async fn test_index_get() {
        let app = test::init_service(
            App::new()
                .service(auth_routes::RootRoute)
                .service(auth_routes::TestGet)).await;

        let request1 = test::TestRequest::get().uri("/").to_request();
        let request2 = test::TestRequest::get().uri("/test_get").to_request();

        let response1 = test::call_service(&app, request1).await;
        let response2 = test::call_service(&app, request2).await;
        assert_eq!(response1.status(), StatusCode::OK);
        assert_eq!(response2.status(), StatusCode::OK);

        let response1_body = test::read_body(response1).await;
        println!("{}",format!("response 1 : {:?}", response1_body));

        let response2_body = test::read_body(response2).await;
        println!("{}",format!("response 2 : {:?}", response2_body));
    }
}

