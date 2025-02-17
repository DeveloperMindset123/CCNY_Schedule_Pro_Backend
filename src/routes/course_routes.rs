// imported from rmp_routes
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
use ccny_course_catalog::CCNYCourseCatalog;

// returns list of courses within the current department specified
#[get("/ccny/courses/{department}")]
pub async fn retrieve_course_list(query_params : web::Path<(String)>) -> Result<String> {
    Ok(String::from("Not yet implemented."))
}

// retrieve list of departments
// available to select from
#[get("/ccny/departments")]
pub async fn retrieve_department_list() -> Result<String> {
    Ok("Not Yet Implemented".to_string())
}

// retrieves information about a specific course
// based on the provided department and course name
// the datatypes for both query params will be String
#[get("/ccny/courses/{department}/{course_name}")]
pub async fn retrieve_course_info(query_params : web::Path<(String, String)>) -> Result<String> {
    let (department_name, course_name) = query_params.into_inner();
    Ok(format!("department name recieved : {department_name:?}\n course name recieved : {course_name:?}"))
}



// example code for reference
// async fn main() -> Result<()> {
//     let mut course_instance = CCNYCourseCatalog::new(String::from("computer science"), Some(String::from("data structures")));
//     // return type of list_of_courses
//     // Result<Vec<CourseInfo>, anyhow::Error>
//     let mut list_of_courses = course_instance.get_courses_list().await;
//     println!("{list_of_courses:#?}");
//     Ok(())
// }

// // additional example code for reference
// async fn main() -> Result<()> {
//     let mut course_instance = CCNYCourseCatalog::new(String::from("computer science"), Some(String::from("data structures")));
//     // return tyep of list_of_courses
//     // Result<Vec<CourseInfo>, anyhow::Error>
//     let mut list_of_courses = course_instance.get_courses_list().await;
//     let mut course_info = course_instance.get_course_info().await;
//     println!("{course_info:#?}");
//     Ok(())
// }

