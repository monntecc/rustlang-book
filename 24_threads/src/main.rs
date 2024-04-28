/*
    Handling concurrent programming safely and efficiently is another of Rust’s major goals.
    Concurrent programming, where different parts of a program execute independently,
    and parallel programming, where different parts of a program execute at the same time,
    are becoming increasingly important as more computers take advantage of their multiple processors.
    Historically, programming in these contexts has been difficult and error prone: Rust hopes to change that.
*/

/*
    In most current operating systems, an executed program’s code is run in a process,
    and the operating system will manage multiple processes at once.
    Within a program, you can also have independent parts that run simultaneously.
    The features that run these independent parts are called threads.
    For example, a web server could have multiple threads so that it could
    respond to more than one request at the same time.

    Splitting the computation in your program into multiple threads to run
    multiple tasks at the same time can improve performance, but it also adds complexity.
    Because threads can run simultaneously, there’s no inherent guarantee about the order in which
    parts of your code on different threads will run. This can lead to problems, such as:

    1. Race conditions, where threads are accessing data or resources in an inconsistent order
    2. Deadlocks, where two threads are waiting for each other, preventing both threads from continuing
    3. Bugs that happen only in certain situations and are hard to reproduce and fix reliably
    4. Rust attempts to mitigate the negative effects of using threads, but programming in a
    multithreaded context still takes careful thought and requires a code structure that is different
    from that in programs running in a single thread.
*/

#[allow(dead_code)]
#[allow(unused_variables)]
use std::thread;
use std::time::Duration;

fn main() {
    /* Create a new thread */
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi, number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    /*
        Join will tell, that we suspend current thread, and allow
        to `handle` thread in our case to finish their work.
    */
    handle.join().unwrap();

    /* Do the same, but on main thread */
    for i in 1..5 {
        println!("Hi, number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // We need to pass `move` keyword here,
        // because thread need to move ownership from v
        println!("Here's a vector: {:?}", v);
    });

    /*
        Join will tell, that we suspend current thread, and allow
        to `handle` thread in our case to finish their work.
    */
    handle.join().unwrap();
}
