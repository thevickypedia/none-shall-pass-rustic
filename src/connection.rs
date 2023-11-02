extern crate reqwest;

use std::env;

pub fn verify_url(hyperlink: (String, String), exclusions: Vec<String>) {
    let (text, url) = hyperlink;  // type string which doesn't implement `Copy` trait
    let resp = reqwest::blocking::get(url.clone());  // since value is moved
    if resp.is_ok() {
        return;
    }
    for exclusion in exclusions {
        if url.contains(&exclusion) {
            println!("'{}' failed to resolve but excluded", url);
            return;
        }
    }
    // without clone, value will be borrowed after move
    println!("'{}' - '{}' failed to resolve", text, url);
    println!("Setting exit code to 1");
    env::set_var("exit_code", "1");
    if env::var("debug").unwrap().as_str() == "true" {
        if resp.is_err() {
            println!("{:?}", resp.err());
        } else {
            println!("{:?}", resp.unwrap().error_for_status());
        }
    }
}
