// we need to publish the blogs that we have written
use self::models::Post;
use diesel::prelude::*;
use CCNY_Schedule_Pro_Backend::*;
use std::env::args;

fn main() {

    // where is posts and published coming from?
    // published is a field from the SQL schema that has been defined
    use self::schema::posts::dsl::{posts, published};
    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    println!("post id : {id:?}");
    let connection = &mut establish_connection();

    // hardcoded post id since id is not being retrieved from database
    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(connection)
        .unwrap();

    println!("Published post {}", post.title);
}