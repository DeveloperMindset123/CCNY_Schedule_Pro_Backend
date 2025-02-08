pub mod models;
pub mod schema;

// file to show the last five published posts
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewPost, Post};      // import the models we have defined to contain data

// define the function that will create
// and insert new posts to database
pub fn create_post(conn : &mut PgConnection, title : &str, body : &str) -> Post {
    use crate::schema::posts;       // import schema locally

    // create new post
    // the values will be based on function parameter 
    let new_post = NewPost {
        title,
        body
    }

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

