/*
    Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references
    to that data; normally, this action is disallowed by the borrowing rules. To mutate data, the pattern uses unsafe code
    inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.
    Unsafe code indicates to the compiler that we’re checking the rules manually instead of relying on the compiler to check them for us;
    we will discuss unsafe code more in Chapter 19.

    We can use types that use the interior mutability pattern only when we can ensure that the borrowing
    rules will be followed at runtime, even though the compiler can’t guarantee that.
    The unsafe code involved is then wrapped in a safe API, and the outer type is still immutable.
*/

/*
    Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error
    if you try using it in a multithreaded context. We’ll talk about how to get the functionality of
    RefCell<T> in a multithreaded program in Chapter 16.

    Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

    1. Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
    2. Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows
    checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
    3. Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside
    the RefCell<T> even when the RefCell<T> is immutable.
    4. Mutating the value inside an immutable value is the interior mutability pattern.

    Let’s look at a situation in which interior mutability is useful and examine how it’s possible.
*/

#[allow(dead_code)]
#[allow(unused_variables)]
use std::cell::RefCell;
use std::rc::Rc;

/* IMPLEMENT USE CASE OF INTERIOR MUTABILITY PATTERN */
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// Implement the tests for above structures
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // self.sent_messages.push(String::from(msg)); -- we cant do like this, because we need mutable reference
            // And now we can fix this using a RefCell, e.g:
            self.sent_messages.borrow_mut().push(String::from(msg)); // we are borrow a mutable reference, and then push the value
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

/* COMBINING RC AND REFCELL, e.g: */
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    /*

    Next code will show the common use errors, when trying to borrow a value,
    e.g:

    let a = 5;
    let b = &mut a; -- error, we cannot borrow 'a' as mutable, when is immutable

    let mut c = 10;
    let d = &c;
    *d = 20; -- error, we cannot rewrite data on immutable reference

    */

    // We also can combine Rc and RefCell, e.g:
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
