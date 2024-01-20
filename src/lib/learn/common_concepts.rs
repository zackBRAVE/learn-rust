fn run() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;

        println!("inner x = {x}")
    }

    println!("outer x = {x}");

    print_labeled_measurement(get_squared(x), 'h');

    let condition = false;
    let y = if condition { 3 } else { 4 };
    println!("result of y is {y}");

    a_loop();
    embedded_loop();
    count_down();
    for_loop();
}

fn get_squared(value: i32) -> i32 {
    value.pow(2)
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("Measurement is {value}{unit_label}")
}

fn a_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break get_squared(counter * 3);
        }
    };

    println!("the result is {result}")
}

fn embedded_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn count_down() {
    let mut count_down_from = 3;
    while count_down_from > 0 {
        println!("{count_down_from}");
        count_down_from -= 1;
    }
}

fn for_loop() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("element is {element}");
    }

    for number in (1..11).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}
