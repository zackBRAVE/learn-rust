use std::thread::{self, JoinHandle};

pub fn run(v: Vec<i32>) -> JoinHandle<()> {
    let v2 = vec![1, 2, 3, 4];
    thread::spawn(move || {
        println!("here's a vector: {:?}", v);
        for i in v2 {
            println!("from vector: {i}");
        }
    })
}
