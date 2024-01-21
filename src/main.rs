use std::{env, process};

// use learn_rust::closure;
// use learn_rust::multi_thread;

use learn_rust::Config;

fn _mini_grep() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = learn_rust::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn main() {
    // closure::run();
    // multi_thread::run();
    _mini_grep()
}
