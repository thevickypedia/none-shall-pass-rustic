use std::process::{Command, Stdio};


pub fn run(command: &str) -> bool {
    let output = Command::new("sh")  // invoke a shell
        .arg("-c")  // execute command as interpreted by program
        .arg(command)  // run the command
        .stdout(Stdio::null())  // Redirect stdout to /dev/null
        .stderr(Stdio::null())  // Redirect stderr to /dev/null
        .status()  // check for status
        .expect("Failed to execute command");
    if output.success() {
        log::debug!("GitHub wiki cloned successfully");
        true
    } else {
        log::warn!("Command failed with an error code: {:?}", output.code());
        false
    }
}
