// Library crate

#[allow(dead_code)]
#[allow(unused_variables)]

/* Trait for common behaviour */
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // Box smart pointer, fix any data which implmenet Draw trait
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/*
  Above implementation is different aproch of trait bounds.
  Trait bounds, when we use a generics and specify with `where` a
  type to, may only have one type of it. So, if we want for example
  mixed up with buttons, sliders, images we cannot do this using
  trait bounds, but can using trait objects.
*/

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // draw button
        println!("Draw button!");
    }
}
