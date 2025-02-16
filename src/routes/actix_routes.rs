// Implements all the relevant endpoints here.
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error};
use futures_util::StreamExt;        // originally futures::StreamExt;
use serde::{Serialize, Deserialize};
use CCNY_Schedule_Pro_Backend::models::{NewUser, User};
use CCNY_Schedule_Pro_Backend::utils::type_of;
use CCNY_Schedule_Pro_Backend::*;
use diesel::prelude::*;
use diesel::dsl::exists;
use diesel::select;

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

#[post("/signup")]
pub async fn signup_handler(mut payload : web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();        // init variable to store body data
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        // error handler
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(actix_web::error::ErrorBadRequest("overflow"));
        }

        body.extend_from_slice(&chunk);
    }

    // convert from json to struct
    // NOTE : make sure to try unwrapping using ?
    let user_info = serde_json::from_slice::<NewUser>(&body)?;

    println!("Type info of user_info:\n{:?}", type_of(&user_info.first_name));

    // recreate database connection
    let connection = &mut establish_connection();


    use self::schema::users::dsl::*;

    // test to check if user information can be retrieved successfully
    // println!("Does user exists? {:#?}", users.filter(email.like(user_info.email)));


    // let mut query = users.into_boxed();
    // // println!("Distinct names : {:?}", distinct_names)

    // if let Some(retrieved_email) = user_info.email {
    //     query = query.filter(email.eq(retrieved_email));
    // }

    // let results = query.load(&database_connection);
    
    
    let email_exists : QueryResult<bool> = select(exists(users.filter(email.eq("dasa60196@gmail.com")))).get_result(connection);

    println!("{:?}", email_exists);


    // println!("{:?}",users.filter(email.eq(user_info.email)).load(&mut <&mut PgConnection as TryInto<T>>::try_into(database_connection).unwrap()).expect("No Such User"));

    // TODO : check if user by current email already exists
    // pass in all corresponding data
    let new_user = create_user(
        connection,
        user_info.first_name,
        user_info.last_name,
        user_info.email,
        user_info.major,
        user_info.date_of_birth,
        user_info.pronouns,
        user_info.gender,
        user_info.degree_type,
        user_info.college_year
    );
    // handle return type
    Ok(HttpResponse::Ok().json(user_info))
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
