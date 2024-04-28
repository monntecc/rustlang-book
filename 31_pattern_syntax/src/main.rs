fn main() {
    /* Matching literals */
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    /* Matching named variables */
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    /* Multiple patters */
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    /* Matching ranges or values */
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII latter"),
        _ => println!("something else"),
    }

    /* Destructing to break apart values */
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Or another example
    match p {
        Point { x, y: 0 } => {
            println!("On the x axis at {}", x);
        }
        Point { x: 0, y } => {
            println!("On the y axis at {}", y);
        }
        Point { x, y } => {
            println!("On neither axios: ({}, {})", x, y);
        }
    }

    // Or another example
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit");
        }
        Message::Move { x, y } => {
            println!("Moved to ({}, {})", x, y)
        }
        Message::Write(message) => {
            println!("Message writte: {}", message);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Color changed to R: {}, G: {}, B: {}", r, g, b);
        }
    }

    /* Ignoring values in a pattern */
    foo(3, 4);

    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    // Or another example
    let mut settings_value = Some(5);
    let new_settings_value = Some(10);

    match (settings_value, new_settings_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            settings_value = new_settings_value;
        }
    }

    println!("Settings is: {:?}", settings_value);

    // Or another example
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    /* Match guards */
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // Or another example
    let x = Some(5);
    let y = 10;

    match x {
        Some(n) if n == y => println!("Mathed, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    // Or another example
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("Yes"),
        _ => println!("No"),
    }

    /* Bindings */
    enum Message_2 {
        Hello { id: i32 },
    }

    let msg = Message_2::Hello { id: 5 };

    match msg {
        Message_2::Hello {
            id: id_variable @ 3..=7, // bind operator
        } => println!("Found an id in range: {}", id_variable),
        Message_2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message_2::Hello { id } => println!("Found some other id: {}", id),
    }
}
