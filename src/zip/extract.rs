use reqwest::blocking::Client;
use std::fs::File;
use zip::read::ZipArchive;
use std::fs;
use std::io;

pub fn download_and_extract_rust() {
    // URL du fichier ZIP à télécharger
    let url = "https://cdn.viiper.fr/rust.zip";
    
    // Nom du fichier ZIP après téléchargement
    let file_name = "rust.zip";

    println!("Downloading Rust project template...");
    
    // Télécharger le fichier ZIP
    let client = Client::new();
    let mut response = client.get(url).send().unwrap();
    let mut file = File::create(file_name).unwrap();
    response.copy_to(&mut file).unwrap();

    // Ouvrir le fichier ZIP
    let file = File::open(file_name).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    println!("Extracting Rust project template...");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    // delete rust.zip
    fs::remove_file("rust.zip").unwrap();

    println!("Rust project template extracted!");
}

pub fn download_and_extract_cpp() {
    // URL du fichier ZIP à télécharger
    let url = "https://cdn.viiper.fr/cpp.zip";
    
    // Nom du fichier ZIP après téléchargement
    let file_name = "cpp.zip";

    println!("Downloading C++ project template...");
    
    // Télécharger le fichier ZIP
    let client = Client::new();
    let mut response = client.get(url).send().unwrap();
    let mut file = File::create(file_name).unwrap();
    response.copy_to(&mut file).unwrap();

    // Ouvrir le fichier ZIP
    let file = File::open(file_name).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    println!("Extracting C++ project template...");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    // delete rust.zip
    fs::remove_file("cpp.zip").unwrap();

    println!("C++ project template extracted!");
}