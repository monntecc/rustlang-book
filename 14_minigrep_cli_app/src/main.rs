use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    // Collect arguments from the cli, passed after cargo run command,
    // or executable, e.g: (cargo run test poem.txt) or (./app.exe test poem.exe)
    let args: Vec<String> = env::args().collect();

    // Get query and filename from cli arguments
    let config: Config = Config::new(&args).unwrap_or_else(|error| {
        // Print error information
        println!("Problem parsing arguments: {}", error);
        // Exit the application
        process::exit(1);
    });

    println!("Searching for: `{}`", config.query);
    println!("In file: `{}`", config.filename);

    // Check for some errors in run function
    if let Err(error) = run(config) {
        // Print error information
        println!("Application error: {}", error);
        // Exit the application
        process::exit(1);
    }
}

/*
    Function to run, read and manipulate with
    file contents, takes a config as parameter
*/
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Get file contents
    let contents: String = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

/*
    Struct to save information about
    parsed arguments needed for the application,
    in our case (query and filename)
*/
struct Config {
    query: String,
    filename: String,
}

/*
    Implement the config struct with needed methods
*/
impl Config {
    /*
        Function to parse our cli arguments config,
        and return tuple with query and filename.
    */
    fn new(args: &[String]) -> Result<Config, &str> {
        // Check if length of the arguments is not smaller than 3
        if args.len() < 3 {
            return Err("Not enough arguments, please specify: `query` and `filename`");
        }

        // Get second argument variable ('cause first one is by default an executable path),
        // for searching query
        let query = args.get(1).unwrap().to_string().clone();
        // Get third argument variable for a filename
        let filename = args.get(2).unwrap().to_string().clone();

        Ok(Config { query, filename })
    }
}
