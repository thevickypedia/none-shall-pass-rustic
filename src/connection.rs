extern crate reqwest;

use std::env;

use log::{debug, error, warn};

pub fn verify_url(hyperlink: (String, String), exclusions: Vec<String>) {
    let (text, url) = hyperlink;  // type string which doesn't implement `Copy` trait
    let resp = reqwest::blocking::get(url.clone());  // since value is moved
    if resp.is_ok() {
        return;
    }
    for exclusion in exclusions {
        if url.contains(&exclusion) {
            warn!("'{}' failed to resolve but excluded", url);
            return;
        }
    }
    // without clone, value will be borrowed after move
    error!("'{}' - '{}' failed to resolve", text, url);
    error!("Setting exit code to 1");
    env::set_var("exit_code", "1");
    if resp.is_err() {
        debug!("{:?}", resp.err());
    } else {
        debug!("{:?}", resp.unwrap().error_for_status());
    }
}
