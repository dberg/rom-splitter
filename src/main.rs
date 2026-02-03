use std::fs;
use std::path::{PathBuf};
use ap::parser::{Parser, Policy};

/// We get the `path`, `extension`, and `size` of each directory we'll produce.
/// TODO: Handle errors
fn main() -> std::io::Result<()> {
    let env_args = std::env::args().skip(1).collect();
    let options = Options::parse(&env_args);

    let files = read_rom_files_list(&options);
    for file in files {
        println!("{}", file.display())
    }

    Ok(())
}

fn read_rom_files_list(options: &Options) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in fs::read_dir(&options.path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let extension = path.extension().unwrap().to_str().unwrap();
        let extension_match = extension == options.extension;

        if path.is_file() && extension_match {
            files.push(path);
        }
    }

    files
}

struct Options {
    pub path: PathBuf,
    pub extension: String,
    pub max_roms_per_directory: usize,
}

impl Options {
    fn parse(args: &Vec<String>) -> Options {
        let options = Parser::new()
            .arg("path", 'p', Policy::Default(String::from(".")))
            .arg("extension", 'e', Policy::Required)
            .arg("max-roms-per-directory", 'm', Policy::Default(String::from("100")))
            .run(args).unwrap();

        let path = options.get("path").unwrap();
        let path = PathBuf::from(path);
        let path = path.canonicalize().unwrap();

        if !path.is_dir() {
            eprintln!("Error: '{}' is not a directory", path.display());
            std::process::exit(1)
        }

        let extension = options.get("extension").unwrap();
        let extension = extension.clone();

        let max_roms_per_directory = options.get("max-roms-per-directory").unwrap();
        let max_roms_per_directory = max_roms_per_directory.parse::<usize>().unwrap();

        Options {
            path,
            extension,
            max_roms_per_directory
        }
    }
}