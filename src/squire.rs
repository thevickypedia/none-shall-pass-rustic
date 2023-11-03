use std::env;
use std::net::ToSocketAddrs;
use std::sync::{Arc, Mutex};

pub fn increment_counter(counter: Arc<Mutex<i32>>) {
    let mut count = counter.lock().unwrap();
    *count += 1;
}

pub fn get_exclusions() -> Vec<String> {
    let mut exclusions: Vec<String> = vec!["localhost".to_string(), "127.0.0.1".to_string()];
    let server_details = "localhost:80";
    let server: Vec<_> = server_details
        .to_socket_addrs()
        .expect("Unable to resolve domain")
        .collect();
    for slice in server.as_slice() {
        let server_ip = slice.ip().to_string().clone();
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
