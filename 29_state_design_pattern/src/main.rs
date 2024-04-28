/*
    The state pattern is an object-oriented design pattern. The crux of the pattern is that we define a set of
    states a value can have internally. The states are represented by a set of state objects, and the value’s behavior
    changes based on its state. We’re going to work through an example of a blog post struct that has a field to
    hold its state, which will be a state object from the set "draft", "review", or "published".

    The state objects share functionality: in Rust, of course, we use structs and traits rather than objects and
    inheritance. Each state object is responsible for its own behavior and for governing when it should change into
    another state. The value that holds a state object knows nothing about the different behavior of
    the states or when to transition between states.

    The advantage of using the state pattern is that, when the business requirements of the program change,
    we won’t need to change the code of the value holding the state or the code that uses the value.
    We’ll only need to update the code inside one of the state objects to change its rules or
    perhaps add more state objects.
*/

#[allow(dead_code)]
#[allow(unused_variables)]
use state_design_pattern::Post;

fn main() {
    /* Create a post */
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today!");
    assert_eq!("", post.content()); // No text, we are only added text

    post.request_review();
    assert_eq!("", post.content()); // Not text, we are requesting a review

    post.approve();
    assert_eq!("I ate a salad for lunch today!", post.content()); // Text is approved
}
