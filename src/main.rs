extern crate env_logger;
extern crate glob;
extern crate log;
extern crate regex;
extern crate reqwest;

use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use log::{error, info};

mod lookup;
mod connection;
mod git;
mod files;
mod squire;

fn runner(filename: &str,
          exclusions: Vec<String>,
          counter: Arc<Mutex<HashMap<String, Arc<Mutex<i32>>>>>) -> i32 {
    let mut urls = 0;
    let text = match files::read(filename) {
        Ok(content) => content,
        Err(error) => {
            error!("{}", error);
            return urls;
        }
    };
    let text = text.to_string();
    let mut threads = Vec::new();
    for hyperlink in lookup::find_md_links(text.as_str()) {
        urls += 1;
        let (name, url) = hyperlink;
        let exclusions_cloned = exclusions.clone();
        let counter_cloned = counter.clone();
        let handle = thread::spawn(move || {
            let success_flag = connection::verify_url((name, url), exclusions_cloned);
            if success_flag {
                let mut success_count = counter_cloned.lock().unwrap();
                let success_counter = success_count.entry("success".to_string()).or_insert(Arc::new(Mutex::new(0)));
                *success_counter.lock().unwrap() += 1;
            } else {
                let mut failed_count = counter_cloned.lock().unwrap();
                let failed_counter = failed_count.entry("failed".to_string()).or_insert(Arc::new(Mutex::new(0)));
                *failed_counter.lock().unwrap() += 1;
                if env::var("exit_code").unwrap_or("0".to_string()) != "1" {
                    env::set_var("exit_code", "1");
                }
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        if handle.join().is_err() {
            error!("Error awaiting thread")
        }
    }
    urls
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
            error!("Cloning was successful but wiki path wasn't found");
            env::set_var("exit_code", "1");
        }
    }
    let counter: Arc<Mutex<HashMap<String, Arc<Mutex<i32>>>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut threads = Vec::new();
    for md_file in files::get_markdown() {
        let exclusions_cloned = exclusions.clone();
        let counter_cloned = counter.clone();
        let handle = thread::spawn(move || {
            let count = runner(&md_file, exclusions_cloned, counter_cloned);
            info!("Scanned '{}' with {} URLs", md_file.split("/").last().unwrap(), count);
        });
        threads.push(handle);
    }
    for handle in threads {
        if handle.join().is_err() {
            error!("Error awaiting thread")
        }
    }
    squire::unwrap(counter);
    let elapsed = start.elapsed();
    info!("'none-shall-pass' protocol completed. Elapsed time: {:?}s", elapsed.as_secs());
    exit(squire::get_exit_code());
}
