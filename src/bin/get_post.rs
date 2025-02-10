use self::models::Post;
use diesel::prelude::*;
use CCNY_Schedule_Pro_Backend::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::posts;

    let post_id = args()
}