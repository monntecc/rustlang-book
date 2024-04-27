/*
    Closures -  a function-like construct you can store in a variable:

    Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
    You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context.
    Unlike functions, closures can capture values from the scope in which they’re defined.
    We’ll demonstrate how these closure features allow for code reuse and behavior customization.
*/

#[allow(dead_code)]
#[allow(unused_variables)]
use std::{thread, time::Duration};

/*
    Struct Cacher was created for storing cached our closure data inside.
    Accepts a T generic parameter, which is type of simple function with one
    parameter of type u32 and return type of u32.

    Contains two fields:
    - calculation - for closure generic type
    - value - optional type which holds our closure value
*/
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // Will store our closure
    calculation: T,
    // Will store a value from a closure
    value: Option<u32>,
}

// Implement the Cacher struct
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // Define a constructor, which accepts a closure
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // Function that returns a value from a closure
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg); // Call our closure with arg parameter
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    /*
        Here we are creating an simple closure.
    */
    let mut cashed_result = Cacher::new(|num: u32| -> u32 {
        println!("Calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    });
    /*
        NOTICE
        Closures behind a functions, must not be explicitly annotated!
        Parameters and return type may stay without annotation,
        which is will be automatically added by compiler.

        e.g:
    */
    let example_closure = |x| x; // We do not specified a types here
    let _ = example_closure(String::from("hello")); // After this line, compiler will automatically set type String to closure
                                                    // let s = example_closure(5); -- this otherwise will return an error, because closure type already in String

    if intensity > 25 {
        println!("Today, do {} pushups!", cashed_result.value(intensity)); // To call closure, we use functional syntax
        println!("Next, do {} situps!", cashed_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to say hydrated!");
        } else {
            println!("Today, run for {} minutes!", cashed_result.value(intensity))
        }
    }
}

fn main() {
    // Workout simmulation
    let intensity: u32 = 43;
    let random_number: u32 = 8;
    // Call generate workout function
    generate_workout(intensity, random_number);

    /*
        Capturing the environment with closures,
        e.g:
    */
    let x = 4;
    let equal_to_x = |z| z == x; // in that case, closure does not take an ownership from `x` variable
                                 // if we want to move ownership, we need to specify before closure declaration a `move` keyword, e.g:
                                 /*
                                     let x = 5;
                                     let closure = move |z| z == x;
                                     println!("{}", x); // error
                                 */
    let y = 4;
    assert!(equal_to_x(y));
}
