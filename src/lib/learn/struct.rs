struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn run() {
    let mut user = User {
        active: true,
        username: String::from("zack"),
        email: String::from("zack@brave.com"),
        sign_in_count: 1,
    };

    user.email = String::from("newemail@brave.com");

    let mut user2 = build_user(String::from("test"), String::from("username"));

    user2.sign_in_count += 1;

    println!("user2 count {}", user2.sign_in_count);

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    println!(
        "user3 {email} {username} {active}",
        email = user2.email,
        username = user3.username,
        active = user2.active,
    );

    println!("{}", test());
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
fn test() -> i32 {
    let color = Color(0, 0, 0);
    color.0
}
