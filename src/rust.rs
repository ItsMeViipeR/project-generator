use std::error::Error;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn get_file(file_path: PathBuf) -> File {
    return if !file_path.exists() {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .append(true)
            .open(file_path)
            .expect("Couldn't open Cargo.toml file");

        file
    } else {
        let file = OpenOptions::new()
            .create_new(false)
            .write(true)
            .append(true)
            .open(file_path)
            .expect("Couldn't open Cargo.toml file");

        file
    }
}

pub fn init_cargo_files(crate_name: &str, crate_version: &str, _crate_dependencies: Vec<&str>) -> Result<(), Box<dyn Error>> {
    create_dir_all(crate_name).expect("Error while trying to create crate directory");
    create_dir_all(format!("{}/src", crate_name).as_str()).expect("Error while trying to create src directory.");

    // create Cargo.toml file in ./crate-name
    let file_path = PathBuf::from(crate_name).join("Cargo.toml");
    let lock_file_path = PathBuf::from(crate_name).join("Cargo.lock");
    let main_rs_path = PathBuf::from(crate_name).join("src").join("main.rs");

    let mut file = get_file(file_path);
    let _ = get_file(lock_file_path);
    let mut main_file = get_file(main_rs_path);

    file.set_len(0).expect("Failed to flush Cargo.toml file");
    main_file.set_len(0).expect("Failed to flush src/main.rs file");

    file.write(format!("[package]\nname = \"{}\"\nversion = \"{}\"", crate_name, crate_version).as_bytes()).expect("Error while trying to write into the Cargo.toml file.");
    main_file.write(format!("fn main() {{\n     println!(\"Project created with Project generator v0.1.0\");\n}}").as_bytes()).expect("Error while trying to write into the src/main.rs file.");

    Ok(())
}