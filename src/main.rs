extern crate env_logger;
extern crate glob;
extern crate log;
extern crate regex;
extern crate reqwest;

use std::env;
use std::path::Path;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use log::{error, info, warn};

mod lookup;
mod connection;
mod git;
mod files;
mod squire;

fn runner(filename: &str, exclusions: Vec<String>, counter: Arc<Mutex<i32>>) {
    let text = match files::read(filename) {
        Ok(content) => content,
        Err(error) => {
            error!("{}", error);
            return;
        }
    };
    let text = text.to_string();
    let mut threads = Vec::new();
    for hyperlink in lookup::find_md_links(text.as_str()) {
        let (name, url) = hyperlink;
        let name = name.as_str().to_string();
        let url = url.as_str().to_string();
        // Requires explicit variable assignment to avoid 'use occurs due to use in closure'
        // Clone exclusions and pass the clone into the closure
        let exclusions_cloned = exclusions.clone();
        let counter_cloned = counter.clone();
        let handle = thread::spawn(move || {
            connection::verify_url((name, url), exclusions_cloned, counter_cloned)
        });
        threads.push(handle);
    }
    for handle in threads {
        if handle.join().is_err() {
            error!("Error awaiting thread")
        }
    }
}

fn main() {
    println!("Activating the 'none-shall-pass' protocol for hyperlink validation in markdown files");
    let start = Instant::now();
    let arguments: Vec<String> = env::args().collect();
    let owner = &arguments[1];
    let repo = &arguments[2];
    let fail = &arguments[3];
    let debug = &arguments[4];
    let exclude_hostnames = &arguments[5];
    if debug == "true" {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    info!("fail flag: {}\tdebug flag: {}", fail, debug);
    let mut exclusions = squire::get_exclusions();
    if !exclude_hostnames.is_empty() {
        info!("Exclusion list: {}", exclude_hostnames);
        for exclusion in exclude_hostnames.split(',') {
            exclusions.push(exclusion.trim().to_string());
        }
    }
    let wiki_path = format!("{}.wiki", repo);
    let command = format!("git clone https://github.com/{}/{}.git", owner, wiki_path);
    if git::run(command.as_str()) {
        let path = Path::new(wiki_path.as_str());
        if !path.exists() {
            error!("Setting exit code to 1");
            env::set_var("exit_code", "1");
        }
    }
    let mut count = 0;
    let counter = Arc::new(Mutex::new(0));  // Create a new counter for each thread
    for md_file in files::get_markdown() {
        info!("Scanning '{}'", md_file);
        runner(&md_file, exclusions.clone(), counter.clone());
        count += *counter.lock().unwrap();
    }
    let code = squire::get_exit_code();
    info!("Exit code: {}", code);
    let elapsed = start.elapsed();
    info!("'none-shall-pass' protocol completed. Elapsed time: {:?}s", elapsed.as_secs());
    info!("Total URLs validated: {}", count);
    if code == 1 && fail == "true" {
        error!("Setting exit code to 1");
        exit(code);
    } else if code == 1 {
        warn!("Setting exit code to 0, although there were errors");
    }
    exit(0)
}
