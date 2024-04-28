use core::fmt;
use std::ops::Add;

#[allow(dead_code)]
#[allow(unused_variables)]

/* Assosiated types */
trait MyIterator {
    type Item; // unknown type, will be only known after implement

    fn next(&mut self) -> Option<Self::Item>;
}

// e.g:
struct Counter {}

// Using assosiated types, we can only have one
// implementation of it, therefore generics can
// have multiple.
impl MyIterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

// Example with generics:
trait MyIterator2<T> {
    fn next(&mut self) -> Option<T>;
}
// for u32
impl MyIterator2<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}
// for i32
// here we can use another type, using assosicated type - no
impl MyIterator2<i32> for Counter {
    fn next(&mut self) -> Option<i32> {
        Some(0)
    }
}

/* Default Generic Type Parameters */
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Another example using two different types,
// e.g:
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

/* Calling methods with the same name */
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

/* Supertraits */
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} * ", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}. {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} * ", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

/* Newtype pattern */
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    // Checking adding a point struct
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // Check methods with the same name
    let person = Human;
    person.fly(); // will call method from a human
                  // If we want to call method from another trait,
                  // we could use next syntax, e.g:
    Pilot::fly(&person); // will call method from a pilot
    Wizard::fly(&person); // will call method from a wizard

    // Check outline print function
    let point = Point { x: 5, y: 3 };
    point.outline_print();

    // Check wrapper
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
