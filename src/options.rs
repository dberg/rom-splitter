use std::path::PathBuf;
use ap::parser::{Parser, Policy};
use crate::app_error::AppError;

/// The `path` is the directory where the rom files resides, and
/// where the new directories will be created.
pub struct Options {
    pub version: bool,
    pub path: PathBuf,
    pub extension: String,
    pub max_roms_per_directory: usize,
}

impl Options {
    pub fn parse(args: &Vec<String>) -> Result<Options, AppError> {
        let options = Parser::new()
            .arg("version", 'v', Policy::Flag)
            .arg("path", 'p', Policy::Default(String::from(".")))
            .arg("extension", 'e', Policy::Default(String::new()))
            .arg("max-roms-per-directory", 'm', Policy::Default(String::from("100")))
            .run(args)
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;

        let version = options.get("version").map_err(|e| AppError::InvalidArgument(e.to_string()))? == "true";

        if version {
            return Ok(Options {
                version,
                path: PathBuf::new(),
                extension: String::new(),
                max_roms_per_directory: 0,
            });
        }

        let extension = options.get("extension").map_err(|e| AppError::InvalidArgument(e.to_string()))?.clone();
        if extension.is_empty() {
            return Err(AppError::InvalidArgument(String::from("Missing argument --extension")));
        }

        let path = options.get("path").map_err(|e| AppError::InvalidArgument(e.to_string()))?;
        let path = PathBuf::from(path).canonicalize()?;

        if !path.is_dir() {
            return Err(AppError::InvalidArgument(
                format!("'{}' is not a directory", path.display())
            ));
        }

        let max_roms_per_directory = options
            .get("max-roms-per-directory")
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?
            .parse::<usize>()
            .map_err(|_| AppError::InvalidArgument(
                String::from("max-roms-per-directory must be a positive integer")
            ))?;

        Ok(Options { version, path, extension, max_roms_per_directory })
    }
}
