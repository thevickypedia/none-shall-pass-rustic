mod lookup;
mod connection;
mod git;

extern crate glob;
extern crate regex;

use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::net::ToSocketAddrs;
use std::path::Path;
use std::process::exit;
use std::thread;
use std::time::Instant;

use glob::glob;

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn get_exclusions() -> Vec<String> {
    let mut exclusions: Vec<String> = vec!["localhost".to_string(), "127.0.0.1".to_string()];
    let server_details = "localhost:80";
    let server: Vec<_> = server_details
        .to_socket_addrs()
        .expect("Unable to resolve domain")
        .collect();
    for slice in server.as_slice() {
        let server_ip = slice.ip().to_string().clone();
        if server_ip.starts_with("::") {
            continue;
        }
        exclusions.push(server_ip);
    }
    exclusions
}

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
    md_files
}

fn runner(filename: &str) -> bool {
    let mut fail = false;
    let text = match read_file(filename) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("{}", error);
            return false;  // return instead of setting flag
        }
    };
    let text = text.to_string();
    let mut threads = Vec::new();
    let exclusions = get_exclusions();
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

fn get_exit_code() -> i32 {
    let string = env::var("exit_code").unwrap_or("0".to_string());
    string.parse::<i32>().unwrap_or(0)
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
    for md_file in md_files() {
        println!("Scanning '{}'", md_file);
        runner(&md_file);
    }
    let code = get_exit_code();
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
