// Implements all the relevant endpoints here.
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, HttpRequest};
// use futures::StreamExt;
use serde::{Serialize, Deserialize};

#[get("/")]
pub async fn RootRoute() -> impl Responder {
    HttpResponse::Ok().body("Basic Hello World Route")
}

#[get("/test_get")]
pub async fn TestGet() -> impl Responder {
    HttpResponse::Ok().body("This is a second test get")
}

#[post("/echo")]
pub async fn echo(req_body : String) -> impl Responder {
    HttpResponse::Ok().body(req_body)      // reply the provided parameter as response
}

// example from Actix Web
// #[derive(Deserialize, Serialize, Debug)]
// pub struct Info {
//     username : String,
// }

// // handler function to enter user information
// pub async fn enter_username_info(info : web::Json<Info>) -> Result<String> {
//     // return type to satisfy Result<String>
//     Ok(format!("Welcome {}!", info.username))
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObj {
    name: String,
    number: i32,
}

/// This handler uses json extractor
pub async fn enter_username_info(item: web::Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

/// This handler uses json extractor with limit
async fn extract_item(item: web::Json<MyObj>, req: HttpRequest) -> HttpResponse {
    println!("request: {req:?}");
    println!("model: {item:?}");

    HttpResponse::Ok().json(item.0) // <- send json response
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("This is a manual hello!")
}

// #[post("/test_json_general")]
// pub async fn enter_username_info_general(mut payload : web::Payload) -> Result<HttpResponse

// dynamic route example
#[post("/{conversation_data}")]
pub async fn send_conversation_to_database() -> impl Responder {
    HttpResponse::Ok().body("Not yet implemented.")
}

// generally passing in JSON data
