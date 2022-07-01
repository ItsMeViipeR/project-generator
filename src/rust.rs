use std::error::Error;
use std::fs;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Dependency {
    name: &'static str,
    version: &'static str
}

impl Dependency {
    pub fn new(name: &'static str, version: &'static str) -> Self {
        Self {
            name,
            version,
        }
    }
}

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

pub fn init_cargo_files(crate_name: &str, crate_version: &str, crate_dependencies: Vec<Dependency>) -> Result<(), Box<dyn Error>> {
    create_dir_all(crate_name).expect("Error while trying to create crate directory");
    create_dir_all(format!("{}/src", crate_name).as_str()).expect("Error while trying to create src directory.");

    // create Cargo.toml file in ./crate-name
    let file_path = PathBuf::from(crate_name).join("Cargo.toml");
    // Create the lock file
    let lock_file_path = PathBuf::from(crate_name).join("Cargo.lock");
    // Create src/main.rs file
    let main_rs_path = PathBuf::from(crate_name).join("src").join("main.rs");

    // Make the files
    let mut file = get_file(file_path.clone());
    let _ = get_file(lock_file_path);
    let mut main_file = get_file(main_rs_path);

    // Erase files' content
    file.set_len(0).expect("Failed to flush Cargo.toml file");
    main_file.set_len(0).expect("Failed to flush src/main.rs file");

    // write into the files
    /* match file.write(format!("[package]\nname = \"{}\"\nversion = \"{}\"", crate_name, crate_version).as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(Box::new(e))
    } */

    match main_file.write(format!("fn main() {{\n     println!(\"Project created with Project generator v0.1.0\");\n}}").as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(Box::new(e))
    }

    for dependency in crate_dependencies.clone() {
        // println!("{} = {}", dependency.name, dependency.version);

        let content = fs::read_to_string(file_path.clone()).unwrap_or(String::from(""));

        if content.is_empty() {
            match file.write(format!("[package]\nname = \"{}\"\nversion = \"{}\"\n\n[dependencies]\n{} = \"{}\"", crate_name, crate_version, dependency.name, dependency.version).as_bytes()) {
                Ok(_) => (),
                Err(e) => return Err(Box::new(e))
            }
        } else {
            file.set_len(0).expect("Cannot set file len to 0");

            match file.write(format!("{}\n{} = \"{}\"", content, dependency.name, dependency.version).as_bytes()) {
                Ok(_) => (),
                Err(e) => return Err(Box::new(e))
            }
        }
    }

    Ok(())
}