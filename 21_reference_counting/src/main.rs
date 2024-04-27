/*
    In the majority of cases, ownership is clear: you know exactly which variable owns a given value.
    However, there are cases when a single value might have multiple owners.
    For example, in graph data structures, multiple edges might point to the same node,
    and that node is conceptually owned by all of the edges that point to it.
    A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it and so has no owners.

    You have to enable multiple ownership explicitly by using the Rust type Rc<T>,
    which is an abbreviation for reference counting.
    The Rc<T> type keeps track of the number of references to a value to determine
    whether or not the value is still in use. If there are zero references to a value,
    the value can be cleaned up without any references becoming invalid.

    Imagine Rc<T> as a TV in a family room. When one person enters to watch TV,
    they turn it on. Others can come into the room and watch the TV.
    When the last person leaves the room, they turn off the TV because it’s no longer being used.
    If someone turns off the TV while others are still watching it,
    there would be uproar from the remaining TV watchers!

    We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program
    to read and we can’t determine at compile time which part will finish using the data last.
    If we knew which part would finish last, we could just make that part the data’s owner,
    and the normal ownership rules enforced at compile time would take effect.
*/

use std::rc::Rc;

#[allow(dead_code)]
#[allow(unused_variables)]
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    /*
         In next example we will have an error, because an a variable already
         been moved to another owner, e.g:

         let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
         let b = Cons(3, Box::new(a));
         let c = Cons(4, Box::new(a)); -- will generated error

         To fix this, we could use Rc<T> reference couting smart pointer, e.g:
    */
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // In this case, rc does not deep copying a list,
                                    // it is only increase the reference count
    let c = Cons(4, Rc::clone(&a));

    // Another example, e.g:
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a)); // 1
    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a)); // 2
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a)); // 3
    }
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
}
