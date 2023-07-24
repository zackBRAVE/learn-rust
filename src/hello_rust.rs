use ferris_says::say;
use std::io::{stdout, BufWriter};

fn output_test() {
    let std_out = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut buf_writer = BufWriter::new(std_out.lock());
    say(message.as_bytes(), width, &mut buf_writer).unwrap();
    println!("here's a sentence {}", message);

    let result = ((5 * 6 + 10) % 7) / 2;
    println!("result of {} = {}", "((5 * 6 + 10) % 7) / 2", result)
}
