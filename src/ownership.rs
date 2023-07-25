fn run() {
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}", s);

    let mut str = String::from("hello reference");
    change(&mut str);
    let len = calculate_length(&str);

    println!("The length of '{}' is {}", str, len);
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

fn change(str: &mut String) {
    str.push_str("test");
}
