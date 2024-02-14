extern crate env_logger;
extern crate glob;
extern crate regex;
extern crate reqwest;

use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

mod lookup;
mod connection;
mod git;
mod files;
mod squire;
mod parser;

fn runner(filename: &str,
          exclusions: Vec<String>,
          counter: Arc<Mutex<HashMap<String, Arc<Mutex<i32>>>>>) -> i32 {
    let mut urls = 0;
    let text = match files::read(filename) {
        Ok(content) => content,
        Err(error) => {
            log::error!("{}", error);
            return urls;
        }
    };
    let text = text.to_string();
    let mut threads = Vec::new();
    for hyperlink in lookup::find_md_links(text.as_str()) {
        urls += 1;
        let hyperlink_clone = hyperlink.clone();
        let exclusions_cloned = exclusions.clone();
        let counter_cloned = counter.clone();
        let handle = thread::spawn(move || {
            let success_flag = connection::verify_url(&hyperlink, exclusions_cloned);
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
        threads.push((hyperlink_clone, handle));
    }
    for (url_info, handle) in threads {
        if handle.join().is_err() {
            log::error!("Error awaiting thread: {:?}", url_info)
        }
    }
    urls
}

fn main() {
    let config = parser::arguments();
    let start = Instant::now();
    println!("Activating the 'none-shall-pass' protocol for hyperlink validation in markdown files");
    if config.debug {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    log::info!("debug flag: {}", config.debug);
    let mut exclusions = vec!["localhost".to_string(), "127.0.0.1".to_string(), "0.0.0.0".to_string()];
    if !config.exclude.is_empty() {
        log::info!("Exclusion list: {:?}", &config.exclude);
        for exclusion in config.exclude {
            exclusions.push(exclusion);
        }
    }
    let wiki_path = format!("{}.wiki", config.repo);
    log::debug!("Cloning {}", &wiki_path);
    let command = format!("git clone https://github.com/{}/{}.git", config.owner, wiki_path);
    if git::run(command.as_str()) {
        let path = Path::new(&wiki_path);
        if !path.exists() {
            log::error!("Cloning was successful but wiki path wasn't found");
            env::set_var("exit_code", "1");
        }
    }
    let counter: Arc<Mutex<HashMap<String, Arc<Mutex<i32>>>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut threads = Vec::new();
    for md_file in files::get_markdown() {
        let md_file_cloned = md_file.clone();  // clone due to use in closure
        let exclusions_cloned = exclusions.clone();
        let counter_cloned = counter.clone();
        let handle = thread::spawn(move || {
            let count = runner(&md_file_cloned, exclusions_cloned, counter_cloned);
            log::info!("Scanned '{}' with {} URLs", md_file_cloned.split('/').last().unwrap().to_string(), count);
        });
        threads.push((md_file, handle));
    }
    for (file, handle) in threads {
        if handle.join().is_err() {
            log::error!("Error awaiting thread: {}", file)
        }
    }
    squire::unwrap(counter);
    let elapsed = start.elapsed();
    log::info!("'none-shall-pass' protocol completed. Elapsed time: {:?}s", elapsed.as_secs());
    exit(squire::get_exit_code());
}
