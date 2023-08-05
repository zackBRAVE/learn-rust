use std::collections::HashMap;

use crate::garden::vegetables;

pub mod garden;

fn main() {
    let plant = vegetables::Asparagus {};
    println!("I'm growing {:?}!", plant);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 200);
    scores.insert(String::from("Red"), 5);
}
