// bin to write post to database via cli
// essentially treating this as a "seperate" crate 
use CCNY_Schedule_Pro_Backend::*;
use std::io::{stdin, Read};     // retrieve and save user input from terminal

// define main() function
fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    // ask the user what the title of the post should be
    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();           // remove the traliing newline

    // EOF is implemented underneath
    // it's terminal commands represented in string format
    println!("\n OK! Let's write {title} (Press {EOF} when finished)\n",);
    
}