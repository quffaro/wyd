// TODO
mod library;

use std::thread;
use std::sync::mpsc::{self, TryRecvError};

struct Counter {
    counter: u64
}

impl Counter {
    fn tick(&mut self) {
        self.counter += 1;
    }
    fn count(&mut self, count: u64) {
        self.counter = count;
    }
    fn new() -> Counter {
        Counter {
            counter: 0
        }
    }
}

fn main() {
    library::viewer::viewer();
    // let mut ctr = Counter::new();
    // let mut counter = 0;
    // let (ty, ry) = mpsc::channel();
    // match Some(1) {
    //     Some(_) => {
    //         let (tx, rx) = mpsc::channel();
    //         thread::spawn(move || {
    //             library::request::request_string();
    //             tx.send(true).unwrap()
    //         });
    //         thread::spawn(move || loop {
    //             match rx.try_recv() {
    //                 Ok(_) | Err(TryRecvError::Disconnected) => {
    //                     // println!("{}", counter);
    //                     ty.send(true).unwrap();
    //                     break;
    //                 }
    //                 Err(TryRecvError::Empty) => {
    //                     counter += 1;
    //                     ()
    //                 }
    //             }
    //         });
    //     }
    //     None => {}
    // };

    // loop { 
    //     match ry.try_recv() {
    //         Ok(_) => break,
    //         Err(_) => ctr.count(counter),
    //     }
    // }
}
