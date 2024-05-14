use std::env;
use std::process::exit;

/// Represents the command-line arguments for none-shall-pass-rustic.
pub struct Args {
    pub owner: String,
    pub repo: String,
    pub debug: bool,
    pub exclude: Vec<String>,
}

/// Parses and returns the command-line arguments for RuStream.
///
/// # Returns
///
/// An `Args` struct containing parsed command-line arguments.
pub fn arguments() -> Args {
    let args: Vec<String> = env::args().collect();

    let mut debug = false;

    let mut version = false;
    let mut owner = String::new();
    let mut repo = String::new();
    let mut exclude_me = String::new();

    // Loop through the command-line arguments and parse them.
    let mut i = 1; // Start from the second argument (args[0] is the program name).
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                let helper = "Takes the arguments, debug, \
                --owner, --repo, --exclude, and --version/-v\n\n\
        --debug: Optional boolean value to enable debug level logging.\n\
        --owner: The account owner of the repository. The name is not case sensitive.\n\
        --repo: The name of the repository without the .git extension. The name is not case sensitive.\n\
        --exclude: Optional list of hostnames (whitespace separated) to be excluded.\n"
                    .to_string();
                println!("Usage: {} [OPTIONS]\n\n{}", args[0], helper);
                exit(0)
            }
            "-V" | "-v" | "--version" => {
                version = true;
            }
            "--owner" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    owner.clone_from(&args[i]);
                } else {
                    println!("\n--owner\n\tInput requires a value [type=missing]\n");
                    exit(1)
                }
            }
            "--repo" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    repo.clone_from(&args[i]);
                } else {
                    println!("\n--repo\n\tInput requires a value [type=missing]\n");
                    exit(1)
                }
            }
            "--exclude" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    exclude_me.clone_from(&args[i]);
                } else {
                    println!("\n--exclude\n\tInput requires a value [type=missing]\n");
                    exit(1)
                }
            }
            "--debug" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    debug = match args[i].clone().as_str() {
                        "true" => true,  // true as true
                        "1" => true,  // 1 as true
                        _ => false  //  anything else? set debug to false
                    }
                } else {
                    println!("\n--debug\n\tInput requires a value [type=missing]\n");
                    exit(1)
                }
            }
            _ => {
                println!("Unknown argument: {}", args[i]);
                exit(1)
            }
        }
        i += 1;
    }
    if version {
        const PKG_NAME: &str = env!("CARGO_PKG_NAME");
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("{} {}", PKG_NAME, VERSION);
        exit(0)
    }
    if owner.is_empty() || repo.is_empty() {
        println!("\n--owner | --repo\n\tMandatory requirement unsatisfied [tpe=missing]\n");
        exit(1)
    }
    let mut exclude = Vec::new();
    if !exclude_me.is_empty() {
        exclude.extend(exclude_me.split(' ').filter(|s| !s.is_empty()).map(String::from));
    }

    Args {
        owner,
        repo,
        debug,
        exclude,
    }
}
