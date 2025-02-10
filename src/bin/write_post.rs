// bin to write post to database via cli
// essentially treating this as a "seperate" crate 
use CCNY_Schedule_Pro_Backend::*;
use std::io::{stdin, Read};     // retrieve and save user input from terminal

// define main() function
fn main() {
    let connection = &mut establish_connection();       // return PgConnection

    let mut title = String::new();
    let mut body = String::new();

    // ask the user what the title of the post should be
    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();           // remove the traliing newline

    // EOF is implemented underneath
    // it's terminal commands represented in string format
    // the exit command is dependent on the OS being used
    // will output "(Press CTRL+D when finished)"
    println!("\n OK! Let's write {title} (Press {EOF} when finished)\n",);
    stdin().read_to_string(&mut body).unwrap();

    // create_post is a function defined within lib.rs
    // would be the same as writting CCNY_Schedule_Pro_Backend::create_post(connection, title, &body);
    let post = create_post(connection, title, &body);
    println!("\nSaved draft {title} with id {}", post.id);
}

// define EOF command
// EOF : end of file?
#[cfg(not(windows))]        // if OS isn't windows
const EOF : &str = "CTRL+D";

#[cfg(windows)]
const EOF : &str = "CTRL+Z";