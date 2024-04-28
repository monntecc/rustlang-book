/*
    The Rust type system has some features that we’ve so far mentioned but haven’t yet discussed.
    We’ll start by discussing newtypes in general as we examine why newtypes are useful as types.
    Then we’ll move on to type aliases, a feature similar to newtypes but with slightly different semantics.
    We’ll also discuss the ! type and dynamically sized types.
*/

#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    /* Type aliases */
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    /* Main case of type aliases */
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("Hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --snip--
        Box::new(|| {})
    }

    // Create type alias for it
    type Thunk = Box<dyn Fn() + Send + 'static>;

    // And after we can use it, e.g:
    let f: Thunk = Box::new(|| println!("Hi"));

    /* Never type */
    /*
        Rust has a special type named ! that’s known in type theory lingo as the empty
        type because it has no values. We prefer to call it the never type because it
        stands in the place of the return type when a function will never return.
        Here is an example:
    */
    fn bar() -> ! {
        print!("forever ");

        loop {
            print!(" and ever!");
        }
    }

    /* Dynamically sized types */
    let s1: &str = "Hello there!"; // we cant use str here, because it is unknown size at compile-time
    let s2: &str = "How's it going?";

    // Here will be no error, because compiler
    // automaticaly add Sized trait to it.
    fn generic<T>(t: T) {
        // --snip--
    }
    // Or you can add manualy Sized trait, and specify
    // that it may not be here
    /*
        fn generic2<T: ?Sized>(t: T) {
            // --snip--
        }

        But there will be an error, because if type
        does not implement an Sized trait,
        we have no idea about his size at compile-time
    */
}
