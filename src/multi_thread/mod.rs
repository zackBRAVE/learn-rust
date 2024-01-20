pub mod basics;
pub mod comm;

fn _run_basics() {
    basics::spawn::run();
    let handle1 = basics::handle::run();

    let v = vec![1, 2, 3, 4];
    let handle2 = basics::moove::run(v);

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn _run_comm() {
    // comm::mpsc::run();
    comm::shared_state::run();
}

pub fn run() {
    // _run_basics();
    // _run_comm();
}
