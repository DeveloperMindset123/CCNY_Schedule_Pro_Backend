use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error, Result};
use futures_util::StreamExt;        // originally futures::StreamExt;
use serde::{Serialize, Deserialize};
use CCNY_Schedule_Pro_Backend::models::{NewUser, User, UserLogin};
use CCNY_Schedule_Pro_Backend::utils::{type_of, calculate_hash};
use CCNY_Schedule_Pro_Backend::*;
use diesel::prelude::*;
use diesel::dsl::exists;
use diesel::select;
use diesel::sql_query;

// #[get("/rmp/get_professor_list")]
// pub async fn professor_list_handler() {
//     todo!("not yet implemented");
// }

// retrieve summary of professor based on professor name
// name of professor will be passed in as query parameter
#[get("/rmp/summary/{professor_name}")]
pub async fn professor_summary_handler(query_params : web::Path<(String)>) -> Result<String> {
    // extract query param
    let professor = query_params.into_inner();
    Ok(format!("Retrieved user query : {}!", professor))
}

// retrieve comments about a professor
// based on query parameters provided as query params
// #[get("/rmp/comments/{professor_name}")]
// pub async fn professor_comments_handler() {
//     todo!("not yet implemented");
// }
