// Implements all the relevant endpoints here.
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error};
use futures_util::StreamExt;        // originally futures::StreamExt;
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

// TODO : replace with actual data container struct
// this is primarily for example purpose
#[derive(Debug, Serialize, Deserialize, Clone)]
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

// define global function
pub const MAX_SIZE : usize = 262_144;

// example to retrieve general JSON payload data
// use this as template reference for writting api routes that accepts json data
#[post("/index")]
pub async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        println!("current chunk : {chunk:?}");
        
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(actix_web::error::ErrorBadRequest("overflow"));
        }

        println!("body data : {body:?}");

        // extend_from_slice() : appends given bytes to this bytes mut
        body.extend_from_slice(&chunk);
    }

    // serde_json::from_slice : is used to convert a bytes to corresponding JSON text
    // refer to the documentation for example corresponding to this method
    // @see https://docs.rs/serde_json/1.0.138/serde_json/fn.from_slice.html
    let obj = serde_json::from_slice::<MyObj>(&body)?;      // extract struct from json

    // tested to check if payload accepts data successfully
    // for testing purposes
    // println!("{:?}", obj.clone());

    // satisfy return type
    // return the json data as part of the successful response
    Ok(HttpResponse::Ok().json(obj)) 
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
