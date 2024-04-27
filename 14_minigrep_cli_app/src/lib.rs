// Root of library crate

use std::error::Error;
use std::{env, fs};

/*
    Function to run, read and manipulate with
    file contents, takes a config as parameter
*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Get file contents
    let contents: String = fs::read_to_string(config.filename)?;

    // Check for case sensitive
    let resulsts = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // Print all lines, that contains a provided query
    for line in resulsts {
        println!("{}", line);
    }

    Ok(())
}

/*
  Function to search the contents in the file,
  using query from function parameters
*/
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Simplify function using collection methods
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/*
  Function to search the contents in the file,
  using query from function parameters (case insensitivly)
*/
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    // Simplify function using collection methods
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

/*
  Struct to save information about
  parsed arguments needed for the application,
  in our case (query and filename)
*/
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

/*
  Implement the config struct with needed methods
*/
impl Config {
    /*
        Function to parse our cli arguments config,
        and return tuple with query and filename.
    */
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Will skip first cmd line argument, which is a path to a binary

        // Get argument variable for a query
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string!"),
        };

        // Get argument variable for a filename
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string!"),
        };

        // Get environment variable to check if we are using case insensitive
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

/*
  Define a test modules, to check if
  some functionality are worked
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
