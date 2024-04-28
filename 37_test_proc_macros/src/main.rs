use procedural_macros::HelloMacro;
use procedural_macros_derive::HelloMacro;

#[derive(HelloMacro)]
struct Human;

fn main() {
    Human::hello_macro(); // Hello, Macro! My name is Human
}
