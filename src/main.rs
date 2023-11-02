extern crate glob;
extern crate regex;

use std::env;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::thread;
use std::time::Instant;

mod lookup;
mod connection;
mod git;
mod files;
mod squire;

fn runner(filename: &str) -> bool {
    let mut fail = false;
    let text = match files::read(filename) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("{}", error);
            return false;  // return instead of setting flag
        }
    };
    let text = text.to_string();
    let mut threads = Vec::new();
    let exclusions = squire::get_exclusions();
    for hyperlink in lookup::find_md_links(text.as_str()) {
        let (name, url) = hyperlink;
        let name = name.as_str().to_string();
        let url = url.as_str().to_string();
        // Requires explicit variable assignment to avoid 'use occurs due to use in closure'
        // Clone exclusions and pass the clone into the closure
        let exclusions_cloned = exclusions.clone();
        let handle = thread::spawn(move || {
            connection::verify_url((name, url), exclusions_cloned)
        });
        threads.push(handle);
    }
    for handle in threads {
        if handle.join().is_err() {
            fail = true;
        }
    }
    fail
}

fn main() {
    println!("Activating the 'none-shall-pass' protocol for hyperlink validation in markdown files");
    let start = Instant::now();
    let arguments: Vec<String> = env::args().collect();
    let owner = &arguments[1];
    let repo = &arguments[2];
    let fail = &arguments[3];
    let debug = &arguments[4];
    env::set_var("debug", debug);
    println!("Fail flag is set to {}", fail);
    println!("Debug flag is set to {}", debug);
    let wiki_path = format!("{}.wiki", repo);
    let command = format!("git clone https://github.com/{}/{}.git", owner, wiki_path);
    if git::run(command.as_str()) {
        let path = Path::new(wiki_path.as_str());
        if !path.exists() {
            println!("Setting exit code to 1");
            env::set_var("exit_code", "1");
        }
    }
    for md_file in files::get_markdown() {
        println!("Scanning '{}'", md_file);
        runner(&md_file);
    }
    let code = squire::get_exit_code();
    println!("Exit code: {}", code);
    let elapsed = start.elapsed();
    println!("'none-shall-pass' protocol completed. Elapsed time: {:?}s", elapsed.as_secs());
    if code == 1 && fail == "true" {
        println!("Setting exit code to 1");
        exit(code);
    } else if code == 1 {
        println!("Setting exit code to 0, although there were errors");
    }
    exit(0)
}
