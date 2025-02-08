// example file to test and ensure database connection sample works

use self::models::*;
use diesel::prelude::*;
// use diesel_demo::*;      // originally (name of project)
use CCNY_Schedule_Pro_Backend::*;

// initially, will show 0 posts
fn main() {
    // imports a bunch of aliases so we can say posts
    // instead of posts::table and published instead of posts::published
    // 
    // it's always important to keep imports to
    // schema::table_name_placeholder::dsl::* 
    // inside the current function to prevent polluting the module namespace
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)       // note, load does not take in a reference (connection is moved and cosumed here)
        .expect("error loading posts");     // error handling message in case .unwrap() fails
        // if expect message is printed, program terminates by this point

    // specifies number of posts to be displayed
    println!("Displaying {} posts", results.len());

    // iterate over resulting data
    for post in results {
        println!("{}", post.title);
        println!("----------------\n");     // minor formatting
        println!("{}", post.body);      // print out content within the particular post.
    }
}