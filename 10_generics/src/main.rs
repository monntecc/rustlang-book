#[allow(dead_code)]
#[allow(unused_variables)]

// Create struct with a generic value
struct Point<T, U> {
    x: T,
    y: U,
}

// Implemetation for the structs could also be a generic value,
// in current context, we will have implementation for any types, e.g:
impl<T, U> Point<T, U> {
    pub fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }

    pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
// If we need to do implementation for specific type,
// we could do like this, e.g:
impl Point<i32, i32> {
    pub fn sub(&self) -> i32 {
        &self.x * &self.y
    }
}

// We also could use generic types is enum definitions:
enum OurOption<T> {
    Some(T),
    None,
}

fn main() {
    /* GENERIC TYPES */

    // Example code of finding the largest number in a vector
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is = {}", largest);

    // What is we would like do the same, but using another vector?
    // So we need to copy the same functionality, but change the vector contents
    let number_list = vec![102, 34, 6000, 90, 54, 2, 10, 15];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is = {}", largest);

    // To prevent this we could extract functionality to other function
    let number_list = vec![25, 90, 5, 722, 12, 445, 12];
    let largest = get_largest(number_list);
    println!("The largest number is = {}", largest);

    // But will we do, when we are what to do the same logic, but for different type?
    // Create the new function? Noooo, we have a generic types!, e.g:
    let char_list = vec!['w', 'x', 's', 'j'];
    let largest = get_largest_2(char_list);
    println!("The largest character is = {}", largest);

    // We also could use generics with structs, e.g:
    let p1 = Point { x: 10.5, y: 10.5 };
    let p2 = Point { x: 2, y: 7 };
    let p2 = Point { x: 'a', y: 'b' };
    // Or with two different types:
    let p2 = Point { x: 'a', y: 10 };
    let p3 = Point::new(10, 20.0); // or using implementation function
                                   // We also can you specific type functions:
    let p4 = Point::new(10, 10);
    let sub = p4.sub(); // 10 * 10 = 100
                        // Mixup points using implementation function
    let mixed = p4.mixup(p3);
    println!("mixed.x = {}, mixed.y = {}", mixed.x, mixed.y);
}

// Function to find the largest from a vec
fn get_largest(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// Generic function, that accepts any value to compare with,
// and using the traits `PartialOrd` and `Copy` to indicate,
// that our generic could be comparable
fn get_largest_2<T: PartialOrd + Copy>(vector: Vec<T>) -> T {
    let mut largest = vector[0];

    for item in vector {
        if item > largest {
            largest = item;
        }
    }
    largest
}
