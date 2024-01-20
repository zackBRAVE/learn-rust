#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn run() {
    let string1 = String::from("abcd");
    let result;
    {
        // let string2 = String::from("xyz");
        let string2 = "xyz";
        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{i:?} {}", i.part);

    println!("{}", first_word(novel.as_str()));

    let announcement = String::from("this is an announcement");
    println!(
        "{}",
        longest_with_an_announcement(string1.as_str(), novel.as_str(), announcement)
    );
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first_word(sentence: &str) -> &str {
    let bytes = sentence.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &sentence[0..i];
        }
    }

    &sentence[..]
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
