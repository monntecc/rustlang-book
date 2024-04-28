/*
    Message passing is a fine way of handling concurrency, but it’s not the only one.
    Another method would be for multiple threads to access the same shared data.
    Consider this part of the slogan from the Go language documentation again:
    “do not communicate by sharing memory.”

    What would communicating by sharing memory look like? In addition, why would
    message-passing enthusiasts caution not to use memory sharing?

    In a way, channels in any programming language are similar to single ownership,
    because once you transfer a value down a channel, you should no longer use that value.
    Shared memory concurrency is like multiple ownership: multiple threads can access the
    same memory location at the same time. As you saw in Chapter 15, where smart pointers
    made multiple ownership possible, multiple ownership can add complexity because these different
    owners need managing. Rust’s type system and ownership rules greatly assist in getting this management correct.
    For an example, let’s look at mutexes, one of the more common concurrency primitives for shared memory.
*/

#[allow(dead_code)]
#[allow(unused_variables)]
use std::sync::Mutex;
use std::{sync::Arc, thread};

fn main() {
    /*
        Mutexes

        Mutex is an abbreviation for mutual exclusion, as in, a mutex allows
        only one thread to access some data at any given time.
        To access the data in a mutex, a thread must first signal that it wants access
        by asking to acquire the mutex’s lock. The lock is a data structure that is part of
        the mutex that keeps track of who currently has exclusive access to the data.
        Therefore, the mutex is described as guarding the data it holds via the locking system.
    */
    let m = Mutex::new(5);

    {
        /* Access data in a mutex */
        let mut num = m.lock().unwrap();
        *num = 10; // Manipulate data using deref operator
        println!("Inner scope: {:?}", m); // data will be locked
    }

    println!("Main thread: {:?}", m); // data will be 10

    /* Another example with a threads */

    let counter = Arc::new(Mutex::new(0)); // allow a mutex to have a multi owners
                                           /*
                                               Arc smart pointer is the same as Rc, but allows us
                                               to share data between different threads
                                           */
    let mut handles = vec![];

    for _ in 0..10 {
        /* Spawn new thread to manipulate data in a mutex */
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // lock a mutex
            *num += 1; // change a data
        });

        handles.push(handle); // push to vector
    }

    for handle in handles {
        /* Wait threads to finish */
        handle.join().unwrap();
    }

    println!("Result is: {}", *counter.lock().unwrap()); // result is 10
}
