/*
    Implementing the Deref trait allows you to customize the behavior of the dereference
    operator * (not to be confused with the multiplication or glob operator).
    By implementing Deref in such a way that a smart pointer can be treated like a regular reference,
    you can write code that operates on references and use that code with smart pointers too.
*/

#[allow(dead_code)]
#[allow(unused_variables)]
use std::ops::Deref;

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    /*
        What actually derefence do? This will go to memory address,
        stored at variable y, so it was memory address for variable x,
        and will go to the value of x and return it, in our case = 5;
    */
    assert_eq!(5, *y);

    // Or we could use a Box<T> smart pointer, e.g:
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // And we will get the same result

    // Or our own smart pointer, e.g:
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // And we will get the same result

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // Working, therefore we are pass a &MyBox<String> reference,
               // but function waiting for a &str reference.
               // It is working, because we are implemented a Deref trait,
               // which is return in our case from &MyBox<String> -> &String.
               // String also implements a Deref trait, so we will get from &<String> -> &str,
               // exactly what function is expected.
               /*
                    If Rust won't to do auto deref coercion, we needed to do in our case,
                    something like that, e.g:

                    hello(&(*m)[..]);

                    Firstly we are derefence an m to a String,
                    after that we get slices from a String and convert
                    type of it to str, and after we are get reference to it -> &str
               */

    /*
        Rust does deref coercion when it finds types and trait implementations in three cases:

        1. From &T to &U when T: Deref<Target=U>
        2. From &mut T to &mut U when T: DerefMut<Target=U>
        3. From &mut T to &U when T: Deref<Target=U>
    */
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

/* Create the own smart pointer */
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implement a Deref trait for our smart pointer
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
