use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn unwrap(counter: Arc<Mutex<HashMap<String, Arc<Mutex<i32>>>>>) {
    let counter_lock = counter.lock().unwrap();  // Lock counter to stop updating
    let success = counter_lock.get("success");  // Extract success values
    let mut success_count = 0;  // Set default value for success
    let failed = counter_lock.get("failed");  // Extract failed values
    let mut failed_count = 0;  // Set default value for failed
    // Ref: https://rust-lang.github.io/rust-clippy/master/index.html#/unnecessary_unwrap
    if let Some(success_ref) = success {
        success_count = *success_ref.lock().unwrap();
    }
    if let Some(failed_ref) = failed {
        failed_count = *failed_ref.lock().unwrap();
    }
    log::info!("Hyperlinks resolved: {}", success_count);
    log::info!("Hyperlinks unresolved: {}", failed_count);
    log::info!("Total hyperlinks validated: {}", success_count + failed_count);
}
