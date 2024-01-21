pub mod basics;
pub mod iters;

fn _run_basics() {
    // basics::inventory::run();
    basics::ownership::run();
    basics::traits::run();
}

fn _run_iters() {
    iters::iter::run();
}

pub fn run() {
    // _run_basics();
    _run_iters();
}
