extern crate regex;

extern crate glob;

use glob::glob;
use std::env;

use std::process::Command;
use std::time::Instant;

fn md_files() -> Vec<String> {
    let pattern = "**/*.md";
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let md_files: Vec<String> = glob(&format!("{}/{}", current_dir.display(), pattern))
        .expect("Failed to read glob pattern")
        .filter_map(|entry| {
            if let Ok(path) = entry {
                Some(path.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();
    return md_files;
}

fn run_git_cmd(command: & str) -> bool {
    let output = Command::new("sh")  // invoke a shell
        .arg("-c")  // execute command as interpreted by program
        .arg(command)  // run the command
        .status()  // check for status
        .expect("Failed to execute command");
    if output.success() {
        return true;
    } else {
        println!("ERROR: Command failed with an error code: {:?}", output.code());
        return false;
    }
}

pub fn main() {
    let start = Instant::now();
    let arguments: Vec<String> = env::args().collect();
    let owner = &arguments[1];
    let repo = &arguments[2];
    let command = format!("git clone https://github.com/{}/{}.wiki.git", owner, repo);
    run_git_cmd(command.as_str());
    for md_file in md_files() {
        println!("{}", md_file)
    }
    let elapsed = start.elapsed();
    println!("{}", elapsed.as_secs())
}
