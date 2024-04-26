#[allow(dead_code)] // Allow to dead code (not used variables)
#[allow(unused_variables)] // Supress warning about declared unused variables

fn main() {
    /* VARIABLES */

    // By default all variables in Rust, without marked with `mut` keyword, was immutable,
    // so cannot be changed after declaration
    let mut x = 5; // Make mutable variable, that can be changed after declaration, by adding `mut` keyword
    println!("The value of `x` is: {}", x);
    x = 6; // Change value of `x` variable to 6
    println!("The value of `x` after change is: {}", x);

    // Rust also supports `Shadowing`, which is proccess of creating variables in the same name
    // for example:
    let y = 10; // Firsly, define `y` variable with some value
    let y = "test"; // Then, create variable with the same name, but another value (may appear another type)

    // Also Rust has an constant variables. Difference between `let` immutable and constant one is next:
    // - Constants must to be annotated with type, when immutable variables may not
    // - We cant mutate a constant variable, if constant declared, it will own the same value for all process live
    const SUBSRIBER_COUNT: u32 = 100_000; // Also, in Rust we have `_` keyword, which allows to separate big numbers,
                                          // e.g: 100_000 OR 100000

    /* DATA TYPES */
    // Rust have the next scalar data types, which is represent only one value:
    // Integers
    // Floating-point numbers
    // Booleans
    // Characters

    // Integers:
    /* Length:          Signed:         Unsigned: */
    /* 8-bit            i8              u8        */
    /* 16-bit           i16             u16       */
    /* 32-bit           i32             u32       (used by default if declare number variable implicitly) */
    /* 64-bit           i64             u64       */
    /* 128-bit          i128            u128      */
    /* arch             isize           usize     */
    let a = 98_222; // Decimal
    let b = 0xfff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)

    // Floating-point numbers
    // By default, Rust use f64 for floating-point numbers
    let f = 2.0;
    let g: f32 = 3.0;

    // Addition
    let sum = 5 + 10;
    // Substraction
    let difference = 95.5 - 4.3;
    // Division
    let quotient = 56.7 / 32.2;
    // Remainder
    let remainder = 43 % 5;

    // Booleans (has value true or false)
    let t = true;
    let f = false;

    // Characters
    let c = 'z';
    let z = 'Z';
    let angry_emoji = '😡';

    /* COMPUND TYPES */
    // Tuple type - type that can group some values in one (comma separated list), e.g:
    let tup = ("Welcome, Rust Book!", 100_000);
    let (welcome_text, price) = tup; // Subtract tuple data to variables
    let price = tup.1; // Also we could get the value by index in tuple

    // Arrays
    let error_codes = [200, 404, 500]; // To create an array, we are use brackets and values separated by comma
    let not_found = error_codes[1]; // To get the value from array, we need to specify an index in brackets after array variable name
                                    // let invalid_index = error_codes[3]; // We also cannot access index, which is not exists, application will panic on start

    // Arrays also can be declared by `;`, which is says in current scope,
    // create an array with 8 values and all set to 0
    let byte = [0; 8];

    /* FUNCTIONS */
    let sum = my_function(1, 20);
    println!("The sum of numbers is: {}", sum);

    /* CONTROL FLOW */
    let number = 5;

    // Rust has `if` statement, to compare the state of some variables,
    // which accepts only `boolean` values as input, the syntax is next:
    if number < 10 {
        // First we check this one
        println!("First condition was true!");
    } else if number < 22 {
        // If first one returns false, we check `else if` block
        println!("Second condition was true!");
    } else {
        // If every `else if` block returns `false`, we go to the `else` one
        println!("Condition was false!");
    }

    // `if` statement can be used also to declare variable, e.g:
    let condition = true;
    let number = if condition { 5 } else { 6 };

    /* LOOPS */
    // In Rust we have tree types of loop: `loop`, `while` and `for`

    // Loop `loop` will executes until we call a `break` keyword, e.g:
    let mut counter = 0;
    loop {
        println!("Looping through `loop`");
        counter += 1;

        if counter == 5 {
            break;
        };
    }

    // Loop `while`, will be executed for the moment, when given expression will be `true`, e.g:
    let mut condition = true;
    while condition {
        println!("Condition is true!");

        condition = false; // loop will be exectued one time, because after first one we changed the condition values to `false`
    }

    // Loop `for`, will used for looping though some sword of collections, e.g:
    let collection = [10, 20, 30, 40, 50, 60];
    for value in collection.iter() {
        println!("Current loop value in collection: {}", value);
    }
    // Or, we could loop through range type, represented by standart library, e.g:
    for number in 1..10 {
        println!("Range value: {}", number);
    }

    /* COMMENTS */
    // As you can see above, Rust has 2 types of comments:
    // - one line comment
    /* - block comment (multi-line) */
}

/* FUNCTIONS */
// To create function in Rust, we use `fn` keyword
// By default, functions are declared in current scope, and cannot be used from another scope or file.
// To allow this, we need to specify `pub` keyword before `fn` one

// By default, functions also are returns void (no value), if we are want to return an value,
// use the `->` syntax after function declarating and specify return type
fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // To return an value from a function, we can use `return` keyword, and pass after value to return,
    // or simply leave nave of the variable without any keyword at the end of the file
    x + y
}
