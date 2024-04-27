#[allow(dead_code)]
#[allow(unused_variables)]

/*
    Rust has an module system, which allows to separate some parts of code.
    To define a module, we need to specify `mod` keyword, and name of module.

    Modules can contain another modules inside them, which also can contains e.g: structs, enums, etc.

    Simple module structure is here:

    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}

            fn seat_at_table() {}
        }

        mod serving {
            fn take_order() {}

            fn server_order() {}

            fn take_payment() {}
        }
    }
*/

/*
    To get access for any fuction in another module, we need to specify a path to it.
    Path could be absolute (from crate module), or relative, e.g:

    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaraunt() {
        - Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        - Relative path
        front_of_house::hosting::add_to_waitlist();
    }

    Above example wouldn't work, `because add_to_waitlist` function has no public modifier,
    and we cannot access it from another module.

    For default every child in module is private, so we can't get information from it,
    without specifing, that module must be a public (pub keyword), for example:
*/

// mod front_of_house {
//     pub mod hosting {
//         // We are specify, that current module is public
//         pub fn add_to_waitlist() {} // The same for the function
//     }
// }

pub fn eat_at_restaraunt() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // And woula, we have access to it

    // Relative path
    front_of_house::hosting::add_to_waitlist(); // From here also
}

// Another example using `super` keyword to access modules from relative path:
fn server_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order(); // We able to call cook_order, because it is defined in the same module
        super::server_order(); // And also we could call this one, using `super` keyword, which allows us to access parent module
    }

    fn cook_order() {}
}

/*
    Not lets talk about privary rules when it comes to structs using another example:

    mod back_of_house_2 {
        struct Breakfest {
            toast: String,
            seasonal_fruit: String,
        }

        impl Breakfest {
            fn summer(toast: &str) -> Breakfest {
                Breakfest {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }

    pub fn eat_at_restaraunt_2() {
        let mut meal = back_of_house_2::Breakfest::sumer("meal");
    }

    In this way we also will get an error, because Breakfest struct is private by default, as the implementation
    method for it `summer`, so we can't access it from another module like this.

    To fix that we could use:
*/
mod back_of_house_2 {
    // add pub keyword here
    pub struct Breakfest {
        pub toast: String, // Fields in this case also was private, it we want to public it, just add to each of them `pub` keyword
        seasonal_fruit: String,
    }

    impl Breakfest {
        // add pub keyword here
        pub fn summer(toast: &str) -> Breakfest {
            Breakfest {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaraunt_2() {
    let mut meal = back_of_house_2::Breakfest::summer("Hamburger"); // It will be OK
    meal.toast = String::from("Wheat"); // Also will be ok, because `toast` field is a public

    /*
       let meal2 = back_of_house_2::Breakfest {
           toast: String::from("wheat"),
           seasonal_fruit: String::from("peaches"), // this will generate an error, because seasonal_fruit is a private
       };
    */
}

mod back_of_house_3 {
    // If we are mark enum as public, all fields inside also will be public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaraunt_3() {
    let order1 = back_of_house_3::Appetizer::Soup; // We can access, because enum is a pulic
    let order2 = back_of_house_3::Appetizer::Salad; // The same thing
}

// Specifing full path to the our module is a little bit annoying,
// so in Rust we have something like `use`, which allow us to shortinize a i module calling, e.g:
mod front_of_house_2 {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// we can `import` something from another module, using `use` keyword
// Difference between `self` and `crate` is next:
/*
    self:: references current module
    crate:: references main create module
*/
use self::front_of_house_2::hosting;

pub fn eat_at_restaraunt_4() {
    front_of_house_2::hosting::add_to_waitlist(); // this is a little bit annoying
    hosting::add_to_waitlist(); // this is ok
}

/*
    Something different modules could have a fields, functions or structs with the same name.
    To prevent conflicts when importing something, we can use parent module to access this type, e.g:

    use std::io;
    use std::fmt;

    fn function1() -> io::Result<()> {}
    fn function2() -> fmt::Result {}

    Or another example, with renaming some of structure, e.g:
*/
use std::fmt::Result;
use std::io::Result as IoResult;

fn funtion1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}

/*
    If we want, to external code will be able to call some function from current scope and module,
    we need to specify `pub` keyword before importing `use`, e.g:

    pub use crate::front_of_house_2::hosting;
*/

/*
    Rust also have nested paths, so if we need to import more than 1 item from some module,
    we could use something like that:

    use rand::{Rng, CryptoRng, ErrorKind::Transient};

    instead of:

    use rand::Rng;
    use rand::CryptoRng;
    use rand::ErrorKind::Transient;

    Also we could import all public items from module, e.g:

    use rand::*;

    Or import one item with access to parent module, e.g:

    use rand::{self, Rng}; -- We could use Rng without specifing before rand, but also we could use rand to access another items
*/

/*
    Modules could be separated by different files, what i have done here.
    And after this we also will be able to call front_of_house, e.g:
*/
mod front_of_house; // Specify that we will use this module
use front_of_house::hosting::add_to_waitlist; // And import something from it
