use std::process::{Command, Stdio};


pub fn run(command: &str) -> bool {
    let output = Command::new("sh")  // invoke a shell
        .arg("-c")  // execute command as interpreted by program
        .arg(command)  // run the command
        .stdout(Stdio::null())  // Redirect stdout to /dev/null
        .stderr(Stdio::null())  // Redirect stderr to /dev/null
        .status();  // check for status
    match output {
        Ok(status) => {
            if status.success() {
                log::debug!("GitHub wiki cloned successfully");
                true
            } else {
                log::warn!("Command failed with an error code: {:?}", output.unwrap().code());
                false
            }
        },
        Err(err) => {
            log::error!("Failed to execute command: {}", err);
            false
        },
    }
}
