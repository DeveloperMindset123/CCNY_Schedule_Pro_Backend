use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// define routes for basic api handles
// Responder is a trait
// trait implemented by types that can be converted to an HTTP response.
// remove ; at the end since otherwise the return type will be validated as () instead of Responder
//
// for testing, refer to .rest and the test.rs file
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

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("This is a manual hello!")
}

#[post("/{conversation_data}")]
pub async fn send_conversation_to_database() -> impl Responder {
    HttpResponse::Ok().body("Not yet implemented.")
}

