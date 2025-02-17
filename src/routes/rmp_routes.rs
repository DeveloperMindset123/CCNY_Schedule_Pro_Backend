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
use rateMyProfessorApi_rs::methods::RateMyProfessor;

#[get("/rmp/get_professor_list")]
pub async fn professor_list_handler() -> Result<HttpResponse, Error> {
    let professor_data = utils::get_professor_list().await;

    // testing purpose to check if return type works
    // println!("{:#?}", professor_data);     // should be serde_json::Value type

    // Ok(format!("success"))
    Ok(HttpResponse::Ok().json(professor_data))
}

// retrieve summary of professor based on professor name
// name of professor will be passed in as query parameter
#[get("/rmp/summary/{professor_name}")]
pub async fn professor_summary_handler(query_params : web::Path<(String)>) -> Result<HttpResponse, Error> {
    // extract query param
    let professor = query_params.into_inner();
    let professor_summary = utils::retrieve_professor_summary(&professor).await;
    Ok(HttpResponse::Ok().json(professor_summary))
}

// retrieve comments about a professor
// based on query parameters provided as query params
#[get("/rmp/comments/{professor_name}")]
pub async fn professor_comments_handler(query_params : web::Path<(String)>) -> Result<HttpResponse, Error> {
    let name_professor = query_params.into_inner();
    Ok(HttpResponse::Ok().json(utils::retrieve_professor_comments(&name_professor).await))
}
