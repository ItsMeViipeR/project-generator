use std::io;
use crate::{flush, os, rust::{self, Dependency}};

pub fn init_rust() {
    os::clear();

    let mut crate_name: String = String::new();
    let mut crate_version: String = String::new();
    let mut dependencies: String = String::new();
    let mut crate_dependencies: Vec<&str> = vec![];
    let mut crate_dependencies_formatted: Vec<Dependency> = vec![];

    print!("Please enter the crate name: ");

    flush();

    io::stdin().read_line(&mut crate_name).expect("Failed to get user input");

    os::clear();

    print!("Please enter the crate version like x.x.x: ");

    flush();

    io::stdin().read_line(&mut crate_version).expect("Failed to get user input");

    os::clear();

    print!("Specify all dependencies like this: name = version, second_name = version: ");

    flush();

    io::stdin().read_line(&mut dependencies).unwrap_or(1);

    os::clear();

    dependencies = dependencies.clone().replace("\n", "").replace("\r", "");

    let temp = dependencies.split(", ");

    crate_dependencies = temp.collect::<Vec<&str>>();

    for dep in crate_dependencies {
        let mut deps = vec![];

        if dep != "none" {
            deps = dep.split(" = ").collect::<Vec<&str>>();

            crate_dependencies_formatted.push(Dependency::new(deps[0].to_string(), deps[1].to_string()));

            println!("{:?}", crate_dependencies_formatted);
        }
    }

    // println!("Name: {}; Version: {}; Dependencies: {:?}", crate_name.replace("\n", "").replace("\r", ""), crate_version.replace("\n", "").replace("\r", ""), crate_dependencies_formatted);

    match rust::init_cargo_files(&*crate_name.replace("\n", "").replace("\r", ""), &*crate_version.replace("\n", "").replace("\r", ""), crate_dependencies_formatted) {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {}", err),
    }
}