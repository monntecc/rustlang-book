/*
    A pointer is a general concept for a variable that contains an address in memory.
    This address refers to, or “points at,” some other data.
    The most common kind of pointer in Rust is a reference, which you learned about in Chapter 4.
    References are indicated by the & symbol and borrow the value they point to.
    They don’t have any special capabilities other than referring to data, and have no overhead.
*/

#[allow(dead_code)]
#[allow(unused_variables)]

/*
    Common use, we have recurisve enum, that have himself as parameter value.
    The Rust compiler doesn't know about size of this, so we can't pass the raw value.
    We could use Box<List> to fix that issue, e.g:

    Reference to the cons could be found here: https://en.wikipedia.org/wiki/Cons
*/
enum List {
    Cons(i32, Box<List>),
    Nil,
}

/*
    The Rust Programming Language will check amount of size
    to allocate, when you initialize a variable.
    In current case, variable thats assings a Message enum,
    will get the size of the max available size of the most
    big structure inside the enum (currently is a ChangeColor).
*/
enum Message {
    Quit,                       // Quit does need any space
    Move { x: i32, y: i32 },    // Move need space for two values
    Write(String),              // Write need also space for a string
    ChangeColor(i32, i32, i32), // ChangeColor also for a tuple of integers
}

use List::{Cons, Nil};

fn main() {
    /*
        Starting with Box<T> smart pointer.
        It is allow us to allocate memory on the heap.
        e.g:

        At this example, we're specify, that we want to store `5` as value
        on the heap, and memory address to this value on the stack
    */
    let b = Box::new(5);
    println!("b = {}", b);

    /*
        Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack.
        But they don’t have many extra capabilities either.
        You’ll use them most often in these situations:

        1. When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size.
        2. When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so.
        3. When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type.
    */

    // Create a list using a cons
    let _ = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
