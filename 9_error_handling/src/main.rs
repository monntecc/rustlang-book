/*
    Error handling in Rust more different approaches to do.
    To throw an error which would crash a program, you can use a `panic!` macro, e.g:

    fn main() {
        a();
    }

    fn a() {
        b();
    }

    fn b() {
        c(22);
    }

    fn c(num: i32) {
        if num == 22 {
            panic!("Don't pass in 22!"); // Will panicf
        }
    }

    We will get an error at compile, without any information who call this function.
    To see it, just export RUST_BACKTRACE=1 environment, and console will print more
    information about a crash.

*/

#[allow(dead_code)]
#[allow(unused_variables)]
use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    /*
        First approach of the handle the errors, is the Result enum,
        which is look like this:

        enum Result<T, E> {
            Ok(T),
            Err(E),
        }

        e.g:
    */
    // This function will return an result enum value
    let f = File::open("hello.txt");

    // To handle error, we could use a match keyword
    let f = match f {
        Ok(file) => file, // If ok, return the file
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {}", other_error)
            }
        },
    };

    // We could handle the error also using the closures (more about this in 13 chapter)
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Also we can use `.unwrap()` expression, to indicate that we could get an error, e.g:
    let f = File::open("hello.txt").unwrap(); // We could get the file contents, or panic!
                                              // Or use `.expect()` expression, to specify error message to panic! macro, e.g:
    let f = File::open("hello.txt").expect("Failed to open hello.txt file!");
}

// Function to read username from file
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    // Or we can simplify the code, without needing of match expression, e.g:
    // let f = File::open("hello.txt")?; - by adding `?`,
    // which is would end function earlier and return the error

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Refactored function to be easier
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Or we can simplify a more
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// Or even a more
fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

/*
    About the main function, which by default returns nothing.
    We could modify it to return an error, for example:

    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;

        Ok(())
    }
*/
