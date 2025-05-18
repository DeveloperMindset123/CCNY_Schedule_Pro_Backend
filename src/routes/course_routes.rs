// imported from rmp_routes
use actix_web::{get, web, HttpResponse, Error, Result};
use CCNY_Schedule_Pro_Backend::*;
use ccny_course_catalog::CCNYCourseCatalog;

#[get("/ccny/department_lists")]
pub async fn get_department() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(serde_json::json!(CCNYCourseCatalog::get_department_list())))
}

#[get("/ccny/courses/{department}/{course_name}")]
pub async fn retrieve_course_info(query_params : web::Path<(String, String)>) -> Result<HttpResponse, Error> {
    let (department_name, course_name) = query_params.into_inner();
    Ok(HttpResponse::Ok().json(utils::retrieve_course_info_helper(&department_name, &course_name).await))
}

#[get("/ccny/courses/{department}")]
pub async fn retrieve_course_list_based_on_department(query_params : web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(utils::retrieve_course_list_helper(&query_params.into_inner()).await))
}
