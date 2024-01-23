#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    // Quit,
    // Move { x: i32, y: i32 },
    Write(String),
    // ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    // Alaska,
    // Kansas,
    // NewYork,
}

enum Coin {
    // Penny,
    // Nickle,
    Dime,
    Quarter(UsState),
}

fn run() {
    let v4_address = IpAddr::V4(127, 0, 0, 1);
    let v6_address = IpAddr::V6(String::from("::1"));

    println!("{:?}\n{:?}", v4_address, v6_address);

    let message = Message::Write(String::from("this is a message from me"));

    message.call();

    let some_number = Some(5);
    println!(
        "plus one of {:?} is {:?}",
        some_number,
        plus_one(some_number)
    );

    let some_char = Some('z');
    println!("some char is {:?}", some_char);

    let absent_number: Option<i32> = None;
    println!(
        "plus one of {:?} is {:?}",
        absent_number,
        plus_one(absent_number)
    );

    let none = plus_one(None);
    println!("plus one of None is {:?}", none);

    let coin = Coin::Dime;

    println!("{}", value_in_cents(&coin));
    println!("{}", value_in_cents(&Coin::Quarter(UsState::Alabama)));

    let dice = 9;
    dice_move(dice);
    dice_reroll(dice);
    dice_do_nothing(dice);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to {max}")
    }

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("{:?}", state),
        _ => count += 1,
    }
    println!("count is {count}");

    let coin2 = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin2 {
        println!("{:?}", state)
    } else {
        count += 1
    }
    println!("count is {count}");
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_num_spaces: u8) {}
fn reroll() {}

fn dice_move(dice: u8) {
    match dice {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn dice_reroll(dice: u8) {
    match dice {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

fn dice_do_nothing(dice: u8) {
    match dice {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        // Coin::Penny => 1,
        // Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state is {:?}", state);
            25
        }
    }
}
