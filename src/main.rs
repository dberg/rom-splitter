use std::{fs, io};
use std::io::Write;
use std::path::PathBuf;
use ap::parser::{Parser, Policy};
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

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
fn main() -> Result<(), AppError> {
    let env_args: Vec<String> = std::env::args().skip(1).collect();

    if env_args.iter().any(|a| a == "--version" || a == "-v") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let options = Options::parse(&env_args)?;

    let mut files: Vec<RomFile> = read_rom_files_list(&options)?;
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

    rom_slices.iter().for_each(|slice| {
       println!("{} {} files", slice.directory_name, slice.rom_files.iter().count());
    });

    print!("\nProceed to create the directories above and then move the files into the new directories? [Y/n]: ");
    io::stdout().flush()?;

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    let answer = answer.trim().to_lowercase();
    if answer == "y" || answer == "yes" {
        create_directories_and_move_files(rom_slices, &options)?;
    } else {
        println!("Aborting!");
    }


    Ok(())
}

fn create_directories_and_move_files(rom_slices: Vec<RomSlice>, options: &Options) -> Result<(), AppError> {
    for slice in rom_slices {
        let path = options.path.join(slice.directory_name);
        // create directory
        if let Err(e) = fs::create_dir(path.clone()) {
            // Ignore directories that already exist
            if e.kind() != io::ErrorKind::AlreadyExists {
                return Err(AppError::Io(e));
            }
        }
        // move files
        for rom_file in slice.rom_files {
            let dest = path.join(rom_file.filename);
            fs::rename(rom_file.path, dest)?;
        }
    }

    Ok(())
}

struct RomSlice {
    rom_files: Vec<RomFile>,
    directory_name: String,
}

impl RomSlice {
    fn new(idx: usize, digits: usize, rom_files: Vec<RomFile>) -> Self {
        let char_ini = rom_files.first().unwrap().filename.chars().next().unwrap();
        let char_ini = if char_ini.is_ascii_alphanumeric() { char_ini } else { '_' };
        let char_end = rom_files.last().unwrap().filename.chars().next().unwrap();
        let char_end = if char_end.is_ascii_alphanumeric() { char_end } else { '_' };
        let directory_name = format!("part-{idx:0width$}-{char_ini}-to-{char_end}", width = digits);
        RomSlice { rom_files, directory_name }
    }
}

#[derive(Clone)]
struct RomFile {
    path: PathBuf,
    filename: String,
}

fn read_rom_files_list(options: &Options) -> Result<Vec<RomFile>, AppError> {
    let mut files = Vec::new();

    for entry in fs::read_dir(&options.path)? {
        let entry = entry?;
        let path = entry.path();

        let Some(extension) = path.extension().and_then(|e| e.to_str()) else {
            continue;
        };

        if path.is_file() && extension == options.extension {
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| AppError::InvalidArgument(
                    format!("invalid filename: {}", path.display())
                ))?
                .to_string();
            files.push(RomFile { path, filename });
        }
    }

    Ok(files)
}

/// The `path` is the directory where the rom files resides, and
/// where the new directories will be created.
struct Options {
    pub path: PathBuf,
    pub extension: String,
    pub max_roms_per_directory: usize,
}

impl Options {
    fn parse(args: &Vec<String>) -> Result<Options, AppError> {
        let options = Parser::new()
            .arg("path", 'p', Policy::Default(String::from(".")))
            .arg("extension", 'e', Policy::Required)
            .arg("max-roms-per-directory", 'm', Policy::Default(String::from("100")))
            .run(args)
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;

        let path = options.get("path").unwrap();
        let path = PathBuf::from(path).canonicalize()?;

        if !path.is_dir() {
            return Err(AppError::InvalidArgument(
                format!("'{}' is not a directory", path.display())
            ));
        }

        let extension = options.get("extension").unwrap().clone();

        let max_roms_per_directory = options
            .get("max-roms-per-directory")
            .unwrap()
            .parse::<usize>()
            .map_err(|_| AppError::InvalidArgument(
                String::from("max-roms-per-directory must be a positive integer")
            ))?;

        Ok(Options { path, extension, max_roms_per_directory })
    }
}
