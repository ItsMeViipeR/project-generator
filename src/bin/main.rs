use project_generator::*;

fn main() {
    let user_menu_result: u32 = user_menu::init_user_menu();

    match user_menu_result {
        1 => projects::rust::init_rust(),
        2 => std::process::exit(0),
        _ => println!("Invalid input!"),
    }
}