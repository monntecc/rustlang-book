use std::env;
use std::process;

use minigrep_cli_app::Config;

fn main() {
    // Collect arguments from the cli, passed after cargo run command,
    // or executable, e.g: (cargo run test poem.txt) or (./app.exe test poem.exe)
    let args: Vec<String> = env::args().collect();

    // Get query and filename from cli arguments
    let config: Config = Config::new(&args).unwrap_or_else(|error| {
        // Print error information
        eprintln!("Problem parsing arguments: {}", error);
        // Exit the application
        process::exit(1);
    });

    println!("Searching for: `{}`", config.query);
    println!("In file: `{}`", config.filename);

    // Check for some errors in run function
    if let Err(error) = minigrep_cli_app::run(config) {
        // Print error information
        eprintln!("Application error: {}", error);
        // Exit the application
        process::exit(1);
    }
}
