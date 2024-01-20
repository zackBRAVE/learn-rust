// multi producer, single consumer
use std::{
    sync::mpsc,
    thread::{self, sleep},
    time::Duration,
};

fn _receive() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

fn _try_receive() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        sleep(Duration::from_millis(1));
        tx.send(val).unwrap();
        // println!("{:?}", val);
    });

    let mut i = 0;
    loop {
        let received = rx.try_recv();
        match received {
            Ok(s) => {
                println!("Got: {s}");
                break;
            }
            Err(_) => {}
        }
        i += 1;
        println!("{i} cycle run");
    }
}

pub fn _multi_send() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            String::from("!"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            sleep(Duration::from_secs_f32(0.5))
        }
        sleep(Duration::from_secs(3));
    });

    let mut count = 0;
    for received in rx {
        println!("Got: {received}");
        println!("cycle {count}");
        count += 1;
    }
}

pub fn _multi_sender() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            String::from("!"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            sleep(Duration::from_secs_f32(0.3))
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            sleep(Duration::from_secs_f32(0.3))
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

pub fn run() {
    // _receive();
    // _try_receive();
    // _multi_send();
    _multi_sender();
}
