/*
    One increasingly popular approach to ensuring safe concurrency is message passing,
    where threads or actors communicate by sending each other messages containing data.
    Here’s the idea in a slogan from the Go language documentation:
    “Do not communicate by sharing memory; instead, share memory by communicating.”

    To accomplish message-sending concurrency, Rust's standard library provides an implementation of channels.
    A channel is a general programming concept by which data is sent from one thread to another.

    You can imagine a channel in programming as being like a directional channel of water,
    such as a stream or a river. If you put something like a rubber duck into a river,
    it will travel downstream to the end of the waterway.

    A channel has two halves: a transmitter and a receiver. The transmitter half is the
    upstream location where you put rubber ducks into the river,
    and the receiver half is where the rubber duck ends up downstream.
    One part of your code calls methods on the transmitter with the data you want to send,
    and another part checks the receiving end for arriving messages.
    A channel is said to be closed if either the transmitter or receiver half is dropped.
*/

#[allow(dead_code)]
#[allow(unused_variables)]
use std::sync::mpsc;
use std::{thread, time::Duration};

fn main() {
    /* Create a channel */
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    /* Send message using thread */
    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("spawned"),
            String::from("thead!"),
        ];

        for msg in vals {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    /* Another thread to send messages */
    thread::spawn(move || {
        let vals = vec![
            String::from("More"),
            String::from("messages"),
            String::from("from"),
            String::from("another"),
            String::from("thead!"),
        ];

        for msg in vals {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    /* Receieve message from a channel */
    for recieved in rx {
        println!("Got: {}", recieved); // Will get messages from all threads
    }
}
