#[allow(dead_code)]
#[allow(unused_variables)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to check if our rect can hold in itself another one
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100")
        }

        Guess { value }
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    // Import all data from parent module (crate one)
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // Will success
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_holf_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // Will success
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        // Must to success, 4 = 4
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_two_not_equal() {
        // May fail, because 4 = 4
        assert_ne!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Maya");
        // We also can add custom error messages
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, values was: `{}`",
            result
        );
    }

    #[test]
    // We could specify a `should_panic` attribute
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    // We could also add a expected message to it
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_2() {
        Guess::new(200);
    }

    /*
        Tests also could have an result type,
        which allows as to use `?` mark inside the function body.
    */
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 5 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two does not equal four!"))
        }
    }
}
