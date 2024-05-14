extern crate reqwest;

use std::error::Error;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use lookup::Hyperlink;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Response {
    pub ok: bool,
    #[serde(default = "default_response")]
    pub response: String
}

fn default_response() -> String { String::new() }

pub fn verify_url(hyperlink: &Hyperlink, exclusions: Vec<String>, request: Client) -> Response {
    let text = hyperlink.text.to_string();
    let url = hyperlink.url.to_string();
    let response = request.get(&url).send();
    let error_reason;
    match response {
        Ok(ok) => {
            let status_code = ok.status().as_u16();
            if status_code < 400 {
                log::debug!("'{}: {}' - {}", text, url, ok.status());
                return Response { ok: true, ..Default::default() };
            }
            error_reason = format!("'{}: {}' resolved but returned '{}'", text, url, ok.status());
            if status_code == 429 || status_code == 403 {
                // too many requests or forbidden
                log::warn!("{}", error_reason);
                return Response { ok: true, ..Default::default() };
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
            return Response { ok: true, ..Default::default() };
        }
    }
    log::error!("{}", error_reason);
    return Response { ok: false, response: error_reason };
}
