use std::error::Error;
use std::fs::OpenOptions;
use std::path::PathBuf;

pub fn init_cargo_files(crate_name: &str, _crate_version: &str, _crate_dependencies: Vec<&str>) -> Result<(), Box<dyn Error>> {
    // create Cargo.toml file in ./crate-name
    let file_path = PathBuf::from(crate_name).join("Cargo.toml");

    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(true)
        .expect("Couldn't open Cargo.toml file");

    match writeln!(file, "{}", line) {
        Ok(_) => {
            
        }
        Err(err) => eprintln!("Error: {}", err),
    }

    Ok(())
}