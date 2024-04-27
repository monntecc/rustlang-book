/*
    Rust’s memory safety guarantees make it difficult, but not impossible,
    to accidentally create memory that is never cleaned up (known as a memory leak).
    Preventing memory leaks entirely is not one of Rust’s guarantees, meaning memory leaks are memory safe in Rust.
    We can see that Rust allows memory leaks by using Rc<T> and RefCell<T>:
    it’s possible to create references where items refer to each other in a cycle.
    This creates memory leaks because the reference count of each item in
    the cycle will never reach 0, and the values will never be dropped.
*/

use crate::List::{Cons, Nil};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

/* CREATING A TREE DATA STRUCTURES */
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // Creation of reference cycles
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("A initial rc count = {}", Rc::strong_count(&a));
    println!("A next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("A rc count after b creation = {}", Rc::strong_count(&a));
    println!("B initial rc count = {}", Rc::strong_count(&b));
    println!("B next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("B rc count after changing a = {}", Rc::strong_count(&b));
    println!("A rc count after changing a = {}", Rc::strong_count(&a));

    // Tree structure implemetation:
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "Leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // Number of references which have ownership of the data
        Rc::weak_count(&leaf)    // Number of reference which have not ownership of the data
    );

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
        "Branch strong = {}, weak = {}",
        Rc::strong_count(&branch), // Number of references which have ownership of the data
        Rc::weak_count(&branch)    // Number of reference which have not ownership of the data
    );

    println!(
        "Leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // Number of references which have ownership of the data
        Rc::weak_count(&leaf)    // Number of reference which have not ownership of the data
    );

    println!(
        "Leaf parent after change = {:?}",
        leaf.parent.borrow().upgrade()
    );
}
