#[allow(dead_code)]
#[allow(unused_variables)]

// Define a struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// We could define a structs as tuple, e.g:
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Create a new rectangle struct
#[derive(Debug)] // Add support to print the struct using println marco
struct Rectangle {
    width: u32,
    height: u32,
}

// In Rust we also have a methods, which can be created on the struct object using implementation,
// e.g:
impl Rectangle {
    // self is the current instance of the object.
    // if self is defined in method arguments, this is indicated that method can be only called from the instance,
    // if self is not inluded, we could call the method statically
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to check if our rect can hold in itself another one
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// We also could create an assosiated functions (static methods), which can be called without instance.
// We could create themselfs on previous struct implementation, but Rust allowed to use multiple, so we can do this
impl Rectangle {
    // Create a square
    // If we are creating static methods, we not pass the `self` to the arguments
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Create an instance of the struct
    let mut user1 = User {
        email: String::from("user1@email.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 3,
    };

    // We can access the values from the struct by default, using dot notation, e.g:
    let name = user1.username;
    println!("user1 name = {}", name);
    user1.username = String::from("user1xd"); // change a username value (remember, instance of struct must be a mutable)

    // Create another user using our function
    let user2 = build_user(String::from("user2"), String::from("user2@email.com"));
    println!(
        "user2: email = {}, name = {}, active = {}, sign_in_count = {}",
        user2.email, user2.username, user2.active, user2.sign_in_count
    );

    // Also we can copy some fields from another struct to new one, e.g:
    let user3 = User {
        email: String::from("user3@email.com"),
        username: String::from("user3"),
        ..user2 // active and sign_in_count fields are copied from user2
    };
    println!(
        "user3: email = {}, name = {}, active = {}, sign_in_count = {}",
        user3.email, user3.username, user3.active, user3.sign_in_count
    );

    // Create an rectangle instance
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect: {:#?}", rect); // print the rect struct
    println!("Calculated area of rect (function) = {}", area(&rect)); // print the calculated area of the rectangle

    // We also can use implementation method of the rectangle struct
    println!("Calculated area of rect (method) = {}", &rect.area()); // the same result

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("rect can hold rect2? = {}", &rect.can_hold(&rect2)); // call another method

    let square = Rectangle::square(25); // create a square, using assosiated function (static method) in a rectangle struct
    println!("square = {:#?}", square);
}

// Function to creates a user
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}

// Function to calculate rectangle area
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
