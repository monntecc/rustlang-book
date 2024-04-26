use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    // Generate random number (from 1 to 100), 101 in excluded
    let generated = rand::thread_rng().gen_range(1..101);

    //println!("The generated number: {}", generated);

    // Main game loop
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        // Read user input from the console
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // Shadowing variable
        // Preverse input from errors, until we will get a valid number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&generated) {
            Ordering::Less => println!("{}", "Too small! Try again.".red()),
            Ordering::Greater => println!("{}", "Too big! Try again.".red()),
            Ordering::Equal => {
                println!("{}", "You won!".green());
                break;
            }
        }
    }
}
