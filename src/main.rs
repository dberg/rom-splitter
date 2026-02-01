use std::fs;
use std::path::Path;
use ap::parser::{Parser, Policy};

/// We get the `path`, `extension`, and `size` of each directory we'll produce.
fn main() -> std::io::Result<()> {
    let env_args = std::env::args().skip(1).collect();
    // TODO: handle errors
    let options = Parser::new()
        .arg("path", 'p', Policy::Default(String::from(".")))
        .arg("extension", 'e', Policy::Required)
        .arg("max-roms-per-directory", 'm', Policy::Default(String::from("100")))
        .run(&env_args).unwrap();

    // TODO: handle errors
    let path = options.get("path").unwrap();
    let extension = options.get("extension").unwrap();
    let roms_per_directory = options.get("roms-per-directory").unwrap();
    println!("path: {}, extension: {}, roms_per_directory: {}", path, extension, roms_per_directory);

    // get path from command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);

    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory", path.display());
        std::process::exit(1)
    }

    println!("Files in {}:", path.display());

    for entry in fs::read_dir(path)?.flatten() {
        let path = entry.path();

        // Skip directories
        if path.is_file() {
            if let Some(name) = path.file_name() {
                println!("\t{}", name.to_string_lossy());
            }
        }
    }

    Ok(())
}
