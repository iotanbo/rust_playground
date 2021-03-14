
// https://doc.rust-lang.org/book/ch16-00-concurrency.html

// * Green threading: when threads are language-provided (not system-provided)
// * Rust uses system-provided threads, although there are crates that provide green threading


// * channels is rust's build-in implementation of actors
// * technically speaking, this is just a multi-producer, single consumer queue
//   with a regular thread



use std::thread;
use std::time::Duration;

// Multi-producer, single consumer blocking queue
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

use std::sync::{Arc, Mutex};


fn basic_concurrency_demo() {

    // Create single thread and execute immediately
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("hi number {} from the SPAWNED thread!", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..=3 {
        println!("hi number {} from the MAIN thread!", i);
        thread::sleep(Duration::from_millis(100));
    }

    handle.join().unwrap();

}

fn channels_demo() {

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        println!(" * Another thread is running, prepares to sleep for 1 sec.");
        tx.send("hi there!".to_owned()).unwrap();
        thread::sleep(Duration::from_millis(1000));
        tx.send("exiting!".to_owned()).unwrap();
        println!(" * Another thread exits.");
    });

    let quit_msg = "exiting!".to_owned();
    loop {
        let msg = rx.recv().unwrap();  // this will block while waiting for a new message
        println!("* Received message from another thread: '{:?}'", msg);
        if msg == quit_msg { break; }
    }
    
}


fn mutex_demo() {

    let counter = Arc::new(Mutex::new(0u32));

    let mut adder_handles = vec![];
    let mut subtractor_handles = vec![];

    // Create 5 threads that subtract 1 from the counter
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            //*num -= 1;
            *num = num.overflowing_sub(1).0;
        });
        subtractor_handles.push(handle);
    }

    // Create 5 threads that add 2 to the counter
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            //*num += 2;
            *num = num.overflowing_add(2).0;
        });
        adder_handles.push(handle);
    }

    // Join adder threads
    for handle in adder_handles {
        handle.join().unwrap();
    }

    // Join subtractor threads
    for handle in subtractor_handles {
        handle.join().unwrap();
    }

    // Verify counter
    let counter_val = *counter.lock().unwrap();
    println!("Result: {}", counter_val);
    assert_eq!(counter_val, 5);

}


pub fn demo() {

    println!("== concurrency demo begin ==");


    basic_concurrency_demo();
    channels_demo();
    mutex_demo();

    println!("== concurrency demo end ==");
    println!();
}
