#[allow(dead_code)]
#[allow(unused_variables)]
// Define an simple enum
/* enum IpAddrKind {
        V4,
        V6,
    }
*/
// Or we can create an enum with already storing data in that, e.g:
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Create message enum, that contains for different fields with another types.
// We also could create 4 different types for it, but it will be another types.
// Whem we create fields like this, all will be on another type, but in one main type (enum Message),
// which is very useful
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Annonymous struct
    Write(String),              // Single string
    ChangeColor(i32, i32, i32), // Three integers
}

// Just like in structs, we also could create assocciated functions or methods on enums,
// e.g:
impl Message {
    fn some_function() {
        println!("Hello from Message enum!");
    }
}

// Create US States enum
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    // ...
}

// Create Coin enum
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // For simple enum option we can you this syntax:
    /*
        let localhost = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("localhost"),
        };
    */
    // For extended one with data annotation, we use this:
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    println!("localhost ip struct: {:#?}", localhost);

    Message::some_function(); // call assocciated function from Message enum

    /* OPTIONAL ENUMS */
    // Rust has no null values, so without them we have `Option` enum.
    // This type of enum is generic, so can has any value.
    // Enum does include only two values, Some<(T)> or None, where Some is the value or None is the no value there
    // Practical way to use it, e.g:
    let some_number = Some(5); // This variable has value of 5
    let some_string = Some("a string"); // This a string of `a string`
    let absent_number: Option<i32> = None; // This contains no value

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // we will have an error here, because it is different types
    // But we could do this:
    let sum = x + y.unwrap_or(0); // We unwrap the value from the option enum, and if it is none, pass 0 as default value

    // Using `match` with option enum
    match y {
        Some(value) => println!("y value is = {}", value),
        None => println!("y value is none!!!!!"),
    }

    // Get cents from the value using enum declaration
    let cents = value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5); // value is 5
    let six = plus_one(five); // value is 6
    let none = plus_one(None); // value is none

    let some_value: Option<i32> = Some(3);
    match some_value {
        Some(_) => println!("Got 3!!"),
        _ => (), // we can also use `_`, it is shortcut for `and for the latest options i will do...`
    }

    // Option enum can be used also in the `if` statement,
    // which is different from `match` expression, because we can do whatever we want,
    // without handling all options at once
    // e.g:
    if let Some(3) = some_value {
        println!("Got 3 from let expression!");
    }
}

// Function to return value in cents from different states,
// based on passed coin and the state
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Function that will add `1` value to exist number,
// if number will be present (using option enum)
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
