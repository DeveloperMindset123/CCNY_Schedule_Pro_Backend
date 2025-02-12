// when it comes to database, there are 4 options available
// C : create
// R : read
// U : update
// D : delete

// this bin handles deletion of data within the datbase
use diesel::prelude::*;
use CCNY_Schedule_Pro_Backend::*;       // self-import crate
use std::env::args;

fn main() {
    use self::schema::posts::dsl::*;

    // retrieve the target to be deleted
    // we pass in the title of the post here
    // when the bin command is invoked within the terminal
    let target = args().nth(1).expect("expected a target to match against");

    // print statement for testing purposes
    // println!("target found : {:?}", target);
    let pattern = format!("%{}%", target);      // "%{}% : used for string interpolation"

    // println!("formatted pattern : {pattern:?}");     // testing purpose

    let connection = &mut establish_connection();

    // attempt to delete the post
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");
    
    println!("Deleted {} posts", num_deleted);
}