use std::io::Write;

pub fn init_user_menu() -> u32 {
    print!("Welcome to the Project Generator!\n 0. Exit\n 1. Rust\n 2. C++\n> ");
    std::io::stdout().flush().unwrap();
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    let user_input: u32 = user_input.trim().parse().unwrap();

    user_input
}