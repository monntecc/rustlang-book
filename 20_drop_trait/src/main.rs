/*
    The second trait important to the smart pointer pattern is Drop,
    which lets you customize what happens when a value is about to go out of scope.
    You can provide an implementation for the Drop trait on any type,
    and that code can be used to release resources like files or network connections.

    We’re introducing Drop in the context of smart pointers because the functionality
    of the Drop trait is almost always used when implementing a smart pointer.
    For example, when a Box<T> is dropped it will deallocate the space on the heap that the box points to.
*/

// Example of using Drop trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // first example
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointer created!");

    // another example
    let cc = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created! x2");
    // cc.drop(); - we can't do this, otherwise we need to call drop(c); function, e.g:
    drop(cc);
    println!("CustomSmartPointer dropped before the end of main.!");
}
