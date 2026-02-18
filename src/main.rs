use std::fs;
use std::path::{PathBuf};
use ap::parser::{Parser, Policy};

/// We get the `path`, `extension`, and `size` of each directory we'll produce.
/// The `path` indicates the directory of the rom files.
/// The `extension` indicates which files to process. For example, `nes`.
/// The `size` is how many files we want per directory.
///
/// After reading the `RomFile`s, we split the vector by `size`.
/// For each slice, we use the first and last chars to build the directory names.
/// We build each directory with the pattern:
/// part-{COUNTER}-{CHAR_START_FIRST_FILE}-to-{CHAR_START_LAST_FILE}
/// Where `COUNTER` is a two digit string 01, 02, 03 and so on.
/// `CHAR_START_FIRST_FILE` is the first char of the first file in the slice,
/// and CHAR_START_LAST_FILE is the first char of the last file in the slice.
///
/// If `CHAR_START_FIRST_FILE` or `CHAR_START_LAST_FILE` is not in
/// [A-Za-z-0-9] we replace it with `_`
/// TODO: Handle errors
fn main() -> std::io::Result<()> {
    let env_args = std::env::args().skip(1).collect();
    let options = Options::parse(&env_args);

    let mut files: Vec<RomFile> = read_rom_files_list(&options);
    files.sort_by(|a, b| a.filename.cmp(&b.filename));

    let rom_files_slices: Vec<Vec<RomFile>> = files
        .chunks(options.max_roms_per_directory)
        .map(|slice| slice.to_vec())
        .collect();

    let digits = (rom_files_slices.iter().count() as f64).log10().ceil() as usize;

    let rom_slices: Vec<RomSlice> = rom_files_slices
        .into_iter()
        .enumerate()
        .map(|(i, rom_files)| RomSlice::new(i, digits, rom_files))
        .collect();

    // TODO: prepare plan. show target directories and files that will go into each directory.
    // TODO: if the user presses `y` we'll move the files into the directories.
    rom_slices.iter().for_each(|slice| {
       println!("{} {} files", slice.directory_name, slice.rom_files.iter().count());
    });

    Ok(())
}

struct RomSlice {
    rom_files: Vec<RomFile>,
    char_ini: char,
    char_end: char,
    idx: usize,
    directory_name: String,
}

impl RomSlice {
    fn new(idx: usize, digits: usize, rom_files: Vec<RomFile>) -> Self {
        let char_ini = rom_files.first().unwrap().filename.chars().next().unwrap();
        let char_ini = if char_ini.is_ascii_alphanumeric() { char_ini } else { '_' };
        let char_end = rom_files.last().unwrap().filename.chars().next().unwrap();
        let char_end = if char_end.is_ascii_alphanumeric() { char_end } else { '_' };
        let directory_name = format!("part-{idx:0width$}-{char_ini}-to-{char_end}", width = digits);
        RomSlice { rom_files, char_ini, char_end, idx, directory_name }
    }
}

#[derive(Clone)]
struct RomFile {
    path: PathBuf,
    filename: String,
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
            let rom_file = RomFile { path, filename };
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