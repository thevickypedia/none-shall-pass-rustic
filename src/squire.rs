use std::collections::HashMap;
use std::env;
use std::net::ToSocketAddrs;
use std::sync::{Arc, Mutex};
use log::info;

pub fn unwrap(counter: Arc<Mutex<HashMap<String, Arc<Mutex<i32>>>>>) {
    let counter_lock = counter.lock().unwrap();  // Lock counter to stop updating
    let success = counter_lock.get("success");  // Extract success values
    let mut success_count = 0;  // Set default value for success
    let failed = counter_lock.get("failed");  // Extract failed values
    let mut failed_count = 0;  // Set default value for failed
    if success.is_some() {
        success_count = *success.unwrap().lock().unwrap();
    }
    if failed.is_some() {
        failed_count = *failed.unwrap().lock().unwrap();
    }
    info!("URLs successfully validated: {}", success_count);
    info!("URLs failed to validate: {}", failed_count);
    info!("Total URLs validated: {}", success_count + failed_count);
}

pub fn get_exclusions() -> Vec<String> {
    let mut exclusions: Vec<String> = vec!["localhost".to_string(), "127.0.0.1".to_string()];
    let server_details = "localhost:80";
    let server: Vec<_> = server_details
        .to_socket_addrs()
        .expect("Unable to resolve domain")
        .collect();
    for slice in server.as_slice() {
        let server_ip = slice.ip().to_string();
        if server_ip.starts_with("::") {
            continue;
        }
        exclusions.push(server_ip);
    }
    exclusions
}

pub fn get_exit_code() -> i32 {
    let string = env::var("exit_code").unwrap_or("0".to_string());
    string.parse::<i32>().unwrap_or(0)
}
