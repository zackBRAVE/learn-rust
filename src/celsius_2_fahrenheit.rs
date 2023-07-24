use std::io::stdin;

fn run() {
    let mut celsius = String::new();

    stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");

    let celsius = celsius
        .trim()
        .parse()
        .expect("Your input should be a number");

    let fahrenheit = celsius_2_fahrenheit(celsius);

    println!("{celsius} celsius is equal to {fahrenheit} fahrenheit.");
}

fn celsius_2_fahrenheit(celsius: f64) -> f64 {
    celsius * 9. / 5. + 32.
}
