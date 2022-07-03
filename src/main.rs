mod rust;
mod os;
mod projects;

use std::io::{self, Write};

use rust::Dependency;

fn flush() {
    match io::stdout().flush() {
        Ok(_) => print!(""),
        Err(e) => eprintln!("Error: {}", e)
    }
}

#[allow(unused_imports, unused_assignments)]
fn main() {
    print!("Please enter the type of project you want to create [rust]: ");

    match io::stdout().flush() {
        Ok(_) => print!(""),
        Err(e) => eprintln!("Error: {}", e)
    }

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Error reading line");

    user_input = user_input.replace("\n", "").as_str().parse().expect("Error parsing from String to &str");

    match user_input.to_lowercase().replace("\r", "").as_str() {
        "rust" => {
            projects::init_rust();
        },
        _ => println!("You need to enter a supported type of project."),
    }
}