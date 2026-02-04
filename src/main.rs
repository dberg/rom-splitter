use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf};
use ap::parser::{Parser, Policy};

/// We get the `path`, `extension`, and `size` of each directory we'll produce.
/// TODO: Handle errors
fn main() -> std::io::Result<()> {
    let env_args = std::env::args().skip(1).collect();
    let options = Options::parse(&env_args);

    let files = read_rom_files_list(&options);

    // We need to calculate the slices which we'll use to make the directories. A slice has
    // collection of rom files, along with first and last char of the slice. It's possible that
    // a slice contains a single char. It's ok to go over the max number of files per slice.
    // That just means the slice is too small.
    //
    // Before creating the directories we probably want to normalize the first char of the
    // filenames to [a-z-A-Z] and any other char we can fold into a special char. That again,
    // can over the max limit.
    //
    // We can first group the rom files per `start_with` and this will give us a count
    // of how many rom files we have per char. It's ok if we go over the max limit for
    // a single char.
    //
    // TODO: I take going over the max number back. We probably wnat to turn the slice
    // TODO: into a directory like part-01-A-B, or part-05-S-S, then part-06-S-T
    // TODO: since, letters like S can have hundreds of titles
    let group_by_start_with_normalized: HashMap<char, Vec<RomFile>> = files
        .into_iter()
        .fold(HashMap::new(), |mut map, rom_file| {
            map.entry(rom_file.start_with).or_default().push(rom_file);
            map
        });

    for (start_with, files) in group_by_start_with_normalized {
        println!("{} - {}", start_with, files.len());
    }

    Ok(())
}

struct RomSlice {
    rom_files: Vec<RomFile>,
    start_with_normalized: char,
}

struct RomFile {
    path: PathBuf,
    filename: String,
    start_with: char,
}

fn read_rom_files_list(options: &Options) -> Vec<RomFile> {
    let mut files = Vec::new();

    for entry in fs::read_dir(&options.path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let extension = path.extension().unwrap().to_str().unwrap();
        let extension_match = extension == options.extension;

        if path.is_file() && extension_match {
            let filename = path.file_name().unwrap().to_str().unwrap().to_string();
            let start_with = filename.chars().next().expect("Filename is empty");
            let rom_file = RomFile { path, filename, start_with };
            files.push(rom_file);
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