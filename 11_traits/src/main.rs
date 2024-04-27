/*
    A trait defines functionality a particular type has and can share with other types.
    We can use traits to define shared behavior in an abstract way.
    We can use trait bounds to specify that a generic type can be any type that has certain behavior.
*/

use std::fmt::Display;

/*
    Below we have defined two structs for article declarations.
    If we want to implement those structs, they we will have some the same functions.
    To not duplicate a functionality, we can use a trait, which helps us with this:
*/
pub trait Summary {
    // Traits only can contains shared methods:
    // fn summarize(&self) -> String;
    // Or we can define a default body:
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    // Another function
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

// Implementing Summary functions for NewsArticle struct
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    // fn summarize(&self) -> String {
    //     format!("{} by {}", self.headline, self.author)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implementing Summary functions for Tweet struct
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Also, we can use traits as function parameters,
// to do this, just type `impl {trait_name}` e.g:
/*
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
*/
// Above example is a synax for a `trait bound`, what actually look like:
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/*
    Trait bounds are powerful when we hove a more parameters, e.g:

    pub fn notify<T: Summary>(item1: &T, item2: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    using impl syntax:

    pub fn notify(item1: &impl Summary, item2: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    Also, if we want to use multiply case as type, using impl syntax is complicated:
    pub fn notify(item1: &(impl Summary + Display), item2: &(impl Summary + Display)) {
        println!("Breaking news! {}", item.summarize());
    }

    using trait bound:

    pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {
        println!("Breaking news! {}", item.summarize());
    }
*/

/*
    If we have a lot of traits for implement the type, it will be a little bit stranger to read,
    so we can use another form of defining traits, where syntax, e.g:
*/
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Drop,
{
    1
}

/*
    Traits also can be used for a returns types, e.g:
*/
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

/*
    Using trait bounds to conditionally implement struct methods:
*/
struct Pair<T> {
    x: T,
    y: T,
}

// Implementation for any types
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Implementation for debug display and partial ordering types (>, <, =) etc.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/*
    Blanket implementation is used, when we want to implement traits, for types,
    which implement another traits, e.g:
    implementing trait `ToString` for type T which implement trait Display:

    impl<T: Display> ToString for T {
        fn to_string(&self) -> String {
            --snip--
        }
    }
*/

fn main() {
    // Showcase of above traits implementation
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling!"),
    };

    println!("Tweet summary = {}", tweet.summarize());
    println!("Article summary = {}", article.summarize()); // There will be used a default body from a trait

    // Call notify function
    notify(&article);

    // Call summarize function from returns_summarizable one
    println!("{}", returns_summarizable().summarize());
}
