extern crate reqwest;

use std::error::Error;
use std::time::Duration;

pub fn verify_url(hyperlink: &(String, String), exclusions: Vec<String>) -> bool {
    let (text, url) = hyperlink;
    let client = reqwest::blocking::ClientBuilder::new().user_agent("rustc");
    let client = client.connect_timeout(Duration::from_secs(3));
    // let client = client.min_tls_version(reqwest::tls::Version::TLS_1_2);
    let request = client.build();
    let response = request.unwrap().get(url).send();
    let error_reason;
    match response {
        Ok(ok) => {
            let status_code = ok.status().as_u16();
            if status_code < 400 {
                log::debug!("'{}: {}' - {}", text, url, ok.status());
                return true;
            }
            error_reason = format!("'{}: {}' resolved but returned '{}'", text, url, ok.status());
            if status_code == 429 || status_code == 403 {
                // too many requests or forbidden
                log::warn!("{}", error_reason);
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
            log::warn!("{} but excluded", error_reason);
            return true;
        }
    }
    log::error!("{}", error_reason);
    false
}
