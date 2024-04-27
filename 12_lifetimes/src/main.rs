#[allow(dead_code)]
#[allow(unused_variables)]
use std::fmt::Display;

// Lifetime annotations in struct definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}
/*
    TIPS
    1. Each parameter taht is a reference gets its own lifetime parameter

    2. If there is exactly one input lifetime parameter, that lifetime is
    assigned to all output lifetime parameters

    3. If there are multiple input lifetime parameters, but one of them is
    &self or &mut self, the lifetime of self is assigned to all output
    lifetime parameters.
*/
impl<'a> ImportantExcerpt<'a> {
    // So, to not break 3 rule, we mark explicitly lifetime annotations
    fn return_part(&'a self, announcement: &str) -> &'a str {
        println!("Attension please: {}!", announcement);
        &self.part
    }
}

fn main() {
    /*
        Dangeling references - that is reference which points
        to invalid data. e.g:

        {
            let r: &i32;            -------+ 'a
                                              |
            {                                 |
                let x = 5;          -+  'b    |
                r = &x;             |         |
            }                       -+  'b    |
                                              |
            println!("r: {}", r); -- error    |
                                   -------+  'a
        }

        This will call an error, because lifetime of this variable
        will be out of scope for the second.

        Lifetime annotations will help borrow cheker to understand,
        what lifetime is valid and what is not.

        Lifetime annotations will used only for references.
    */

    // First example of lifetimes
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    // There, lifetimes of string1 and string2 are the same
    // so longest function has no difference to choose between lifetimes
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is x1 = {}", result);

    // Second example of lifetimes
    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");
        // There, lifetime of string2 will be out of scope after printing to the console,
        // but it will be valid on the printing line, so also no difference
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is x2 = {}", result);
    }

    /*
        In this case we will have an error:

        let string1 = String::from("abcd");
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
        }
        println!("The longest string is x2 = {}", result);

        Because when result was printed, string2 variable already out of scope,
        and we have not access to it
    */

    // Some example of struct lifetime
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a sentence");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    /*
        Also Rust has an `'static` lifetime.
        It means, that this reference could leave as long as program
        is running. By default all of the string literals have a 'static lifetime.
        It is because string literals stores on the programs binary, and we have
        access to it every time at program duration;
    */
    let s: &'static str = "I have a static lifetime!";

    let str1 = "hello";
    let str2 = "world";
    let ann = "AHAHAHAHHA";
    let longest = longest_with_an_announcement(&str1, &str2, &ann);
    println!("Longest from two words = {}", longest);
}

/*
    In this function, we will use generic lifetime annotation,
    which is describes the relationships between the lifetimes of multiple references,
    and how to relate to each other, e.g:

    TIP:
    - All of the generic lifetime annotations starts with ' (apostrophie) or a (tick),
    followed by the name of lifetime

    &i32            a reference
    &'a i32         a reference with an explicit lifetime
    &'a mut i32     a mutable reference with an explicit lifetime

    REMEMBER
    - Explicit lifetime annotations does not set new lifetime,
    they are only describes the relationships between the lifetimes of multiple references.

    In our case we say:
    - x and y parameters such as a return type will be on the same lifetime.
    - if x lifetime is greater than y, return type will accept the x lifetyme
    - the same for y
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/*
    Function takes a reference for the string slice,
    and returns the first word.
*/
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/*
    Since this is final part of this chapter, we would combine
    all of the functionality (generics, traits and lifetimes) together,
    by passing it into one function, e.g:
*/
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
