use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

use glob::glob;

pub fn read(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn get_markdown() -> Vec<String> {
    let pattern = "**/*.md";
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let md_files: Vec<String> = glob(&format!("{}/{}", current_dir.display(), pattern))
        .expect("Failed to read glob pattern")
        .filter_map(|entry| {
            if let Ok(path) = entry {
                Some(path.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();
    md_files
}
