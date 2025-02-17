// Implements all the relevant endpoints here.
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error};
use futures_util::StreamExt;        // originally futures::StreamExt;
use serde::{Serialize, Deserialize};
use CCNY_Schedule_Pro_Backend::models::{NewUser, User, UserLogin};
use CCNY_Schedule_Pro_Backend::utils::{type_of, calculate_hash};
use CCNY_Schedule_Pro_Backend::*;
use diesel::prelude::*;
use diesel::dsl::exists;
use diesel::select;
use diesel::sql_query;

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

// relevant path
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

    // recreate database connection
    let connection = &mut establish_connection();
    use self::schema::users::dsl::*;
    // NOTE : don't prematurely unwrap the boolean value here
    let email_exists : QueryResult<bool> = select(exists(users.filter(email.eq(user_info.email)))).get_result(connection);

    // returns either Ok(true) or Ok(false)
    // depending on the existence of the value
    // println!("{:?}", email_exists.clone().unwrap());

    // return an error message to indicate that email is already in use
    if email_exists.unwrap() == true {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({
            "message" : "user already exists"
            })
        ));
    }


    // println!("{:?}",users.filter(email.eq(user_info.email)).load(&mut <&mut PgConnection as TryInto<T>>::try_into(database_connection).unwrap()).expect("No Such User"));

    // TODO : check if user by current email already exists
    // pass in all corresponding data
    let new_user = create_user(
        connection,
        user_info.first_name,
        user_info.last_name,
        user_info.email,
        &calculate_hash(&user_info.user_password),      // hash and insert to database
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

#[post("/signin")]
pub async fn signin_handler(mut payload : web::Payload) -> Result<HttpResponse, Error> {
    // inner namespce search
    use self::schema::users::dsl::*;

    let mut body = web::BytesMut::new();        // init variable to store body data
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        // error handler
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(actix_web::error::ErrorBadRequest("overflow"));
        }

        body.extend_from_slice(&chunk);
    }

    let login_creds = serde_json::from_slice::<UserLogin>(&body)?;

    let connection = &mut establish_connection();

    // search and check to see if user exists
    // NOTE : don't prematurely unwrap the boolean value here
    // we first want to check if the provided email exists
    let email_exists : QueryResult<bool> = select(exists(users.filter(email.eq(login_creds.clone().email)))).get_result(connection);

    if email_exists.unwrap() == false {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({
            "error" : "email doesn't exist, please register first"
            })));
    }

    // let val = users
    // .filter(email.eq(login_creds.clone().email))
    // .or_filter(user_password.eq(calculate_hash(&login_creds.clone().password))).select(user_password).get_result::<String>(connection);

    // Retrieve the password corresponding to the email
    // match and check if the hashed email matches the email from user input
    // if so, successful login, otherwise, unsuccessful login implemenetation

    // this is the same as writting the following SQL statement below:
    // SELECT user_password FROM users WHERE email={provided_input_email} 
    // retrieves password based on provided email address
    let user_password_verification = users.filter(email.eq(login_creds.clone().email)).select(user_password).get_result::<String>(connection);

    // check if the hashed password matches
    if calculate_hash(&login_creds.clone().password) != user_password_verification.unwrap() {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({
            "error" : "Incorrect password, please enter the correct password"
            })));
    }

    

    // let example_query=sql_query("select first_name,last_name, email,user_password from users where email=? and user_password=?");
    // let query_result : QueryResult<String> = example_query
    //     .bind::<diesel::sql_types::Text,_>(login_creds.clone().email)
    //     .bind::<diesel::sql_types::Text,_>(calculate_hash(&login_creds.clone().password))
    //     .get_result(connection);

    // let user_creds_check : QueryResult<bool> = select(exists(
    //     users.filter()
    // ))

    // otherwise, if email exists, we then need to compare and check if password matches
    

    // if email_exists.unwrap() == false {
    //     return Ok(HttpResponse::BadRequest().json(
    //         serde_json::json!({
    //         "message" : "user by this email doesn't exist, please register before continuing."
    //         })
    //     ));
    // }

    // TODO : perform a nested filter to check if current email and password combination exists
    Ok(HttpResponse::Ok().json(login_creds))
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
