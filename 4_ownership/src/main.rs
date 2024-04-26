#[allow(dead_code)]
#[allow(unused_variables)]

/* OWNERSHIP */

// Memory managment difference table:
/*
TYPE:                   PROS:                               CONS:

Garbage                 - Error free                        - No control over memory
collection:             - Faster write time                 - Slower and unpredcitable runtime performace
                                                            - Larger program size

Manual memory
management:             - Control over all memory           - Error prone
                        - Faster runtime                    - Slower write time
                        - Smaller program size


Ownership               - Control over all memory           - Slower write time.
model:                  - Error free*                       - Learning curve (fighting with the borrow checker)
                        - Faster runtime
                        - Smaller program size
*/

/* STACK AND HEAP */
// During runtime, Rust has access to both Stack and Heap.

// Stack is fixed size, which is calculated in compile time, and cannot growth own size during runtime.
// Stack stores in himself `stack frames`, which is created for every function that executes on program.
// Also `stack frames` store the local variables of the function being executed.
// As the stack, `stack frames` also has fixed size and was calculated during compile time, so every variable must be on some size.
// After function executing, the `stack frame` was removed from the stack and memory with variable was cleaned.

// Heap has dynamic/flexible size, and can be growth in runtime.
// Data stored in the heap, can be dynamic inside, and we are control the lifetime of the data.
// Massive types, own or system (like String type), stores the value in the heap, but pointer for the variable,
// was taken by the stack

// Also accessing data on the stack will be much faster than on the heap,
// because stack has an pointer to value on heap

fn main() {
    // This function will be stored on the stack,
    // Variables `x` and `y` also,
    fn a() {
        let x = "hello";
        let y = 22;
        b();
    }

    // This function will be stored on the stack,
    // value of variable `x`, will be stored on the heap,
    // but pointer of the `x` variable, will go to the stack (because String is an dynamic type)
    fn b() {
        let x = String::from("world!");
    }

    /* OWNERSHIP RULES */
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // s is not valid here, it's not yed declared
    {
        let s = String::from("hello"); // s is valid from this point forward
                                       // do staff with s
    }
    // this scope is now over, and s is no longer valid

    // Data interapting

    let x = 5;
    let y = x; // Copy, will be copied to y variable on the stack

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Here we are cloning the value, not moving
    let s3 = s1; // It was `moving`, so s2 will be an pointer to the same value on the heap, as the s1 variable

    // println!("{}, world", s1); // variable s1 already moved to s2, so owner of this value now is s2 and s1 cannot be accessible more
    println!("{}, world", s3); // it is OK

    /* OWNERSHIPS AND FUNCTIONS */
    let s = String::from("hello");
    takes_ownership(s);
    // println!("s variable: {}", s); // we will be an error, because variable was moved (borrowed) by function,
    // and after the function finished, variable was dropped

    let x = 5;
    makes_copy(x);
    println!("x variable: {}", x); // will be ok, because we are make the copy of the variable in the stack

    let s1 = gives_ownership();
    println!("s1 = {}", s1); // we are move ownership of `some_string` variable from the function, to `s1` here

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3); // we will get an error on s2 variable,
    // because we are taking s2 ownership to the `takes_and_gives_back` and return it to the s3 variable, so:
    println!("s1 = {}, s3 = {}", s1, s3); // will be ok

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len); // will be ok, because the `calculate_length` returns back an
                                                    // ownership of s1 value

    // Rust also has an references `&`, which is obviously pointer to the variable that points to the value,
    // e.g:
    /*
        let s1 = String::from("hello"); - pointer will be stored on the stack, but value on the heap
        let s2 = &s1; // reference to the s1 variable

        s2      ->      s1      ->      "hello"

        So, s2 will have an pointer to the s1 variable, but s1 variable has an pointer to the raw value on the heap
        s1 and s2 variable both are stored in on the stack (only addresses to the values), so (pointers)
    */
    let s1 = String::from("hello");
    let len = calculate_length_fix(&s1);
    println!("The length of '{}' is {}.", s1, len); // will be ok also, because of passing reference

    // Also, variable passed by reference cannot be modified, only accessible,
    // if the reference not passed with `mut` keyword, e.g:
    let mut s1 = String::from("hello");
    println!("s1 before modifing = {}", s1); // hello
    change_value(&mut s1);
    println!("s1 after modifing = {}", s1); // hello, world

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // will be an error, because we cannot borrow mutable reference more than one at a time
    // println!("{}, {}", r1, r2); - errored ne
    println!("{}", r1); // will be ok

    // One more tip about references, you cannot cannot borrow variable as mutable,
    // when it was already borrowed as immutable, e.g:
    /*
       let mut s = String::from("hello");

       let r1 = &s; // ok
       let r2 = &s;  // ok
       let r3 = &mut s; // an error, cannot borrow, because it is already borrowed as immutable
    */
    // If we want to do something like that, we will be able only after previous references will be out of scope, e.g:
    /*
       let mut s = String::from("hello");

      let r1 = &s; // ok
      let r2 = &s;  // ok

      println!("{}, {}", r1, r2);

      let r3 = &mut s; // also ok
      println!("{}", r3);
    */

    // Reference to nothing
    // let reference_to_nothing = dangle(); - an error, we cannot do this, because variable,
    // because variable will be dropped after function exectued

    /* THE RULES OF REFERENCES */
    // 1. At any given time, you can have either one mutable reference
    // or ony number of immutable references.
    //
    // 2. References must always be valid/

    /* SLICES */
    let mut s = String::from("hello world");
    println!("s before slicing = {}", s);
    let hello = &s[..5]; // hello
    println!("hello variable = {}", hello);
    let world = &s[5..s.len()]; // world
    println!("world variable = {}", world);

    let word = first_word(&s); // we will get here `hello`
    println!("first word from s = {}", word);
    s.clear(); // clear the string

    let s2 = "hello world"; // by default, if we are declare string with "", we will get the reference slice type

    // We can use a slices for another types, e.g integers:
    let a = [1, 2, 3, 4, 5, 6];
    let slice = &a[0..2]; // [1, 2, 3]
}

/* OWNERSHIPS AND FUNCTIONS */

// This function takes ownership from the variable,
// after function destroyed, variable will be dropped and not accessible no more
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

// This function makes a copy on the stack,
// because i32 is a fixed type, so will be copied (not moved)
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// This function create a variable with ownership storing in self,
// but return it and ownership was moved to the new owner
fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

// This function will get the ownership for operation time,
// and after that will return it back
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// This function takes a string, calculates the size of it,
// and returns tuple of the passed string and his size
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

// Fix the function above, to not copy an ownership,
// but takes the reference to it
fn calculate_length_fix(s: &String) -> usize {
    let length = s.len();

    length
}

// Function that takes mutable reference to the variable,
// so value can be modified
fn change_value(some_string: &mut String) {
    some_string.push_str(", world");
}

// We cannot do this, because variable `s` was created in scope of this function,
// and after function executes, `s` will be dropped.
// So we couldn't return the reference of the value, which is dropped out of scope after execution
/*
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
*/

// Function that accepts a reference to the slice,
// and finding the first word of that
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // returning the first word, before space using string slice
        }
    }

    &s[..] // returning the slice of entire string
}
