extern crate reqwest;
extern crate glob;
extern crate regex;

use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::process::{Command, exit};
use std::thread;
use std::time::Instant;

use glob::glob;
use regex::Regex;

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn get_patterns() -> Vec<&'static str> {
    let inline_link_re = r"\[([^\]]+)\]\(([^)]+)\)";
    let footnote_link_text_re = r"\[([^\]]+)\]\[(\d+)\]";
    let footnote_link_url_re = r"\[(\d+)\]:\s+(\S+)";
    let anchored_link_re = r"\[([^\]]+)\]:\s+(\S+)";
    return vec![inline_link_re, footnote_link_text_re, footnote_link_url_re, anchored_link_re];
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
    return md_files;
}

fn run_git_cmd(command: &str) -> bool {
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

fn verify_url(hyperlink: (String, String)) {
    let (text, url) = hyperlink;  // type string which doesn't implement `Copy` trait
    let resp = reqwest::blocking::get(url.clone());  // since value is moved
    if resp.is_ok() {
        return;
    }
    // without clone, value will be borrowed after move
    println!("'{}' - '{}' failed to resolve", text, url);
    println!("Setting exit code to 1");
    env::set_var("exit_code", "1");
    // todo: this should happen only when debug is enabled
    // if resp.is_err() {
    //     println!("{:?}", resp.err());
    // } else {
    //     println!("{:?}", resp.unwrap().error_for_status());
    // }
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
    for pattern in get_patterns() {
        let regex = Regex::new(pattern).expect("Failed to compile regex");
        for capture in regex.captures_iter(&text) {
            if let (Some(name), Some(url)) = (capture.get(1), capture.get(2)) {
                let name = name.as_str().to_string();
                let url = url.as_str().to_string();
                let handle = thread::spawn(move || {
                    verify_url((name, url))
                });
                threads.push(handle);
            }
        }
    }
    for handle in threads {
        if let Err(_) = handle.join() {
            fail = true;
        }
    }
    return fail;
}

fn get_exit_code() -> i32 {
    let s = env::var("exit_code").unwrap_or("0".to_string());
    match s.parse::<i32>() {
        Ok(num) => num,
        Err(_) => 0,
    }
}

fn main() {
    let start = Instant::now();
    let arguments: Vec<String> = env::args().collect();
    let owner = &arguments[1];
    let repo = &arguments[2];
    let wiki_path = format!("{}.wiki", repo);
    let command = format!("git clone https://github.com/{}/{}.git", owner, wiki_path);
    if run_git_cmd(command.as_str()) {
        let path = Path::new(wiki_path.as_str());
        if !path.exists() {
            println!("Setting exit code to 1");
            env::set_var("exit_code", "1");
        }
    }
    for md_file in md_files() {
        runner(&md_file);
    }
    let elapsed = start.elapsed();
    println!("Run time: {}s", elapsed.as_secs());
    exit(get_exit_code());
}
