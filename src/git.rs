use std::process::Command;

pub fn run(command: &str) -> bool {
    let output = Command::new("sh")  // invoke a shell
        .arg("-c")  // execute command as interpreted by program
        .arg(command)  // run the command
        .status()  // check for status
        .expect("Failed to execute command");
    if output.success() {
        true
    } else {
        println!("ERROR: Command failed with an error code: {:?}", output.code());
        false
    }
}
