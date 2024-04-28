/*
    There is no consensus in the programming community about what features a
    language must have to be considered object-oriented. Rust is influenced
    by many programming paradigms, including OOP; for example, we explored the features
    that came from functional programming in Chapter 13. Arguably, OOP languages share certain
    common characteristics, namely objects, encapsulation, and inheritance.
    Let’s look at what each of those characteristics means and whether Rust supports it.
*/

#[allow(dead_code)]
#[allow(unused_variables)]

/* ENCAPSULATION */
/*
    Another aspect commonly associated with OOP is the idea of encapsulation,
    which means that the implementation details of an object aren’t accessible
    to code using that object. Therefore, the only way to interact with an object is t
    hrough its public API; code using the object shouldn’t be able to reach into the object’s
    internals and change data or behavior directly. This enables the programmer to change and
    refactor an object’s internals without needing to change the code that uses the object.
*/

pub struct AveragedCollection {
    list: Vec<i32>, // private
    average: f64,   // private
}

impl AveragedCollection {
    // add data to list + update average
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    // remove data from list + update average
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    // average getter
    pub fn average(&self) -> f64 {
        self.average
    }

    // private, update average data
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

/* INHERITANCE */
/*
    Inheritance is a mechanism whereby an object can inherit elements from another object’s definition,
    thus gaining the parent object’s data and behavior without you having to define them again.

    If a language must have inheritance to be an object-oriented language, then Rust is not one.
    There is no way to define a struct that inherits the parent struct’s fields and
    method implementations without using a macro.

    However, if you’re used to having inheritance in your programming toolbox,
    you can use other solutions in Rust, depending on your reason for reaching for
    inheritance in the first place.
*/

/* POLYMORPHISM */
/*
    To many people, polymorphism is synonymous with inheritance. But it’s actually a more
    general concept that refers to code that can work with data of multiple types.
    For inheritance, those types are generally subclasses.

    Rust instead uses generics to abstract over different possible types and trait bounds to
    impose constraints on what those types must provide.
    This is sometimes called bounded parametric polymorphism.
*/

fn main() {}
