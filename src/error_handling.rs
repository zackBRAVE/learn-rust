use std::{
    fs::File,
    io::{ErrorKind, Read},
};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "src/hello.txt";
    let greeting_file_result = File::open(file_path);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    println!("greeting file: {:?}", greeting_file);

    let greeting_file = File::open(file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_path)
                .unwrap_or_else(|error| panic!("Problem creating the file: {:?}", error))
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });

    println!("greeting file: {:?}", greeting_file);

    // let greeting_file = File::open(file_path).unwrap();
    // let greeting_file = File::open(file_path).expect("hello.txt should be included in the project");

    let username = read_username_from_file(file_path).unwrap();
    println!("username is {username}");

    let username = read_username_from_file_2(file_path).unwrap();
    println!("username is {username}");

    let username = read_username_from_file_3(file_path).unwrap();
    println!("username is {username}");

    let lines = last_char_of_first_line("first line,\nsecond line!");
    println!("{:?}", lines.unwrap());

    File::open("not existed file")?;

    Ok(())
}

fn read_username_from_file(file_path: &str) -> Result<String, std::io::Error> {
    let username_file_result = File::open(file_path);
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(size) => {
            println!("size: {:?}", size);
            Ok(username)
        }
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2(file_path: &str) -> Result<String, std::io::Error> {
    let mut username = String::new();
    File::open(file_path)?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_3(file_path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(file_path)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
