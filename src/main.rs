extern crate env_logger;
extern crate glob;
extern crate log;
extern crate regex;
extern crate reqwest;
extern crate serde;

use std::collections::HashMap;
use std::env;
use std::fs::{File, remove_dir_all};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use regex::Regex;
use reqwest::blocking::Client;

use crate::lookup::Hyperlink;

mod lookup;
mod connection;
mod git;
mod files;
mod squire;
mod parser;

#[derive(Clone, Default)]
struct JSONResponse {
    filename: String,
    filepath: String,
    hyperlink: Hyperlink,
}

struct ValidationResult {
    count: i32,
    errors: Vec<JSONResponse>,
}

fn verify_actions() -> Option<bool> {
    match env::var("GITHUB_ACTIONS") {
        Ok(val) => match val.parse() {
            Ok(parsed) => Some(parsed),
            Err(_) => {
                None
            }
        },
        Err(_) => None
    }
}

fn jsonify(data: Vec<JSONResponse>) -> Vec<HashMap<&'static str, String>> {
    let mut data_vec = Vec::new();
    for json_obj in data {
        let mut data_map = HashMap::new();
        data_map.insert("filename", json_obj.filename);
        data_map.insert("filepath", json_obj.filepath);
        data_map.insert("text", json_obj.hyperlink.text);
        data_map.insert("url", json_obj.hyperlink.url);
        data_vec.push(data_map)
    }
    data_vec
}

fn generate_summary(data: Vec<JSONResponse>) {
    let path = "gh_actions_summary.json";
    match File::create(path) {
        Ok(file) => {
            let json_data = jsonify(data);
            match serde_json::to_writer(&file, &json_data) {
                Ok(_) => log::info!("Dumped error information into JSON file"),
                Err(err) => {
                    log::error!("Failed to write JSON data to file: {}", err);
                }
            }
        }
        Err(err) => {
            log::error!("Failed to create summary file: {}", err);
        }
    }
}

fn runner(
    md_file: &str,
    cwd: String,
    exclusions: Vec<String>,
    counter: Arc<Mutex<HashMap<String, Arc<Mutex<i32>>>>>,
    client: Client,
) -> ValidationResult {
    let mut urls = 0;
    log::info!("Reading file: {}", md_file);
    let text = match files::read(md_file) {
        Ok(content) => content,
        Err(error) => {
            log::error!("{}", error);
            return ValidationResult {
                count: urls,
                errors: vec![JSONResponse::default()],
            };
        }
    };
    let mut threads = Vec::new();
    let responses = Arc::new(Mutex::new(Vec::new()));
    for hyperlink in lookup::find_md_links(&text) {
        urls += 1;
        let hyperlink_clone = hyperlink.clone();
        let exclusions_cloned = exclusions.clone();
        let client_cloned = client.clone();
        let counter_cloned = counter.clone();
        let responses_cloned = responses.clone();
        let filepath = if md_file.contains(".wiki") {
            // If the markdown file is in a wiki, removes preceding text, and the .md extension
            let pattern = Regex::new(r"[^.]+\.wiki").unwrap();
            pattern
                .replace(md_file, "wiki")
                .trim_end_matches(".md")
                .to_string()
        } else {
            // If the markdown file is in repo, removes the current working directory and path prefix
            md_file
                .trim_start_matches(&cwd)
                .trim_start_matches('/')
                .to_string()
        };
        let handle = thread::spawn(move || {
            let response = connection::verify_url(&hyperlink, exclusions_cloned, client_cloned);
            if response.ok {
                let mut success_count = counter_cloned.lock().unwrap();
                let success_counter = success_count.entry("success".to_string()).or_insert(Arc::new(Mutex::new(0)));
                *success_counter.lock().unwrap() += 1;
            } else {
                let mut failed_count = counter_cloned.lock().unwrap();
                let failed_counter = failed_count.entry("failed".to_string()).or_insert(Arc::new(Mutex::new(0)));
                *failed_counter.lock().unwrap() += 1;
                let mut locked_responses = responses_cloned.lock().unwrap();
                let filename = filepath.split('/').last().unwrap().to_string();
                let json_response = JSONResponse {
                    filename,
                    filepath,
                    hyperlink: response.hyperlink,
                };
                locked_responses.push(json_response);
            }
        });
        threads.push((hyperlink_clone, handle));
    }
    for (url_info, handle) in threads {
        if handle.join().is_err() {
            log::error!("Error awaiting thread: {:?}", url_info)
        }
    }
    let responses_cloned = responses.lock().unwrap().clone();
    ValidationResult {
        count: urls,
        errors: responses_cloned,
    }
}

fn request_builder() -> Client {
    let client = reqwest::blocking::ClientBuilder::new().user_agent("rustc");
    let client = client.connect_timeout(Duration::from_secs(3));
    // let client = client.min_tls_version(reqwest::tls::Version::TLS_1_2);
    client.build().unwrap()
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
    let wiki = format!("{}.wiki", config.repo);
    let wiki_path = Path::new(&wiki);
    log::debug!("Cloning {}", &wiki);
    let command = format!("git clone https://github.com/{}/{}.git", config.owner, wiki);
    if git::run(command.as_str()) && !wiki_path.exists() {
        log::error!("Cloning was successful but wiki path wasn't found");
    }
    let client = request_builder();
    let errors = Arc::new(Mutex::new(Vec::new()));
    let counter: Arc<Mutex<HashMap<String, Arc<Mutex<i32>>>>> = Arc::new(Mutex::new(HashMap::new()));
    let current_working_dir = env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let mut threads = Vec::new();
    for md_file in files::get_markdown() {
        let md_file_cloned = md_file.clone();  // clone due to use in closure
        let client_cloned = client.clone();
        let exclusions_cloned = exclusions.clone();
        let counter_cloned = counter.clone();
        let errors_cloned = errors.clone();
        let current_working_dir_cloned = current_working_dir.clone();
        let handle = thread::spawn(move || {
            let validation_result = runner(
                &md_file_cloned,
                current_working_dir_cloned,
                exclusions_cloned,
                counter_cloned,
                client_cloned,
            );
            log::info!(
                "Scanned '{}' with {} URLs",
                md_file_cloned.split('/').last().unwrap().to_string(),
                validation_result.count
            );
            let mut locked_errors = errors_cloned.lock().unwrap();
            locked_errors.extend(validation_result.errors);
        });
        threads.push((md_file, handle));
    }
    for (file, handle) in threads {
        if handle.join().is_err() {
            log::error!("Error awaiting thread: {}", file)
        }
    }
    let errors_cloned = errors.lock().unwrap().clone();
    if verify_actions().unwrap_or(false) && !errors_cloned.is_empty() {
        generate_summary(errors_cloned);
    }
    if wiki_path.exists() {
        match remove_dir_all(wiki_path) {
            Ok(_) => log::info!("Removed {:?}", &wiki_path),
            Err(err) => log::error!("Failed to delete: {}", err)
        }
    }
    squire::unwrap(counter);
    let elapsed = start.elapsed();
    log::info!("'none-shall-pass' protocol completed. Elapsed time: {:?}s", elapsed.as_secs());
}
