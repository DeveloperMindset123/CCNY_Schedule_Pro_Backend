pub mod models;
pub mod schema;
pub mod utils;
// file to show the last five published posts
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

// NewPost : struct to insert new data into the database for the posts table
// Post : struct that serves as return type for create_post function
//
// NewUser : struct to insert new data into the database for the users table
// User : struct that serves as return type for create_user function
use self::models::{NewPost, Post, User, NewUser};      // import the models we have defined to contain data

// define the function that will create
// and insert new posts to database
// creates and enters data into database for the posts table
pub fn create_post(conn : &mut PgConnection, title : &str, body : &str) -> Post {
    use crate::schema::posts;       // import schema locally

    // create new post
    // the values will be based on function parameter 
    let new_post = NewPost {
        title,
        body
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

// this function has been created for testing purpose
// using the abot create_post function as reference
pub fn create_user(
    conn : &mut PgConnection, 
    first_name : &str,
    last_name : &str,
    email : &str,
    user_password : &str,
    major : &str,
    date_of_birth : &str,
    pronouns : &str,
    gender : &str,
    degree_type : &str,
    college_year : &str
) -> User {
    use crate::schema::users;

    // since the names match the original fields of the struct
    // we don't need to explcitly mention them 
    let new_user = NewUser {
        first_name,
        last_name,
        email,
        user_password,
        major,
        date_of_birth,
        pronouns,
        gender,
        degree_type,
        college_year
    };

    // insert the above struct
    // into the database
    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())        // satisifes return statement
        .get_result(conn)
        .expect("Error saving new user")
}

// function used to connect to database
// note that prior to execution of this function
// remote database server should be running in the background
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    // DATABASE_URL is what has been set within .env file.
    // NOTE : diesel explicitly looks for a file named DATABASE_URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

