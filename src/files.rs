use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use glob::glob;

pub fn read(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn get_markdown() -> Vec<String> {
    let pattern = "**/*.md";
    let current_dir: PathBuf = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("Failed to get current directory: {}", err);
            return Vec::new()
        },
    };
    let glob_pattern = format!("{}/{}", current_dir.display(), pattern);
    let glob_result = glob(&glob_pattern);
    let md_files: Vec<String> = match glob_result {
        Ok(paths) => paths.filter_map(|entry| {
            match entry {
                Ok(path) => Some(path.to_string_lossy().to_string()),
                Err(_) => None,
            }
        }).collect(),
        Err(err) => {
            log::error!("Failed to read glob pattern: {}", err);
            return Vec::new()
        },
    };
    md_files
}
