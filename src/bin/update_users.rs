// sample code to update information about an existing user
// this example code updates existing user's email information
use self::models::User;
use diesel::prelude::*;
use CCNY_Schedule_Pro_Backend::*;
use std::env::args;

fn main() {
    // test to check if email can be updated or not
    use self::schema::users::dsl::{users, email};

    // comes from command line argument
    let id = args()
        .nth(1)
        .expect("update_users requires a user id")
        .parse::<i32>()
        .expect("Invalid ID");

    println!("retrieved user id : {id:?}");

    // establish_connection() : comes from lib.rs
    let connection = &mut establish_connection();

    // retrieve and update user email
    // test to check
    // search for the user based on the id
    let user = diesel::update(users.find(id))       // search based on some provided info
        .set(email.eq("dasa50196@gmail.com"))       // update based on info that has been discovered
        .returning(User::as_returning())
        .get_result(connection)
        .unwrap();

    // print out email to see if it has updated.
    println!("updated email {:?}", user.email);
}