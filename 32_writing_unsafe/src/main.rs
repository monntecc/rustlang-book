/*
    All the code we’ve discussed so far has had Rust’s memory safety guarantees
    enforced at compile time. However, Rust has a second language hidden inside it that doesn’t
    enforce these memory safety guarantees: it’s called unsafe Rust and works just like regular Rust,
    but gives us extra superpowers.

    Unsafe Rust exists because, by nature, static analysis is conservative. When the compiler
    tries to determine whether or not code upholds the guarantees, it’s better for it to reject
    some valid programs than to accept some invalid programs. Although the code might be okay,
    if the Rust compiler doesn’t have enough information to be confident, it will reject the code.
    In these cases, you can use unsafe code to tell the compiler, “Trust me, I know what I’m doing.”
    Be warned, however, that you use unsafe Rust at your own risk: if you use unsafe code incorrectly,
    problems can occur due to memory unsafety, such as null pointer dereferencing.

    Another reason Rust has an unsafe alter ego is that the underlying computer hardware is inherently unsafe.
    If Rust didn’t let you do unsafe operations, you couldn’t do certain tasks. Rust needs to allow you to do
    low-level systems programming, such as directly interacting with the operating system or even writing your own
    operating system. Working with low-level systems programming is one of the goals of the language.
    Let’s explore what we can do with unsafe Rust and how to do it.
*/

use core::slice;

fn main() {
    let mut num = 5;

    /*
        In here example Rust allow us to create a row pointer,
        but disallow to dereference it without `unsafe` block,
        cause it can occure an null reference error.
    */
    let r1 = &num as *const i32; // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer
                                   // Dereference an raw pointer, e.g:
    unsafe {
        println!("r1 is: {}", *r1); // here we can do dereference of raw pointers
        println!("r2 is: {}", *r2);
    }

    /*
        Raw pointers are allowed to ignore Rust borrowing rules,
        by having mutable and immutable pointers or multiple immutable
        pointers to the same location in memory. Raw pointers are also
        not guarantee to point to a valid memory. Raw pointers are allowed
        to be a null. Finally, raw pointers don't implement any type of automatic
        cleanup.
    */

    // Calling unsafe function or method
    unsafe fn dangerous() {
        // body of unsafe function already in a unsafe block
    }

    unsafe {
        dangerous(); // we can call unsafe function only from unsafe block,
                     // otherwise, we got compile-time error
    }

    // Creating safe abstraction
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // We could also move this code to the normal function,
    // but we would use unsafe block to slice some data, e.g:
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            // We can call slice function only inside unsafe block,
            // because operations on raw pointers are unsafe
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    // check function
    // we do not need to specify unsafe block,
    // because function is normal, only one block inside it is unsafe
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]); // ok
    assert_eq!(b, &mut [4, 5, 6]); // ok

    /* Extern functions to call external code */
    extern "C" {
        // extern abs function from a C language
        fn abs(input: i32) -> i32;
    }

    unsafe {
        // We only can use external functions in a unsafe block,
        // because we don't now, valid or not this function it is
        println!("Absolute value pf -3 according to C: {}", abs(-3));
    }

    // We can also allow other languages to call our Rust
    // functions using extern keyword an a function signature, e.g:
    #[no_mangle] // not a mangle a name of function
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    /* Accessing or modyfing mutable static variable */

    /*
        Static variables are similar to constants, which we discussed in the
        “Differences Between Variables and Constants” section in Chapter 3.
        The names of static variables are in SCREAMING_SNAKE_CASE by convention.
        Static variables can only store references with the 'static lifetime,
        which means the Rust compiler can figure out the lifetime and we aren’t
        required to annotate it explicitly. Accessing an immutable static variable is safe.

        A subtle difference between constants and immutable static variables is that
        values in a static variable have a fixed address in memory. Using the value will
        always access the same data. Constants, on the other hand,
        are allowed to duplicate their data whenever they’re used.
        Another difference is that static variables can be mutable.
        Accessing and modifying mutable static variables is unsafe.
    */
    static HELLO_WORLD: &str = "Hello, World";

    println!("Name is: {}", HELLO_WORLD);

    // Example, e.g:
    static mut COUNTER: u32 = 0; // Declare a global static mutable variable

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);
    unsafe {
        println!("COUNTER is: {}", COUNTER);
    }

    /* Implementing an unsafe Traits */
    unsafe trait Foo {
        // methods go there
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}
