use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let std_out = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut buf_writer = BufWriter::new(std_out.lock());
    say(message.as_bytes(), width, &mut buf_writer).unwrap()
}
