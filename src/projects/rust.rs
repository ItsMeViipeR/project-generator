use std::io::Write;
use crate::zip::extract;

pub fn init_rust() {
    println!("Creating Rust project...");

    print!("What's the name of your project? ");
    let mut project_name = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut project_name).unwrap();
    project_name = project_name.trim().to_string();

    extract::download_and_extract_rust();
    
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    /* rename the rust dir */
    std::fs::rename("rust", project_name).unwrap();

    println!("Rust project created!");
}