extern crate reqwest;

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use log::{error, warn};

use squire;

pub fn verify_url(hyperlink: (String, String), exclusions: Vec<String>, counter: Arc<Mutex<i32>>)
    -> bool {
    let (text, url) = hyperlink;  // type string which doesn't implement `Copy` trait
    squire::increment_counter(counter);
    let client = reqwest::blocking::ClientBuilder::new();
    let client = client.connect_timeout(Duration::from_secs(3));
    // let client = client.min_tls_version(reqwest::tls::Version::TLS_1_2);
    let request = client.build();
    let response = request.unwrap().get(&url).send();
    let error_reason;
    match response {
        Ok(ok) => {
            let status_code = ok.status().as_u16();
            if status_code < 400 {
                return true;
            }
            error_reason = format!("'{}: {}' resolved but returned '{}'", text, url, ok.status());
            if status_code == 429 {
                // too many requests
                warn!("{}", error_reason);
                return true;
            }
        }
        Err(err) => {
            error_reason = format!("'{}: {}' failed to resolve - {:?}",
                                   text, url, err.source().unwrap().to_string())
        }
    }
    for exclusion in exclusions {
        if url.contains(&exclusion) {
            warn!("{} but excluded", error_reason);
            return true;
        }
    }
    error!("{}", error_reason);
    return false;
}
