use std::any::type_name;
use std::hash::{DefaultHasher, Hash, Hasher};
use rateMyProfessorApi_rs::methods::RateMyProfessor;

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

// calculate the hash of a string
pub fn calculate_hash<T: Hash>(t: &T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish().to_string()      // convert u64 output to string output
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
 
// example usage
// fn main() {
//     let x = 21;
//     let y = 2.5;
//     println!("{}", type_of(&y));
//     println!("{}", type_of(x));
// }