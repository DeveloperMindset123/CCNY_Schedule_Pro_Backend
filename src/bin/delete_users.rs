// removes existing users within the table if they exist
// imports stays the same as delete_post.rs bin
use diesel::prelude::*;
use CCNY_Schedule_Pro_Backend::*;       // self-import crate
use std::env::args;


// NOTE : since we need to delete users by email
// we will need to perform some pattern matching to check if the user input is a valid email
fn main() {
    use self::schema::users::dsl::*;        // changed from self::schema::posts::dsl::* --> self::schema::users::dsl::*

    // retrieve the user based on the argument provided
    // filter by email (since that will be unique per user)
    let target = args().nth(1).expect("expected an email to be matched against");

    // we are looking for similar matching names to this email
    // not ideal for actually deleting users within database
    // since email needs to be exact
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();       // connects to database based on DATABASE_URL within .env

    // filter and delete by email
    // email.like is coming from self::schema::users::dsl::email
    let num_users_deleted = diesel::delete(users.filter(email.like(pattern)))
        .execute(connection)
        .expect("Error deleting user based on email provided.");        // error handler in case incorrect email is provided

    println!("Deleted {} users", num_users_deleted);
}