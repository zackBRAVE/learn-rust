use std::io::stdin;

fn run() {
    let mut number = String::new();

    stdin().read_line(&mut number).expect("Failed to read line");

    let number = number
        .trim()
        .parse()
        .expect("Your input should be a positive number");

    println!("fib of {number} is {}", fib(number));
}

fn fib(number: u128) -> u128 {
    match number {
        0 | 1 => 1,
        _ => fib(number - 1) + fib(number - 2),
    }
}
