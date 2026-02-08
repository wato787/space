use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..3 {
            println!("thread: {i}");
        }
    });

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("hi from thread");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("got: {received}");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap());
}
