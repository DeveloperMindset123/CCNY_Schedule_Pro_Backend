use std::any::type_name;
use std::hash::{DefaultHasher, Hash, Hasher};
use rateMyProfessorApi_rs::methods::RateMyProfessor;
use ccny_course_catalog::CCNYCourseCatalog;

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

// calculate the hash of a string
pub fn calculate_hash<T: Hash>(t: &T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish().to_string()      // convert u64 output to string output
}

pub async fn retrieve_course_list_helper(user_department_name : &str) -> serde_json::Value {

    // left course_name empty (not needed in this case)
    let mut college_instance = CCNYCourseCatalog::new(String::from(user_department_name), Some(String::from("")));

    // retrieve list of courses based on department name
    let mut courses_list_res = college_instance.get_courses_list().await;
    serde_json::json!(courses_list_res.unwrap())
}

// helper function for rmp routes api wrappers
// this function will fetch and return the professor list of CCNY
// in JSON format
pub async fn get_professor_list() -> serde_json::Value  {
    let mut rate_my_professor_instance = RateMyProfessor::construct_college("City College of New York");
    let mut list_of_professors = rate_my_professor_instance.get_professor_list().await;
    // check return type of data
    // println!("{:?}", type_of(&list_of_professors.unwrap()));
    serde_json::json!(list_of_professors.unwrap())
    // println!("{list_of_professors:#?}");
}


pub async fn retrieve_professor_summary(professor_name : &str) -> serde_json::Value {
    let mut rmp_instance = RateMyProfessor::construct_college_and_professor("City College of New York", professor_name);
    let teacher_summary_res = rmp_instance.get_teacher_summary(true).await;
    
    // return the response as json data
    serde_json::json!(teacher_summary_res.unwrap())
}

pub async fn retrieve_professor_comments(professor_name : &str) -> serde_json::Value {
    let mut rmp_instance = RateMyProfessor::construct_college_and_professor("City College of New York", professor_name);

    serde_json::json!(rmp_instance.get_professor_comments().await.unwrap())
    // Ok(String::from("Success"))
    // String::from("Success")
}


pub async fn retrieve_department_list_helper() -> serde_json::Value {
    serde_json::json!(CCNYCourseCatalog::get_department_list())
}

pub async fn retrieve_course_info_helper(department_input : &str, course_input : &str) -> serde_json::Value {
    let mut course_instance = CCNYCourseCatalog::new(String::from(department_input),Some(String::from(course_input)));
    serde_json::json!(course_instance.get_course_info().await.unwrap())

}