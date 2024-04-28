// Library crate

// Define a structure representing a Post, which can have different states and content.
pub struct Post {
    state: Option<Box<dyn State>>, // Store the current state of the Post.
    content: String,               // Store the content of the Post.
}

impl Post {
    // Create a new Post with initial state set to Draft and empty content.
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})), // Initial state set to Draft.
            content: String::new(),          // Empty content initially.
        }
    }

    // Add text to the content of the Post.
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Get the content of the Post.
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self) // Get content based on the current state.
    }

    // Transition the Post to the PendingReview state.
    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    // Transition the Post to the Published state.
    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}

// Define a trait representing the state of a Post.
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>; // Request review transition.
    fn approve(self: Box<Self>) -> Box<dyn State>; // Approve transition.
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        // Get content (default implementation).
        ""
    }
}

// Define the Draft state.
struct Draft {}

impl State for Draft {
    // Transition from Draft to PendingReview state.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // No transition from Draft to other states on approval.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// Define the PendingReview state.
struct PendingReview {}

impl State for PendingReview {
    // No transition from PendingReview to other states on request.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Transition from PendingReview to Published state on approval.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// Define the Published state.
struct Published {}

impl State for Published {
    // No transition from Published to other states on request.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // No transition from Published to other states on approval.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Get content of the Post (implementation for Published state).
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
