mod rust;

use std::io::{self, Write};

use rust::Dependency;

fn create_rust_project() {
    match rust::init_cargo_files("Test", "0.1.0", vec![Dependency::new("multifactorials", "0.3.0"), Dependency::new("tauri", "1.0.0")]) {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {}", err)
    }
}

fn main() {
    print!("Please enter the type of project you want to create [rust]: ");

    match io::stdout().flush() {
        Ok(_) => print!(""),
        Err(e) => eprintln!("Error: {}", e)
    }

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Error reading line");

    user_input = user_input.replace("\n", "").as_str().parse().expect("Error parsing from String to &str");

    match user_input.to_lowercase().as_str() {
        "rust" => {
            println!("Creating Rust project...");
            create_rust_project();
        },
        _ => println!("You need to enter a supported type of project."),
    }
}
