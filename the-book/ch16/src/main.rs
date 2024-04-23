// Taken from Chapter 16 of The Rust Programming Language
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    // basic concurency via thread spawning
    let x0 = vec![42, 42, 42];
    let x1 = x0.clone(); // need to clone as closure will take ownership for safety
    let dt = Duration::from_secs_f32(0.25); // implements copy trait so easier to move around
    let process0 = thread::spawn(move || {
        for n in 0..10 {
            println!("Call {n} from spawned thread, len(x0) = {}", x0.len());
            thread::sleep(dt);
        }
    });
    
    for n in 0..10 {
        println!("Call {n} from main thread, len(x1) = {}", x1.len());
        thread::sleep(dt);
    }

    process0.join().unwrap();

    // transferring data between threads with message passing
    let (tx0, rx) = mpsc::channel();
    let tx1 = tx0.clone(); // need to clone here before tx0 goes out of scope

    thread::spawn(move || {
        let signal = vec![0.1, 0.3, 0.9, 1.1, 1.0, 0.55];
        for e in signal {
            let msg = Message::new(0, e);
            tx0.send(msg).unwrap();
            thread::sleep(dt);
        }
    });

    thread::spawn(move || {
        let signal = vec![1.1, 1.3, 1.9, 0.1, 0.0, 1.55];
        for e in signal {
            let msg = Message::new(1, e);
            tx1.send(msg).unwrap();
            thread::sleep(dt);
        }
    });

    for (n, received) in rx.iter().enumerate() {
        println!("Received signal #{n} from tx{}: {}", received.transmitter_id, received.signal);
    }

    // shared state concurrency using Mutexes
    let counter = Arc::new(Mutex::new(0)); // analagous to Rc::new(Refcell::new(0))
    let mut handles = vec![];
    
    for _ in 0..10 {
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

    println!("Counting in threads with Mutexes, n = {}", *counter.lock().unwrap());

}

struct Message {
    transmitter_id: u32,
    signal: f32,
}

impl Message {
    fn new(transmitter_id: u32, signal: f32) -> Message{
        Message{ transmitter_id, signal }
    }
}
