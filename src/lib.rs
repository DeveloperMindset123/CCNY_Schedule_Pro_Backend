pub mod models;
pub mod schema;
pub mod utils;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use self::models::{NewPost, NewUser, Post, User}; // import the models we have defined to contain data

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;
    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

// this function has been created for testing purpose
// using the create_post function as reference
pub fn create_user(
    conn: &mut PgConnection,
    first_name: &str,
    last_name: &str,
    email: &str,
    user_password: &str,
    major: &str,
    date_of_birth: &str,
    pronouns: &str,
    gender: &str,
    degree_type: &str,
    college_year: &str,
) -> User {
    use crate::schema::users;

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
        college_year,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning()) // satisifes return statement
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
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
