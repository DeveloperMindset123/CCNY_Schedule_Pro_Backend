// bin to check if current user information can be retrieved or not.
use self::models::User;     // import modified from Post -> User
use diesel::prelude::*;
use CCNY_Schedule_Pro_Backend::*;
use std::env::args;

fn main() {
    use self::schema::users::dsl::users;

    // retrieve user id
    // from command line argument
    // only expects values such as 1,2,3,4, etc.
    // will take the first argument passed on the command line
    // thus .nth(1) has been set
    let user_id = args()
        .nth(1)
        .expect("read_users requires an user id.")
        .parse::<i32>()
        .expect("Invalid user id");

    let connection = &mut establish_connection();

    // search based on user_id
    // retrieve the first user that pops up
    let user = users
        .find(user_id)
        .select(User::as_select())
        .first(connection)
        .optional();

    // use pattern matching to check if the user's information is valid
    match user {
        Ok(Some(user)) => println!("{:?}", user),     // printout user info if it appears valid directly

        // this will execute if the provided user ID doesn't exist
        Ok(None) => println!("Unable to find user based on provided id of : {}", user_id),

        // this will execute if there's some overall error within the database
        // when it comes to fetching information
        Err(_) => println!("An error occured while fetching user information.")
    }
}