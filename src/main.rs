mod rust;
mod os;

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

            dependencies = dependencies.clone().replace("\r\n", "");

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

            println!("Name: {}; Version: {}; Dependencies: {:?}", crate_name.replace("\r\n", ""), crate_version.replace("\r\n", ""), crate_dependencies_formatted);

            match rust::init_cargo_files(&*crate_name.replace("\r\n", ""), &*crate_version.replace("\r\n", ""), crate_dependencies_formatted) {
                Ok(_) => (),
                Err(err) => eprintln!("Error: {}", err),
            }
        },
        _ => println!("You need to enter a supported type of project."),
    }
}