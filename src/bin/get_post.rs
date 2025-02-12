use self::models::Post;
use diesel::prelude::*;
use CCNY_Schedule_Pro_Backend::*;
use std::env::args;

// the .optional() method will return 
// the method return Option<Post> instead of throwing an error
// we can use the "match" method to determine what might happen as a result
fn main() {
    use self::schema::posts::dsl::posts;

    // post id not found
    let post_id = args()
        .nth(1)
        .expect("get_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    // hardcoded post_id here as well
    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional();          // this allows for returning an Option<Post>, otherwise, an error will be thrown 

    // use pattern matching to check if returned post is valid
    match post {
        Ok(Some(post)) => println!("Post with id : {} has a title : {}\n post body : {}", post.id, post.title, post.body),
        Ok(None) => println!("Unable to find post based on current id of 2"),
        Err(_) => println!("An error occured while fetching"),
    }
}