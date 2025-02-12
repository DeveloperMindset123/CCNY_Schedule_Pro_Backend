// sample to check if users can be successfully created or not.
// created by referencing write_post.rs
use CCNY_Schedule_Pro_Backend::*;       // so we can use the create_user function
use std::io::{stdin, Read};             // retrive information about user via terminal


// NOTE : this bin is primarily for experiemntal purpose
// to ensure new data can indeed be inserted within the database
// for performing CRUD operations
// this the C : create portion
//
// Normally, this would come from the api call via POST method
fn main() {
    // returns mutable reference
    // to PgConnection
    let connection = &mut establish_connection();   

    // create placeholder variables
    // to store the user inputs
    // initialized as empty strings
    let mut first_name = String::new();
    let mut last_name = String::new();
    let mut email = String::new();
    let mut major = String::new();
    let mut date_of_birth = String::new();
    let mut pronouns = String::new();
    let mut gender = String::new();
    let mut degree_type = String::new();
    let mut college_year = String::new();

    // ask user for their first name
    println!("What is the first name of this user?");
    stdin().read_line(&mut first_name).unwrap();

    // remove trailing newline
    // and store the trimmed value via shadowing
    let first_name = first_name.trim_end();       

    println!("What is the last name of this user?");
    stdin().read_line(&mut last_name).unwrap();

    let last_name = last_name.trim_end();

    println!("What is the email of this user?");
    stdin().read_line(&mut email).unwrap();

    let email = email.trim_end();

    println!("What is the major of this user?");
    stdin().read_line(&mut major).unwrap();

    let major = major.trim_end();

    println!("What is the date of birth of this user?");
    stdin().read_line(&mut date_of_birth).unwrap();

    let date_of_birth = date_of_birth.trim_end();

    println!("What is the pronouns of this user?");
    stdin().read_line(&mut pronouns).unwrap();

    let pronouns = pronouns.trim_end();

    println!("What is the gender of this user?");
    stdin().read_line(&mut gender).unwrap();

    let gender = gender.trim_end();

    println!("What is the degree type of this user?");
    stdin().read_line(&mut degree_type).unwrap();

    let degree_type = degree_type.trim_end();

    println!("What is the college year of this user? (Press {EOF} when finished)");
    stdin().read_line(&mut college_year).unwrap();
    let college_year = college_year.trim_end();

    let user = create_user(
        connection,
        first_name,
        last_name,
        email,
        major,
        date_of_birth,
        pronouns,
        gender,
        degree_type,
        college_year 
    );
    println!("\n Saved draft user with name {first_name:?}, {last_name:?} containing ID : {}", user.id);
}

#[cfg(not(windows))]        // if OS isn't windows
const EOF : &str = "CTRL+D";

#[cfg(windows)]
const EOF : &str = "CTRL+Z";