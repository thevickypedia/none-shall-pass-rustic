extern crate reqwest;

use std::env;
use std::error::Error;
use std::time::Duration;

use log::{debug, error, warn};

pub fn verify_url(hyperlink: (String, String), exclusions: Vec<String>) {
    let (text, url) = hyperlink;  // type string which doesn't implement `Copy` trait
    let client = reqwest::blocking::Client::new();
    let request = client.get(url.clone());
    let request_with_timeout = request.timeout(Duration::from_secs(3));
    let resp = request_with_timeout.send();
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
        debug!("Status: {}", resp.err().unwrap().source().unwrap().to_string());
        // debug!("{:?}", resp.err());
    } else {
        debug!("{:?}", resp.unwrap().error_for_status());
    }
}
