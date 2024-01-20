use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn run() -> JoinHandle<()> {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    })
}
